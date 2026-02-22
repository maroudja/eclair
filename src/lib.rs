mod engine;
mod h3;
mod haversine;

use engine::EclairEngine;
use haversine::DrivingParams;
use numpy::ndarray::Array2;
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray1};
use pyo3::prelude::*;

#[pyclass]
struct PyEclairEngine {
    inner: EclairEngine,
}

#[pymethods]
impl PyEclairEngine {
    #[new]
    #[pyo3(signature = (config_path=None, vmin=20.0, vmax=90.0, k=0.015, f_long=1.25, f_peak=1.45, d_peak_km=5.0))]
    fn new(
        config_path: Option<&str>,
        vmin: f64,
        vmax: f64,
        k: f64,
        f_long: f64,
        f_peak: f64,
        d_peak_km: f64,
    ) -> PyResult<Self> {
        let params = DrivingParams {
            vmin,
            vmax,
            k,
            f_long,
            f_peak,
            d_peak_km,
        };
        let inner = EclairEngine::new(config_path, Some(params))
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(Self { inner })
    }

    /// Update driving parameters without reloading the config file.
    fn set_params(
        &mut self,
        vmin: f64,
        vmax: f64,
        k: f64,
        f_long: f64,
        f_peak: f64,
        d_peak_km: f64,
    ) {
        self.inner.params = DrivingParams {
            vmin,
            vmax,
            k,
            f_long,
            f_peak,
            d_peak_km,
        };
    }

    /// Estimate travel distance (meters) and time (seconds) between two points.
    fn estimate_travel(
        &self,
        lat1: f64,
        lon1: f64,
        lat2: f64,
        lon2: f64,
    ) -> PyResult<(f64, f64)> {
        self.inner
            .estimate_travel(lat1, lon1, lat2, lon2)
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    /// Compute square distance and time matrices.
    /// Returns (distance_matrix, time_matrix) as numpy 2D arrays.
    fn compute_matrix<'py>(
        &self,
        py: Python<'py>,
        lats: PyReadonlyArray1<'py, f64>,
        lons: PyReadonlyArray1<'py, f64>,
    ) -> PyResult<(Bound<'py, PyArray2<f64>>, Bound<'py, PyArray2<f64>>)> {
        let lats = lats.as_slice()?;
        let lons = lons.as_slice()?;
        let n = lats.len();

        let (dist_flat, time_flat) = self
            .inner
            .compute_matrix(lats, lons)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        let dist_array = Array2::from_shape_vec((n, n), dist_flat)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let time_array = Array2::from_shape_vec((n, n), time_flat)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok((dist_array.into_pyarray(py), time_array.into_pyarray(py)))
    }

    /// Compute non-square distance and time matrices between origins and destinations.
    /// Returns (distance_matrix, time_matrix) as numpy 2D arrays.
    fn compute_non_square_matrix<'py>(
        &self,
        py: Python<'py>,
        origin_lats: PyReadonlyArray1<'py, f64>,
        origin_lons: PyReadonlyArray1<'py, f64>,
        dest_lats: PyReadonlyArray1<'py, f64>,
        dest_lons: PyReadonlyArray1<'py, f64>,
    ) -> PyResult<(Bound<'py, PyArray2<f64>>, Bound<'py, PyArray2<f64>>)> {
        let origin_lats = origin_lats.as_slice()?;
        let origin_lons = origin_lons.as_slice()?;
        let dest_lats = dest_lats.as_slice()?;
        let dest_lons = dest_lons.as_slice()?;

        let n_origins = origin_lats.len();
        let n_dests = dest_lats.len();

        let (dist_flat, time_flat) = self
            .inner
            .compute_non_square_matrix(origin_lats, origin_lons, dest_lats, dest_lons)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        let dist_array = Array2::from_shape_vec((n_origins, n_dests), dist_flat)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let time_array = Array2::from_shape_vec((n_origins, n_dests), time_flat)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        Ok((dist_array.into_pyarray(py), time_array.into_pyarray(py)))
    }
}

#[pymodule]
fn _eclair(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEclairEngine>()?;
    Ok(())
}
