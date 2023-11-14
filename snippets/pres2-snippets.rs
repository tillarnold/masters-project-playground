struct Transformation<DI: Domain, DO: Domain, MI: Metric, MO: Metric> {
    input_domain: DI,
    output_domain: DO,
    function: Arc<dyn Fn(&DI::Carrier) -> Result<DO::Carrier> + Send + Sync>,
    input_metric: MI,
    output_metric: MO,
    stability_map: Arc<dyn Fn(&MI::Distance) -> Result<MO::Distance> + Send + Sync>,
}



fn make_clamp</*  */>(
    input_domain: //...
    input_metric: //...
    bounds: //...
) -> Result<Transformation</* ... */>> {
   // ...
    make_row_by_row_fallible(
        input_domain,
        input_metric,
        output_row_domain,
        move |arg: &TA| arg.total_clamp(bounds),
    )
}


fn make_row_by_row_fallible</* ... */>(
    //...
    row_function: //...
) -> Result<Transformation</* ... */>> {
    let output_domain = input_domain.translate(output_row_domain);
    Transformation::new(
        Arc::new(move |arg: &DI::Carrier| DI::apply_rows(arg, &row_function)),
        StabilityMap::new_from_constant(1),
        // ...
    )
}

fn apply_rows(
    value: &Self::Carrier,
    row_function: &impl Fn(&DIA::Carrier) -> Result<DOA::Carrier>,
) -> Result<Vec<DOA::Carrier>> {
    value.iter().map(row_function).collect()
}



// other


impl Vector {
    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    fn get(self, idx: i32) -> i32 { unimplemented!() }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    #[ensures(result.0 == self.get(idx))]
    #[ensures(result.1 === self)]
    fn impure_get(self, idx: i32) -> (i32, Self) { unimplemented!() }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    #[ensures(self.len == result.len)]
    #[ensures(result.get(idx) == value)]
    #[ensures(forall(|i : i32| (i >= 0  && i < self.len && !(i == idx))
        ==> result.get(i) == self.get(i)))]
    fn set(self, idx: i32, value: i32) -> Self { unimplemented!() }
}


#[derive(Clone, Copy)]
struct Bounds {
    upper: i32,
    lower: i32,
}

#[derive(Clone, Copy)]
struct ClampTransform {
    bounds: Bounds,
}



#[requires(data.len >= 0)]
#[requires(transform.bounds.lower < transform.bounds.upper)]
#[ensures(result.0.len === data.len)]
#[ensures(result.1 === transform)]
#[ensures(forall(|ip: i32| (0<= ip && ip < data.len) 
    ==> result.0.get(ip) == transform.do_transform(data.get(ip))))]
fn apply_row_by_row(transform: ClampTransform, data: Vector)
-> (Vector, ClampTransform) {
    if data.len <= 0 { return (data, transform); }

    let l = data.len;
    (apply_row_by_row_rec(transform, data, l - 1), transform)
}

impl ClampTransform {
    #[ensures(result.bounds === bounds)]
    fn make_clamp(bounds: Bounds) -> Self { Self { bounds } }

    #[pure]
    #[requires(self.bounds.lower < self.bounds.upper)]
    fn do_transform(self, data: i32) -> i32 {
        max(self.bounds.lower, min(self.bounds.upper, data))
    }

    #[requires(self.bounds.lower < self.bounds.upper)]
    #[ensures(result.0 == self.do_transform(data))]
    #[ensures(result.1 === self)]
    fn do_transform_impure(self, data: i32) -> (i32, Self) {
        if data < self.bounds.lower {
            (self.bounds.lower, self)
        } else if data > self.bounds.upper {
            (self.bounds.upper, self)
        } else {
            (data, self)
        }
    }
}





