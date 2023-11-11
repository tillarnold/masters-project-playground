pub struct Function<TI, TO> {
    pub function: Arc<dyn Fn(&TI) -> Fallible<TO> + Send + Sync>,
}


pub struct PrivacyMap<MI: Metric, MO: Measure>(
    pub Arc<dyn Fn(&MI::Distance) -> Fallible<MO::Distance> + Send + Sync>,
)

pub struct StabilityMap<MI: Metric, MO: Metric>(
    pub Arc<dyn Fn(&MI::Distance) -> Fallible<MO::Distance> + Send + Sync>,
);

pub struct Transformation<DI: Domain, DO: Domain, MI: Metric, MO: Metric> {
    pub input_domain: DI,
    pub output_domain: DO,
    pub function: Arc<dyn Fn(&DI::Carrier) -> Result<DO::Carrier> + Send + Sync>,
    pub input_metric: MI,
    pub output_metric: MO,
    pub stability_map: Arc<dyn Fn(&MI::Distance) -> Result<MO::Distance> + Send + Sync>,
}
