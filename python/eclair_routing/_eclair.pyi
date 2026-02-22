import numpy as np
import numpy.typing as npt

class PyEclairEngine:
    """Eclair routing engine for fast approximate travel distance and time estimation.

    Uses Haversine distance with a road factor, a speed model that varies with
    trip length, and optional H3 hexagonal grid speed factors to adjust for
    population density.

    Args:
        config_path: Path to a CSV or Parquet file containing H3 speed factors.
            The file must have columns ``h3_index`` (hex string, resolution 7)
            and ``factor`` (float between 0 and 1). If None, all hexagons
            default to factor 1.0 (no adjustment).
        vmin: Minimum driving speed in km/h, used for short trips. Default: 20.0.
        vmax: Maximum driving speed in km/h, approached on long trips. Default: 90.0.
        k: Speed curve steepness — controls how quickly speed ramps from vmin
            to vmax as distance increases. Default: 0.015.
        f_long: Asymptotic distance factor for long distances. Default: 1.25.
        f_peak: Peak distance factor for medium distances. Default: 1.45.
        d_peak_km: Distance in km at which the factor peaks. Default: 5.0.

    Example::

        from eclair_routing import EclairEngine

        engine = EclairEngine()
        dist, time = engine.estimate_travel(48.8566, 2.3522, 45.7640, 4.8357)
        print(f"{dist/1000:.0f} km, {time/3600:.1f} hours")
    """

    def __init__(
        self,
        config_path: str | None = None,
        vmin: float = 23,
        vmax: float = 98,
        k: float = 0.017,
        f_long: float = 1.27,
        f_peak: float = 1.58,
        d_peak_km: float = 3.60,
    ) -> None: ...
    def estimate_travel(
        self,
        lat1: float,
        lon1: float,
        lat2: float,
        lon2: float,
    ) -> tuple[float, float]:
        """Estimate travel distance and time between two points.

        Args:
            lat1: Latitude of the origin point (decimal degrees).
            lon1: Longitude of the origin point (decimal degrees).
            lat2: Latitude of the destination point (decimal degrees).
            lon2: Longitude of the destination point (decimal degrees).

        Returns:
            A tuple of ``(distance_meters, time_seconds)``:
                - ``distance_meters``: Estimated driving distance in meters.
                - ``time_seconds``: Estimated driving time in seconds,
                  adjusted by H3 speed factors if a config was loaded.
        """
        ...

    def compute_matrix(
        self,
        lats: npt.NDArray[np.float64],
        lons: npt.NDArray[np.float64],
    ) -> tuple[npt.NDArray[np.float64], npt.NDArray[np.float64]]:
        """Compute square distance and time matrices for a set of points.

        The matrices are symmetric with zero diagonal.

        Args:
            lats: 1D numpy array of latitudes (decimal degrees).
            lons: 1D numpy array of longitudes (decimal degrees).

        Returns:
            A tuple of ``(distance_matrix, time_matrix)``:
                - ``distance_matrix``: 2D numpy array of shape (n, n), distances in meters.
                - ``time_matrix``: 2D numpy array of shape (n, n), times in seconds.
        """
        ...

    def compute_non_square_matrix(
        self,
        origin_lats: npt.NDArray[np.float64],
        origin_lons: npt.NDArray[np.float64],
        dest_lats: npt.NDArray[np.float64],
        dest_lons: npt.NDArray[np.float64],
    ) -> tuple[npt.NDArray[np.float64], npt.NDArray[np.float64]]:
        """Compute non-square distance and time matrices between origins and destinations.

        Args:
            origin_lats: 1D numpy array of origin latitudes (decimal degrees).
            origin_lons: 1D numpy array of origin longitudes (decimal degrees).
            dest_lats: 1D numpy array of destination latitudes (decimal degrees).
            dest_lons: 1D numpy array of destination longitudes (decimal degrees).

        Returns:
            A tuple of ``(distance_matrix, time_matrix)``:
                - ``distance_matrix``: 2D numpy array of shape (n_origins, n_dests), distances in meters.
                - ``time_matrix``: 2D numpy array of shape (n_origins, n_dests), times in seconds.
        """
        ...
