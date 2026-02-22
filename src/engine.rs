use crate::h3::{compute_path_speed_factor, grid_path, lat_lon_to_h3};
use crate::haversine::{estimate_driving_distance, estimate_driving_time, DrivingParams};
use h3o::CellIndex;
use polars::prelude::*;
use std::collections::HashMap;
use std::path::Path;

pub struct EclairEngine {
    factors: HashMap<CellIndex, f64>,
    pub params: DrivingParams,
}

impl EclairEngine {
    /// Create a new engine with optional CSV config and optional driving parameters.
    pub fn new(csv_path: Option<&str>, params: Option<DrivingParams>) -> Result<Self, String> {
        let factors = match csv_path {
            Some(path) => load_factors(path)?,
            None => HashMap::new(),
        };
        Ok(Self {
            factors,
            params: params.unwrap_or_default(),
        })
    }

    /// Estimate travel distance (meters) and time (seconds) between two points.
    pub fn estimate_travel(
        &self,
        lat1: f64,
        lon1: f64,
        lat2: f64,
        lon2: f64,
    ) -> Result<(f64, f64), String> {
        let distance = estimate_driving_distance(lat1, lon1, lat2, lon2, &self.params);
        let base_time = estimate_driving_time(distance, &self.params);

        let speed_factor = self.compute_speed_factor(lat1, lon1, lat2, lon2)?;
        let adjusted_time = base_time / speed_factor;

        Ok((distance, adjusted_time))
    }

    /// Compute square distance and time matrices for a set of points.
    pub fn compute_matrix(
        &self,
        lats: &[f64],
        lons: &[f64],
    ) -> Result<(Vec<f64>, Vec<f64>), String> {
        let n = lats.len();
        if n != lons.len() {
            return Err("lats and lons must have the same length".into());
        }

        let mut dist_matrix = vec![0.0f64; n * n];
        let mut time_matrix = vec![0.0f64; n * n];

        for i in 0..n {
            for j in (i + 1)..n {
                let (dist, time) = self.estimate_travel(lats[i], lons[i], lats[j], lons[j])?;
                dist_matrix[i * n + j] = dist;
                dist_matrix[j * n + i] = dist;
                time_matrix[i * n + j] = time;
                time_matrix[j * n + i] = time;
            }
        }

        Ok((dist_matrix, time_matrix))
    }

    /// Compute non-square distance and time matrices between origins and destinations.
    pub fn compute_non_square_matrix(
        &self,
        origin_lats: &[f64],
        origin_lons: &[f64],
        dest_lats: &[f64],
        dest_lons: &[f64],
    ) -> Result<(Vec<f64>, Vec<f64>), String> {
        let n_origins = origin_lats.len();
        let n_dests = dest_lats.len();

        if n_origins != origin_lons.len() || n_dests != dest_lons.len() {
            return Err("lats and lons must have the same length".into());
        }

        let mut dist_matrix = vec![0.0f64; n_origins * n_dests];
        let mut time_matrix = vec![0.0f64; n_origins * n_dests];

        for i in 0..n_origins {
            for j in 0..n_dests {
                let (dist, time) =
                    self.estimate_travel(origin_lats[i], origin_lons[i], dest_lats[j], dest_lons[j])?;
                dist_matrix[i * n_dests + j] = dist;
                time_matrix[i * n_dests + j] = time;
            }
        }

        Ok((dist_matrix, time_matrix))
    }

    fn compute_speed_factor(
        &self,
        lat1: f64,
        lon1: f64,
        lat2: f64,
        lon2: f64,
    ) -> Result<f64, String> {
        if self.factors.is_empty() {
            return Ok(1.0);
        }

        let origin = lat_lon_to_h3(lat1, lon1)?;
        let dest = lat_lon_to_h3(lat2, lon2)?;

        let path = grid_path(origin, dest)?;
        Ok(compute_path_speed_factor(&path, &self.factors))
    }
}

fn load_factors(path: &str) -> Result<HashMap<CellIndex, f64>, String> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(format!("config file not found: {path}"));
    }

    let df = match p.extension().and_then(|e| e.to_str()) {
        Some("parquet") => LazyFrame::scan_parquet(path.into(), Default::default())
            .map_err(|e| format!("parquet read error: {e}"))?
            .collect()
            .map_err(|e| format!("parquet collect error: {e}"))?,
        _ => CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(path.into()))
            .map_err(|e| format!("CSV read error: {e}"))?
            .finish()
            .map_err(|e| format!("CSV parse error: {e}"))?,
    };

    let h3_col = df
        .column("h3_index")
        .map_err(|e| format!("missing h3_index column: {e}"))?
        .str()
        .map_err(|e| format!("h3_index column must be string: {e}"))?;
    let factor_col = df
        .column("factor")
        .map_err(|e| format!("missing factor column: {e}"))?
        .f64()
        .map_err(|e| format!("factor column must be float: {e}"))?;

    let mut factors = HashMap::new();
    for (h3_opt, factor_opt) in h3_col.into_iter().zip(factor_col.into_iter()) {
        if let (Some(h3_str), Some(factor)) = (h3_opt, factor_opt) {
            let cell: CellIndex = h3_str
                .trim()
                .parse()
                .map_err(|e| format!("invalid H3 index '{h3_str}': {e}"))?;
            factors.insert(cell, factor);
        }
    }

    Ok(factors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_no_config() {
        let engine = EclairEngine::new(None, None).unwrap();
        let (dist, time) = engine
            .estimate_travel(48.8566, 2.3522, 45.7640, 4.8357)
            .unwrap();
        assert!(dist > 400_000.0 && dist < 500_000.0);
        assert!(time > 4.0 * 3600.0 && time < 7.0 * 3600.0);
    }

    #[test]
    fn test_engine_same_point() {
        let engine = EclairEngine::new(None, None).unwrap();
        let (dist, time) = engine
            .estimate_travel(48.8566, 2.3522, 48.8566, 2.3522)
            .unwrap();
        assert_eq!(dist, 0.0);
        assert_eq!(time, 0.0);
    }

    #[test]
    fn test_custom_params() {
        let slow = DrivingParams {
            vmax: 50.0,
            ..Default::default()
        };
        let engine_slow = EclairEngine::new(None, Some(slow)).unwrap();
        let engine_default = EclairEngine::new(None, None).unwrap();

        let (_, time_slow) = engine_slow.estimate_travel(48.8566, 2.3522, 45.7640, 4.8357).unwrap();
        let (_, time_default) = engine_default.estimate_travel(48.8566, 2.3522, 45.7640, 4.8357).unwrap();

        assert!(time_slow > time_default, "slower vmax should give longer time");
    }
}
