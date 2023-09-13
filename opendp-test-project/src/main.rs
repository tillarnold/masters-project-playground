use opendp::{
    core::Transformation,
    domains::{AtomDomain, VectorDomain},
    error::Fallible,
    measurements::{then_base_laplace, then_laplace},
    metrics::{InsertDeleteDistance, SymmetricDistance},
    transformations::{
        make_count, make_create_dataframe, make_select_column, make_split_lines, then_cast_default,
        then_clamp, then_count, then_mean, then_resize, then_sum,
    },
};

mod data;

use crate::data::{read_teacher_data, teacher_data_csv_headers};

/// Example that shows how doing multiple releases compromizes privacy
pub fn multiple_releases() -> Fallible<()> {
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

    // Make a 1.0-epsilon-DP release but do it 100 times
    for _ in 0..10000 {
        let release = noisy_sum.invoke(&data)?;
        //println!("release = {}", release);
        m.push(release);
    }

    let avg: f64 = m.iter().sum::<f64>() / m.len() as f64;
    println!("non private sum {}", raw_data.iter().sum::<i32>());
    println!("avg of the private sums{}", avg);

    Ok(())
}

fn dataframe_example() -> Fallible<()> {
    let count = (make_create_dataframe(teacher_data_csv_headers())?
        >> make_select_column::<&str, String>("age")?
        >> then_count()
        >> then_laplace(1.0))?;

    println!("privacy spend {:?}", count.map(&1));

    let res: i64 = count.invoke(&read_teacher_data())?;

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
    make_create_dataframe(teacher_data_csv_headers())? >> make_select_column::<&str, String>("age")?
}

fn example_mean_age() -> Fallible<()> {
    let raw_data = read_teacher_data();

    let count_meas = (age_trans()? >> then_count::<_, usize>() >> then_laplace(2.0))?;

    let dp_count = count_meas.invoke(&raw_data)?;

    let make_mean_age = (age_trans()?
        >> then_cast_default()
        >> then_clamp((5.0,120.0))
        >> then_resize::<_,SymmetricDistance,InsertDeleteDistance>(dp_count, 42.0) // what is 42 here?
        >> then_mean()
        >> then_laplace(1.0))?;

    println!("privacy spend mean_age{:?}", make_mean_age.map(&1));

    let res: f64 = make_mean_age.invoke(&read_teacher_data())?;

    println!("mean_age {:?}", res);

    Ok(())
}

fn example_vec() -> Fallible<()> {
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
    println!("{:?}", multiple_releases().unwrap());
}
