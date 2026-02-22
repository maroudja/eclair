from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class Point:
    """A geographic point defined by latitude and longitude in decimal degrees."""

    lat: float
    lon: float


@dataclass(frozen=True, slots=True)
class TravelResult:
    """Result of a travel estimation between two points."""

    distance_meters: float
    time_seconds: float

    @property
    def distance_km(self) -> float:
        return self.distance_meters / 1000.0

    @property
    def time_minutes(self) -> float:
        return self.time_seconds / 60.0

    @property
    def time_hours(self) -> float:
        return self.time_seconds / 3600.0

    def __repr__(self) -> str:
        return (
            f"TravelResult(distance={self.distance_km:.1f} km, "
            f"time={self.time_minutes:.0f} min)"
        )
