# eclair-routing

**Efficient and Compact Library for Approximate Instant Routing**

A lightweight, fast alternative to OSRM, ORS, or Google Maps for estimating travel distances and times between geographic points — without a road network graph.

## The idea

Traditional routing engines (OSRM, OpenRouteService, Google Maps, Here) rely on detailed road network graphs. They're precise, but heavy: large datasets to download, complex setups, and significant compute costs.

**eclair-routing** takes a different approach: estimate travel time using the Haversine formula (great-circle distance), a speed model that accounts for trip length, and an optional H3 hexagonal grid of speed factors to adjust for population density.

The result is surprisingly accurate for many use cases (logistics planning, fleet optimization, isochrone estimation) while being:

- **Fast** — Rust core with NumPy integration, no network calls
- **Light** — no road graph data needed, just an optional config file
- **Simple** — `pip install eclair-routing`, one class, three methods
- **Free** — Apache 2.0, no API keys, no limits

### How it works

1. **Haversine distance** between two points, multiplied by a distance factor that varies with trip length to approximate actual driving distance
2. **Speed model** where average speed increases with distance (short urban trips are slower, long highway trips are faster): `speed = vmin + (vmax - vmin) * (1 - e^(-k * distance))`
3. **H3 density adjustment** (optional): the straight line between origin and destination is traced through H3 hexagons (resolution 7, ~5km edge). Each hex can have a speed factor (0 to 1). The **harmonic mean** of factors along the path adjusts the travel time — crossing a dense city slows you down

## Install

```bash
pip install eclair-routing
```

### From source (development)

```bash
git clone https://github.com/maroudja/eclair.git
cd eclair

python -m venv .venv
source .venv/bin/activate

pip install maturin numpy
maturin develop

# Optional: install dev dependencies for tests
pip install pytest
```

### Run tests

```bash
pytest tests/ -v
cargo test --lib
```

## Usage

### Quick start

```python
from eclair_routing import Router, Point

router = Router("car")
result = router.estimate(Point(48.8566, 2.3522), Point(45.7640, 4.8357))
print(result)  # TravelResult(distance=470.4 km, time=332 min)
```

### Transport modes

Five built-in modes: `car`, `truck`, `bike`, `scooter`, `foot`.

```python
from eclair_routing import Router, Point

router = Router("truck")
result = router.estimate(Point(48.8566, 2.3522), Point(43.2965, 5.3698))
print(f"{result.distance_km:.0f} km, {result.time_hours:.1f} h")
```

### Distance and time matrices

```python
from eclair_routing import Router, Point

router = Router("car")

cities = [
    Point(48.8566, 2.3522),   # Paris
    Point(45.7640, 4.8357),   # Lyon
    Point(43.2965, 5.3698),   # Marseille
]

# Square matrix (n x n)
dist_matrix, time_matrix = router.matrix(cities)

# Non-square matrix (origins x destinations)
origins = [Point(48.8566, 2.3522), Point(45.7640, 4.8357)]
destinations = [Point(43.2965, 5.3698), Point(43.6047, 1.4442)]

dist_matrix, time_matrix = router.matrix_od(origins, destinations)
```

### Custom H3 density config

```python
router = Router("car", config_path="factors.parquet")  # CSV or Parquet
router = Router("car", config_path=None)                # disable config
```

The config file maps H3 cell indexes (resolution 7) to speed factors:

```csv
h3_index,factor
872a1008fffffff,0.3
872a1009fffffff,0.8
```

- `factor = 1.0` — normal speed (no adjustment)
- `factor = 0.5` — half speed (travel time doubled)
- `factor = 0.1` — very slow (dense city center)
- Hexagons not in the file default to `1.0`

### Expert API — EclairEngine

For full control over speed-model parameters, use `EclairEngine` directly:

```python
from eclair_routing import EclairEngine

engine = EclairEngine(
    vmin=25.0,       # min speed km/h (short trips)
    vmax=100.0,      # max speed km/h (long trips)
    k=0.02,          # speed curve steepness
    f_long=1.25,     # asymptotic distance factor (long trips)
    f_peak=1.45,     # peak distance factor (medium trips)
    d_peak_km=5.0,   # distance at which factor peaks (km)
)

dist, time = engine.estimate_travel(48.8566, 2.3522, 45.7640, 4.8357)
print(f"{dist/1000:.0f} km, {time/3600:.1f} hours")
```

## License

Apache 2.0 — see [LICENSE](LICENSE)
