import numpy as np
import pytest

from eclair_routing import Router, Point


PARIS = Point(48.8566, 2.3522)
LYON = Point(45.7640, 4.8357)
MARSEILLE = Point(43.2965, 5.3698)


def test_router_car_estimate():
    router = Router("car")
    result = router.estimate(PARIS, LYON)
    assert 400_000 < result.distance_meters < 500_000
    assert 4.0 < result.time_hours < 7.0


def test_router_foot_slower_than_car():
    car = Router("car").estimate(PARIS, LYON)
    foot = Router("foot").estimate(PARIS, LYON)
    assert foot.time_seconds > car.time_seconds


def test_router_same_point():
    router = Router("car")
    result = router.estimate(PARIS, PARIS)
    assert result.distance_meters == 0.0
    assert result.time_seconds == 0.0


def test_router_matrix():
    router = Router("car")
    points = [PARIS, LYON, MARSEILLE]
    dist_mat, time_mat = router.matrix(points)

    assert dist_mat.shape == (3, 3)
    assert time_mat.shape == (3, 3)
    # symmetric
    np.testing.assert_array_almost_equal(dist_mat, dist_mat.T)
    # zero diagonal
    np.testing.assert_array_equal(np.diag(dist_mat), 0.0)


def test_router_matrix_od():
    router = Router("bike")
    origins = [PARIS, LYON]
    destinations = [LYON, MARSEILLE]
    dist_mat, time_mat = router.matrix_od(origins, destinations)

    assert dist_mat.shape == (2, 2)
    assert time_mat.shape == (2, 2)


def test_router_invalid_mode():
    with pytest.raises(ValueError, match="Unknown mode"):
        Router("helicopter")


def test_travel_result_repr():
    router = Router("car")
    result = router.estimate(PARIS, LYON)
    text = repr(result)
    assert "km" in text
    assert "min" in text


def test_travel_result_convenience_properties():
    router = Router("car")
    result = router.estimate(PARIS, LYON)
    assert abs(result.distance_km - result.distance_meters / 1000.0) < 0.01
    assert abs(result.time_minutes - result.time_seconds / 60.0) < 0.01
    assert abs(result.time_hours - result.time_seconds / 3600.0) < 0.001
