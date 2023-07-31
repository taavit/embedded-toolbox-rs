pub trait HumidityReading {
    fn get_humidity(&self) -> f64;
}
