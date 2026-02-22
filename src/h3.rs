use h3o::{CellIndex, LatLng, Resolution};
use std::collections::HashMap;

const H3_RESOLUTION: Resolution = Resolution::Seven;
const MIN_SPEED_FACTOR: f64 = 0.05;

/// Convert lat/lon to H3 cell index at resolution 7.
pub fn lat_lon_to_h3(lat: f64, lon: f64) -> Result<CellIndex, String> {
    let ll = LatLng::new(lat, lon).map_err(|e| format!("invalid coordinates: {e}"))?;
    Ok(ll.to_cell(H3_RESOLUTION))
}

/// Get the grid path (line of cells) between two H3 cells.
pub fn grid_path(origin: CellIndex, dest: CellIndex) -> Result<Vec<CellIndex>, String> {
    let iter = origin
        .grid_path_cells(dest)
        .map_err(|e| format!("grid path error: {e}"))?;

    iter.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("grid path cell error: {e}"))
}

/// Compute the harmonic mean of speed factors along an H3 path.
/// Missing hexes default to 1.0 (normal speed).
/// Each factor is clamped to [MIN_SPEED_FACTOR, 1.0].
pub fn compute_path_speed_factor(
    path: &[CellIndex],
    factors: &HashMap<CellIndex, f64>,
) -> f64 {
    if path.is_empty() {
        return 1.0;
    }

    let n = path.len() as f64;
    let sum_reciprocals: f64 = path
        .iter()
        .map(|cell| {
            let f = factors.get(cell).copied().unwrap_or(1.0);
            1.0 / f.clamp(MIN_SPEED_FACTOR, 1.0)
        })
        .sum();

    // Harmonic mean = n / Σ(1/factor_i)
    n / sum_reciprocals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lat_lon_to_h3() {
        let cell = lat_lon_to_h3(48.8566, 2.3522).unwrap();
        assert_eq!(cell.resolution(), Resolution::Seven);
    }

    #[test]
    fn test_grid_path_same_cell() {
        let cell = lat_lon_to_h3(48.8566, 2.3522).unwrap();
        let path = grid_path(cell, cell).unwrap();
        assert_eq!(path.len(), 1);
    }

    #[test]
    fn test_grid_path_nearby() {
        let a = lat_lon_to_h3(48.8566, 2.3522).unwrap(); // Paris
        let b = lat_lon_to_h3(48.8600, 2.3600).unwrap(); // nearby
        let path = grid_path(a, b).unwrap();
        assert!(path.len() >= 1);
    }

    #[test]
    fn test_harmonic_mean_all_ones() {
        let path = vec![lat_lon_to_h3(48.8566, 2.3522).unwrap()];
        let factors = HashMap::new();
        let result = compute_path_speed_factor(&path, &factors);
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_harmonic_mean_half_speed() {
        let cell = lat_lon_to_h3(48.8566, 2.3522).unwrap();
        let path = vec![cell];
        let mut factors = HashMap::new();
        factors.insert(cell, 0.5);
        let result = compute_path_speed_factor(&path, &factors);
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_harmonic_mean_mixed() {
        // Use distant points to ensure different H3 cells
        let a = lat_lon_to_h3(48.8566, 2.3522).unwrap(); // Paris
        let b = lat_lon_to_h3(45.7640, 4.8357).unwrap(); // Lyon
        assert_ne!(a, b);
        let path = vec![a, b];
        let mut factors = HashMap::new();
        factors.insert(a, 1.0);
        factors.insert(b, 0.5);
        let result = compute_path_speed_factor(&path, &factors);
        // Harmonic mean of [1.0, 0.5] = 2 / (1/1 + 1/0.5) = 2/3 ≈ 0.6667
        assert!((result - 2.0 / 3.0).abs() < 1e-10);
    }
}
