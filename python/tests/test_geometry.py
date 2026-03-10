"""Tests for geometry types via Python bindings."""

import pytest
import math
import breprs


class TestPoint:
    """Test Point class."""

    def test_point_creation(self):
        """Test basic point creation."""
        point = breprs.Point(1.0, 2.0, 3.0)
        assert point.x() == 1.0
        assert point.y() == 2.0
        assert point.z() == 3.0

    def test_point_setters(self):
        """Test point coordinate setters."""
        point = breprs.Point(0.0, 0.0, 0.0)
        point.set_x(5.0)
        point.set_y(10.0)
        point.set_z(15.0)
        assert point.x() == 5.0
        assert point.y() == 10.0
        assert point.z() == 15.0

    def test_point_distance(self):
        """Test distance between points."""
        p1 = breprs.Point(0.0, 0.0, 0.0)
        p2 = breprs.Point(3.0, 4.0, 0.0)
        assert p1.distance_to(p2) == 5.0

    def test_point_add_vector(self):
        """Test adding vector to point."""
        point = breprs.Point(1.0, 2.0, 3.0)
        vector = breprs.Vector(10.0, 20.0, 30.0)
        result = point.add(vector)
        assert result.x() == 11.0
        assert result.y() == 22.0
        assert result.z() == 33.0

    def test_point_subtraction(self):
        """Test subtracting points."""
        p1 = breprs.Point(10.0, 20.0, 30.0)
        p2 = breprs.Point(1.0, 2.0, 3.0)
        vector = p1.sub(p2)
        assert vector.x() == 9.0
        assert vector.y() == 18.0
        assert vector.z() == 27.0


class TestVector:
    """Test Vector class."""

    def test_vector_creation(self):
        """Test basic vector creation."""
        vector = breprs.Vector(1.0, 2.0, 3.0)
        assert vector.x() == 1.0
        assert vector.y() == 2.0
        assert vector.z() == 3.0

    def test_vector_magnitude(self):
        """Test vector magnitude."""
        vector = breprs.Vector(3.0, 4.0, 0.0)
        assert vector.magnitude() == 5.0

    def test_vector_normalized(self):
        """Test vector normalization."""
        vector = breprs.Vector(10.0, 0.0, 0.0)
        normalized = vector.normalized()
        assert normalized.magnitude() == 1.0
        assert normalized.x() == 1.0

    def test_vector_dot_product(self):
        """Test vector dot product."""
        v1 = breprs.Vector(1.0, 0.0, 0.0)
        v2 = breprs.Vector(0.0, 1.0, 0.0)
        assert v1.dot(v2) == 0.0

        v3 = breprs.Vector(1.0, 1.0, 0.0)
        assert v3.dot(v3) == 2.0

    def test_vector_cross_product(self):
        """Test vector cross product."""
        v1 = breprs.Vector(1.0, 0.0, 0.0)
        v2 = breprs.Vector(0.0, 1.0, 0.0)
        cross = v1.cross(v2)
        assert cross.x() == 0.0
        assert cross.y() == 0.0
        assert cross.z() == 1.0

    def test_vector_scale(self):
        """Test vector scaling."""
        vector = breprs.Vector(1.0, 2.0, 3.0)
        scaled = vector.scale(2.0)
        assert scaled.x() == 2.0
        assert scaled.y() == 4.0
        assert scaled.z() == 6.0


class TestDirection:
    """Test Direction class."""

    def test_direction_creation(self):
        """Test basic direction creation."""
        direction = breprs.Direction(1.0, 0.0, 0.0)
        assert direction.x() == 1.0
        assert direction.y() == 0.0
        assert direction.z() == 0.0

    def test_direction_reversed(self):
        """Test direction reversal."""
        direction = breprs.Direction(1.0, 0.0, 0.0)
        reversed_dir = direction.reversed()
        assert reversed_dir.x() == -1.0
        assert reversed_dir.y() == 0.0
        assert reversed_dir.z() == 0.0


class TestPlane:
    """Test Plane class."""

    def test_plane_creation(self):
        """Test plane creation from point and normal."""
        origin = breprs.Point(0.0, 0.0, 0.0)
        normal = breprs.Direction(0.0, 0.0, 1.0)
        plane = breprs.Plane(origin, normal)
        assert plane.origin().x() == 0.0
        assert plane.normal().z() == 1.0

    def test_plane_from_points(self):
        """Test plane creation from three points."""
        p1 = breprs.Point(0.0, 0.0, 0.0)
        p2 = breprs.Point(1.0, 0.0, 0.0)
        p3 = breprs.Point(0.0, 1.0, 0.0)
        plane = breprs.Plane.from_points(p1, p2, p3)
        assert plane.normal().z() == 1.0

    def test_plane_distance_to_point(self):
        """Test distance from point to plane."""
        origin = breprs.Point(0.0, 0.0, 0.0)
        normal = breprs.Direction(0.0, 0.0, 1.0)
        plane = breprs.Plane(origin, normal)
        point = breprs.Point(0.0, 0.0, 5.0)
        assert plane.distance_to(point) == 5.0

    def test_plane_project_point(self):
        """Test projecting point onto plane."""
        origin = breprs.Point(0.0, 0.0, 0.0)
        normal = breprs.Direction(0.0, 0.0, 1.0)
        plane = breprs.Plane(origin, normal)
        point = breprs.Point(3.0, 4.0, 5.0)
        projected = plane.project(point)
        assert projected.x() == 3.0
        assert projected.y() == 4.0
        assert projected.z() == 0.0
