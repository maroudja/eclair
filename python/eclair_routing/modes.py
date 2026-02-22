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
        vmin=15.815330130265373,
        vmax=15.595514798630521,
        k=0.03467149051226458,
        f_long=1.166832624243892,
        f_peak=1.0669920596555764,
        d_peak_km=3.007085888879441,
    ),
    "scooter": ModeConfig(
        vmin=13.50447540255183,
        vmax=55.978549131096585,
        k=0.022273423423204278,
        f_long=1.3287380773553794,
        f_peak=1.2572196411467227,
        d_peak_km=2.1515598765613206,
    ),
    "foot": ModeConfig(
        vmin=5.613404457360978,
        vmax=5.435092391667871,
        k=0.05605399382151624,
        f_long=1.1575157252964234,
        f_peak=1.0714537540904385,
        d_peak_km=1.1556513614627515,
    ),
}
