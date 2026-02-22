/// Parameters for the driving estimation model.
#[derive(Debug, Clone)]
pub struct DrivingParams {
    /// Minimum speed in km/h (short distances).
    pub vmin: f64,
    /// Maximum speed in km/h (long distances).
    pub vmax: f64,
    /// Speed curve steepness factor.
    pub k: f64,
    /// Asymptotic distance factor for long distances (e.g. 1.25).
    pub f_long: f64,
    /// Peak distance factor for medium distances (e.g. 1.45).
    pub f_peak: f64,
    /// Distance in km at which the factor peaks (e.g. 5.0 km).
    pub d_peak_km: f64,
}

impl Default for DrivingParams {
    fn default() -> Self {
        Self {
            vmin: 20.0,
            vmax: 90.0,
            k: 0.015,
            f_long: 1.25,
            f_peak: 1.6,
            d_peak_km: 5.0,
        }
    }
}

/// Haversine distance between two points in meters.
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1 = lat1.to_radians();
    let lon1 = lon1.to_radians();
    let lat2 = lat2.to_radians();
    let lon2 = lon2.to_radians();

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    if dlat == 0.0 && dlon == 0.0 {
        return 0.0;
    }

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    c * 6_371_000.0
}

/// Distance-dependent detour factor.
/// Rises from ~1.0 at d=0, peaks at `d_peak_km`, then decays toward `f_long`.
/// Uses the curve: f_long + (f_peak - f_long) * (d / d_peak) * e^(1 - d / d_peak)
fn distance_factor(distance_km: f64, params: &DrivingParams) -> f64 {
    if distance_km <= 0.0 {
        return 1.0;
    }
    let x = distance_km / params.d_peak_km;
    params.f_long + (params.f_peak - params.f_long) * x * (1.0 - x).exp()
}

/// Estimated driving distance using a distance-dependent detour factor.
pub fn estimate_driving_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64, params: &DrivingParams) -> f64 {
    let straight = haversine_distance(lat1, lon1, lat2, lon2);
    straight * distance_factor(straight / 1000.0, params)
}

/// Estimated driving time in seconds from distance in meters.
/// Speed model: speed increases with distance (short trips are slower).
pub fn estimate_driving_time(distance_m: f64, params: &DrivingParams) -> f64 {
    if distance_m == 0.0 {
        return 0.0;
    }
    let distance_km = distance_m / 1000.0;
    let estimated_speed = params.vmin + (params.vmax - params.vmin) * (1.0 - (-params.k * distance_km).exp());
    (distance_km / estimated_speed) * 3600.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_params() -> DrivingParams {
        DrivingParams::default()
    }

    #[test]
    fn test_same_point() {
        let p = default_params();
        assert_eq!(haversine_distance(48.8566, 2.3522, 48.8566, 2.3522), 0.0);
        assert_eq!(estimate_driving_time(0.0, &p), 0.0);
    }

    #[test]
    fn test_paris_lyon() {
        let dist = haversine_distance(48.8566, 2.3522, 45.7640, 4.8357);
        assert!((dist - 392_000.0).abs() < 10_000.0, "got {dist}");
    }

    #[test]
    fn test_driving_distance_long_converges_to_f_long() {
        let p = default_params();
        // Paris–Lyon (~392 km): factor should be close to f_long (1.25)
        let raw = haversine_distance(48.8566, 2.3522, 45.7640, 4.8357);
        let driving = estimate_driving_distance(48.8566, 2.3522, 45.7640, 4.8357, &p);
        let factor = driving / raw;
        assert!((factor - p.f_long).abs() < 0.01, "long-distance factor {factor} should be ~{}", p.f_long);
    }

    #[test]
    fn test_driving_distance_short_lower_factor() {
        let p = default_params();
        // Very short distance (~1 km): factor should be lower than f_peak
        let raw = haversine_distance(48.8566, 2.3522, 48.8606, 2.3622);
        let driving = estimate_driving_distance(48.8566, 2.3522, 48.8606, 2.3622, &p);
        let factor = driving / raw;
        assert!(factor < p.f_peak, "short-distance factor {factor} should be < f_peak {}", p.f_peak);
        assert!(factor > 1.0, "factor should be > 1.0, got {factor}");
    }

    #[test]
    fn test_driving_time_reasonable() {
        let p = default_params();
        let dist = estimate_driving_distance(48.8566, 2.3522, 45.7640, 4.8357, &p);
        let time = estimate_driving_time(dist, &p);
        let hours = time / 3600.0;
        assert!(hours > 4.0 && hours < 7.0, "got {hours} hours");
    }
}
