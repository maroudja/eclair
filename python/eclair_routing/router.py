from __future__ import annotations

from pathlib import Path

import numpy as np
import numpy.typing as npt

from eclair_routing._eclair import PyEclairEngine
from eclair_routing.models import Point, TravelResult
from eclair_routing.modes import MODES

_DEFAULT_CONFIG = str(Path(__file__).parent / "data" / "default_factors.parquet")
_SENTINEL = object()


class Router:
    """High-level routing interface with named transport modes.

    Args:
        mode: Transport mode — one of ``'car'``, ``'truck'``, ``'bike'``,
            ``'scooter'``, or ``'foot'``.
        config_path: Path to a CSV or Parquet file containing H3 speed factors.
            Defaults to the bundled ``default_factors.parquet``.
            Pass ``None`` explicitly to disable config loading.

    Example::

        from eclair_routing import Router, Point

        router = Router("car")                        # uses default config
        router = Router("car", config_path=None)      # no config file
        router = Router("car", config_path="my.parquet")  # custom config
    """

    def __init__(self, mode: str = "car", *, config_path: str | None = _SENTINEL) -> None:
        if mode not in MODES:
            available = ", ".join(sorted(MODES))
            raise ValueError(f"Unknown mode {mode!r}. Choose from: {available}")

        self._mode = mode
        cfg = MODES[mode]
        resolved_config = _DEFAULT_CONFIG if config_path is _SENTINEL else config_path
        self._engine = PyEclairEngine(
            config_path=resolved_config,
            vmin=cfg.vmin,
            vmax=cfg.vmax,
            k=cfg.k,
            f_long=cfg.f_long,
            f_peak=cfg.f_peak,
            d_peak_km=cfg.d_peak_km,
        )

    @property
    def mode(self) -> str:
        return self._mode

    def estimate(self, origin: Point, dest: Point) -> TravelResult:
        """Estimate travel distance and time between two points.

        Args:
            origin: Origin point.
            dest: Destination point.

        Returns:
            A :class:`TravelResult` with distance and time.
        """
        dist, time = self._engine.estimate_travel(
            origin.lat, origin.lon, dest.lat, dest.lon
        )
        return TravelResult(distance_meters=dist, time_seconds=time)

    def matrix(
        self, points: list[Point]
    ) -> tuple[npt.NDArray[np.float64], npt.NDArray[np.float64]]:
        """Compute a square distance/time matrix for a set of points.

        Args:
            points: List of points.

        Returns:
            A tuple of ``(distance_matrix, time_matrix)`` as 2D numpy arrays
            of shape ``(n, n)``.
        """
        lats = np.array([p.lat for p in points], dtype=np.float64)
        lons = np.array([p.lon for p in points], dtype=np.float64)
        return self._engine.compute_matrix(lats, lons)

    def matrix_od(
        self, origins: list[Point], destinations: list[Point]
    ) -> tuple[npt.NDArray[np.float64], npt.NDArray[np.float64]]:
        """Compute a non-square distance/time matrix (origins × destinations).

        Args:
            origins: List of origin points.
            destinations: List of destination points.

        Returns:
            A tuple of ``(distance_matrix, time_matrix)`` as 2D numpy arrays
            of shape ``(n_origins, n_destinations)``.
        """
        o_lats = np.array([p.lat for p in origins], dtype=np.float64)
        o_lons = np.array([p.lon for p in origins], dtype=np.float64)
        d_lats = np.array([p.lat for p in destinations], dtype=np.float64)
        d_lons = np.array([p.lon for p in destinations], dtype=np.float64)
        return self._engine.compute_non_square_matrix(o_lats, o_lons, d_lats, d_lons)
