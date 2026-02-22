"""Eclair - Efficient and Compact Library for Approximate Instant Routing.

Fast travel distance and time estimation using Haversine formula
with H3 hexagonal grid density adjustments. Rust-powered, NumPy-native.

Quick start::

    from eclair_routing import Router, Point

    router = Router("car")
    result = router.estimate(Point(48.8566, 2.3522), Point(45.7640, 4.8357))
    print(result)  # TravelResult(distance=470.4 km, time=332 min)

Expert API::

    from eclair_routing import EclairEngine

    engine = EclairEngine(vmin=25.0, vmax=100.0, k=0.02, f_long=1.25)
    dist, time = engine.estimate_travel(48.8566, 2.3522, 45.7640, 4.8357)
"""

from eclair_routing._eclair import PyEclairEngine as EclairEngine
from eclair_routing.models import Point, TravelResult
from eclair_routing.router import Router

__all__ = ["EclairEngine", "Router", "Point", "TravelResult"]
