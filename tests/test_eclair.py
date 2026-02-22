import numpy as np
from eclair_routing import EclairEngine


def test_instantiate():
    engine = EclairEngine()
    assert engine is not None


def test_estimate_travel_paris_lyon():
    engine = EclairEngine()
    dist, time = engine.estimate_travel(48.8566, 2.3522, 45.7640, 4.8357)
    assert 400_000 < dist < 500_000, f"distance: {dist}"
    assert 4 * 3600 < time < 7 * 3600, f"time: {time}"


def test_estimate_travel_same_point():
    engine = EclairEngine()
    dist, time = engine.estimate_travel(48.8566, 2.3522, 48.8566, 2.3522)
    assert dist == 0.0
    assert time == 0.0


def test_compute_matrix():
    engine = EclairEngine()
    lats = np.array([48.8566, 45.7640, 43.2965])
    lons = np.array([2.3522, 4.8357, 5.3698])

    dist_matrix, time_matrix = engine.compute_matrix(lats, lons)

    assert dist_matrix.shape == (3, 3)
    assert time_matrix.shape == (3, 3)
    # Diagonal should be zero
    for i in range(3):
        assert dist_matrix[i, i] == 0.0
        assert time_matrix[i, i] == 0.0
    # Symmetric
    for i in range(3):
        for j in range(3):
            assert dist_matrix[i, j] == dist_matrix[j, i]
            assert time_matrix[i, j] == time_matrix[j, i]


def test_compute_non_square_matrix():
    engine = EclairEngine()
    origin_lats = np.array([48.8566, 45.7640])
    origin_lons = np.array([2.3522, 4.8357])
    dest_lats = np.array([43.2965, 43.6047, 44.8378])
    dest_lons = np.array([5.3698, 1.4442, -0.5792])

    dist_matrix, time_matrix = engine.compute_non_square_matrix(
        origin_lats, origin_lons, dest_lats, dest_lons
    )

    assert dist_matrix.shape == (2, 3)
    assert time_matrix.shape == (2, 3)
    assert np.all(dist_matrix > 0)
    assert np.all(time_matrix > 0)
