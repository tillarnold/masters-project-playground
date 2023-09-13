use std::{
    collections::btree_set::SymmetricDifference, env, fs::File, io::BufReader, path::PathBuf,
    str::FromStr,
};

use opendp::{
    core::{Metric, MetricSpace, Transformation},
    domains::{AtomDomain, VectorDomain},
    error::Fallible,
    measurements::then_laplace,
    metrics::{InsertDeleteDistance, SymmetricDistance},
    traits::{InfCast, TotalOrd},
    transformations::{
        self, make_count, make_create_dataframe, make_select_column, then_cast_default, then_clamp,
        then_count, then_mean, then_resize,
    },
};

pub fn example() -> Fallible<()> {
    let x = <i32 as InfCast<i32>>::inf_cast(3i32);

    use opendp::combinators::{make_chain_mt, make_chain_tt};
    use opendp::measurements::then_base_laplace;
    use opendp::transformations::{
        make_cast_default, make_mean, make_split_lines, then_cast_default, then_clamp, then_count,
        then_sum,
    };

    let data = "56\n15\n97\n56\n6\n17\n2\n19\n16\n50".to_owned();
    let bounds = (0.0, 100.0);
    let epsilon = 1.0;
    // remove some epsilon to account for floating-point error
    let sigma = (bounds.1 - bounds.0) / (epsilon - 0.0001);

    // Construct a Transformation to parse a csv string.
    let split_lines = make_split_lines()?;

    // The next transformation wants to conform with the output domain and metric from `split_lines`.
    let cast = make_cast_default::<_, String, f64>(
        split_lines.output_domain.clone(),
        split_lines.output_metric.clone(),
    )?;

    // Since the domain and metric conforms, these two transformations may be chained.
    let load_numbers = make_chain_tt(&cast, &split_lines)?;

    // You can use the more convenient `>>` notation to chain instead.
    // When you use the `then_` version of the constructor,
    //     the `>>` operator will automatically fill the input domain and metric from the previous transformation.
    let load_and_clamp = load_numbers >> then_clamp(bounds);

    // After chaining, the resulting transformation is wrapped in a `Result`.
    let load_and_sum = (load_and_clamp >> then_sum())?;

    // Construct a Measurement to calculate a noisy sum4.
    let noisy_sum = load_and_sum >> then_base_laplace(sigma, None);

    // The same measurement, written more succinctly:
    let noisy_sum = (make_split_lines()?
        >> then_cast_default()
        >> then_clamp(bounds)
        >> then_sum()
        >> then_base_laplace(sigma, None))?;

    // Check that the pipeline is (1, 1.0)-close
    assert!(noisy_sum.check(&1, &epsilon)?);

    // Make a 1.0-epsilon-DP release
    let release = noisy_sum.invoke(&data)?;
    println!("release = {}", release);
    Ok(())
}

pub fn example2() -> Fallible<()> {
    let x = <i32 as InfCast<i32>>::inf_cast(3i32);

    use opendp::combinators::{make_chain_mt, make_chain_tt};
    use opendp::measurements::then_base_laplace;
    use opendp::transformations::{
        make_cast_default, make_mean, make_split_lines, then_cast_default, then_clamp, then_count,
        then_sum,
    };

    let raw_data = [56, 15, 97, 56, 6, 17, 2, 19, 16, 50];

    let data: String = raw_data
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    println!("non private sum {}", raw_data.iter().sum::<i32>());

    let bounds = (0.0, 100.0);
    let epsilon: f64 = 1.0;
    // remove some epsilon to account for floating-point error
    let sigma = (bounds.1 - bounds.0) / (epsilon - 0.0001);

    // The same measurement, written more succinctly:
    let noisy_sum = (make_split_lines()?
        >> then_cast_default()
        >> then_clamp(bounds)
        >> then_sum()
        >> then_base_laplace(sigma, None))?;

    // Check that the pipeline is (1, 1.0)-close

    assert!(noisy_sum.check(&1, &epsilon)?);

    let mut m = vec![];

    // Make a 1.0-epsilon-DP release
    for _ in 0..4 {
        let release = noisy_sum.invoke(&data)?;
        //println!("release = {}", release);
        m.push(release);
    }

    let avg: f64 = m.iter().sum::<f64>() / m.len() as f64;
    println!("non private sum {}", raw_data.iter().sum::<i32>());
    println!("avg {}", avg);

    Ok(())
}

pub fn data_dir() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut par = PathBuf::from(manifest_dir);
    par.pop();
    par.push("example-data");

    par
}

pub fn csv_file() -> PathBuf {
    let mut dir = data_dir();
    dir.push("teacher_survey.csv");
    dir
}

fn csv_headers() -> Vec<&'static str> {
    return vec![
        "name",
        "sex",
        "age",
        "maritalStatus",
        "hasChildren",
        "highestEducationLevel",
        "sourceOfStress",
        "smoker",
        "optimism",
        "lifeSatisfaction",
        "selfEsteem",
    ];
}

fn read_data() -> Vec<Vec<String>> {
    const LIMIT: usize = 100;

    let file = File::open(csv_file()).expect("could not open csv file");
    let reader = BufReader::new(file);

    let mut rdr = csv::Reader::from_reader(reader);

    rdr.records()
        .take(LIMIT)
        .map(|rec| {
            rec.unwrap()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn example3() -> Fallible<()> {
    let count = (make_create_dataframe(csv_headers())?
        >> make_select_column::<&str, String>("age")?
        >> then_count()
        >> then_laplace(1.0))?;

    println!("privacy spend {:?}", count.map(&1));

    let res: i64 = count.invoke(&read_data())?;

    println!("{:?}", res);

    Ok(())
}

fn age_trans() -> Fallible<
    Transformation<
        VectorDomain<VectorDomain<AtomDomain<String>>>,
        VectorDomain<AtomDomain<String>>,
        SymmetricDistance,
        SymmetricDistance,
    >,
> {
    make_create_dataframe(csv_headers())? >> make_select_column::<&str, String>("age")?
}

use opendp::transformations::IsMetricOrdered;

fn example_mean_age() -> Fallible<()> {
    let raw_data = read_data();

    let count_meas = (age_trans()? >> then_count::<_, usize>() >> then_laplace(2.0))?;

    // make_mean_age(scale):
    // return (
    //     age_trans >>
    //     then_cast_default(float) >>
    //     then_clamp(bounds=age_bounds) >>
    //     then_resize(size=dp_count, constant=42.) >>
    //     then_mean() >>
    //     then_base_laplace(scale=scale)
    // )

    let dp_count = count_meas.invoke(&raw_data)?;

    let make_mean_age = (age_trans()?
        >> then_cast_default()
        >> then_clamp((5.0,120.0))
        >> then_resize::<_,SymmetricDistance,InsertDeleteDistance>(dp_count, 42.0) // what is 42 here?
        >> then_mean()
        >> then_laplace(1.0))?;

    println!("privacy spend mean_age{:?}", make_mean_age.map(&1));

    let res: f64 = make_mean_age.invoke(&read_data())?;

    println!("mean_age {:?}", res);

    Ok(())
}

fn example4() -> Fallible<()> {
    let count = (make_count(
        VectorDomain::new(AtomDomain::new_nullable()),
        SymmetricDistance,
    ) >> then_laplace(1.0))?;

    println!("privacy spend {:?}", count.map(&1));

    let res: i64 = count.invoke(&vec![1., 2., 3.0f32])?;

    println!("{:?}", res);

    Ok(())
}

fn main() {
    println!("{:?}", example3().unwrap());
}
