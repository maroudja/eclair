from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class ModeConfig:
    """Speed-model parameters for a transport mode."""

    vmin: float
    vmax: float
    k: float
    f_long: float
    f_peak: float
    d_peak_km: float


MODES: dict[str, ModeConfig] = {
    "car": ModeConfig(
        vmin=23.32592494163679,
        vmax=98.02958396564767,
        k=0.016879550541709618,
        f_long=1.271956898107392,
        f_peak=1.5821720416475133,
        d_peak_km=3.603937984322428,
    ),
    "truck": ModeConfig(
        vmin=27.824789821590215,
        vmax=114.0514532096609,
        k=0.01623845117885414,
        f_long=1.2713675728158658,
        f_peak=1.3918550161999543,
        d_peak_km=2.1068746500018283,
    ),
    "bike": ModeConfig(
        vmin=10.0, vmax=25.0, k=0.03, f_long=1.15, f_peak=1.35, d_peak_km=3.0
    ),
    "scooter": ModeConfig(
        vmin=12.0, vmax=60.0, k=0.016, f_long=1.18, f_peak=1.40, d_peak_km=4.0
    ),
    "foot": ModeConfig(
        vmin=4.0, vmax=5.5, k=0.05, f_long=1.10, f_peak=1.25, d_peak_km=2.0
    ),
}
