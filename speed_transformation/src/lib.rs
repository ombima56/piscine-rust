
pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h * 1000.0 / 3600.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_km_per_hour_to_meters_per_second() {
        assert_eq!(km_per_hour_to_meters_per_second(100.0), 27.77777777777778);
    }
}
