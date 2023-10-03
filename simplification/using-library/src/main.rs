use opendp::{
    core::Transformation,
    domains::{AtomDomain, VectorDomain},
    error::Fallible,
    measurements::then_laplace,
    metrics::SymmetricDistance,
    transformations::{make_clamp, then_sum},
};

fn clamp_transform() -> opendp::error::Fallible<
    Transformation<
        VectorDomain<AtomDomain<f64>>,
        VectorDomain<AtomDomain<f64>>,
        SymmetricDistance,
        SymmetricDistance,
    >,
> {
    make_clamp(
        VectorDomain::new(AtomDomain::default()),
        SymmetricDistance,
        (10.0, 20.0),
    )
}

fn example_client() -> Fallible<()> {
    let count = (clamp_transform()? >> then_sum() >> then_laplace(20.0))?;

    println!("privacy spend {:?}", count.map(&1));

    let res: f64 = count.invoke(&vec![1., 2., 3.0f64])?;

    println!("{:?}", res);

    Ok(())
}

fn main() {
    println!("{:?}", example_client().unwrap());
}
