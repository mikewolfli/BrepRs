"""Tests for primitive creation via Python bindings."""

import pytest
import breprs


class TestBox:
    """Test box primitive creation."""

    def test_box_creation(self):
        """Test basic box creation."""
        box = breprs.Box(10.0, 20.0, 30.0)
        assert box.width() == 10.0
        assert box.height() == 20.0
        assert box.depth() == 30.0

    def test_box_volume(self):
        """Test box volume calculation."""
        box = breprs.Box(10.0, 20.0, 30.0)
        assert box.volume() == 6000.0

    def test_box_at_position(self):
        """Test box creation at specific position."""
        point = breprs.Point(5.0, 10.0, 15.0)
        box = breprs.Box.at(10.0, 20.0, 30.0, point)
        assert box.width() == 10.0

    def test_box_to_solid(self):
        """Test converting box to solid."""
        box = breprs.Box(10.0, 10.0, 10.0)
        solid = box.to_solid()
        assert solid is not None


class TestSphere:
    """Test sphere primitive creation."""

    def test_sphere_creation(self):
        """Test basic sphere creation."""
        sphere = breprs.Sphere(5.0)
        assert sphere.radius() == 5.0

    def test_sphere_volume(self):
        """Test sphere volume calculation."""
        import math
        sphere = breprs.Sphere(5.0)
        expected_volume = (4.0 / 3.0) * math.pi * 125.0
        assert abs(sphere.volume() - expected_volume) < 1e-10

    def test_sphere_surface_area(self):
        """Test sphere surface area calculation."""
        import math
        sphere = breprs.Sphere(5.0)
        expected_area = 4.0 * math.pi * 25.0
        assert abs(sphere.surface_area() - expected_area) < 1e-10

    def test_sphere_at_position(self):
        """Test sphere creation at specific position."""
        center = breprs.Point(10.0, 20.0, 30.0)
        sphere = breprs.Sphere.at(5.0, center)
        assert sphere.radius() == 5.0


class TestCylinder:
    """Test cylinder primitive creation."""

    def test_cylinder_creation(self):
        """Test basic cylinder creation."""
        cylinder = breprs.Cylinder(5.0, 10.0)
        assert cylinder.radius() == 5.0
        assert cylinder.height() == 10.0

    def test_cylinder_volume(self):
        """Test cylinder volume calculation."""
        import math
        cylinder = breprs.Cylinder(5.0, 10.0)
        expected_volume = math.pi * 25.0 * 10.0
        assert abs(cylinder.volume() - expected_volume) < 1e-10


class TestCone:
    """Test cone primitive creation."""

    def test_cone_creation(self):
        """Test basic cone creation."""
        cone = breprs.Cone(5.0, 2.0, 10.0)
        assert cone.radius1() == 5.0
        assert cone.radius2() == 2.0
        assert cone.height() == 10.0


class TestTorus:
    """Test torus primitive creation."""

    def test_torus_creation(self):
        """Test basic torus creation."""
        torus = breprs.Torus(10.0, 2.0)
        assert torus.major_radius() == 10.0
        assert torus.minor_radius() == 2.0
