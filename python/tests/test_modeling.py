"""Tests for modeling operations via Python bindings."""

import pytest
import breprs


class TestBrepBuilder:
    """Test BRep builder operations."""

    def test_builder_creation(self):
        """Test builder creation."""
        builder = breprs.BrepBuilder()
        assert builder is not None

    def test_make_vertex(self):
        """Test vertex creation."""
        builder = breprs.BrepBuilder()
        point = breprs.Point(1.0, 2.0, 3.0)
        vertex = builder.make_vertex(point)
        assert vertex is not None
        assert vertex.point().x() == 1.0

    def test_make_edge(self):
        """Test edge creation."""
        builder = breprs.BrepBuilder()
        v1 = builder.make_vertex(breprs.Point(0.0, 0.0, 0.0))
        v2 = builder.make_vertex(breprs.Point(1.0, 0.0, 0.0))
        edge = builder.make_edge(v1, v2)
        assert edge is not None

    def test_make_wire(self):
        """Test wire creation."""
        builder = breprs.BrepBuilder()
        wire = builder.make_wire()
        assert wire is not None

    def test_make_face(self):
        """Test face creation."""
        builder = breprs.BrepBuilder()
        face = builder.make_face()
        assert face is not None

    def test_make_shell(self):
        """Test shell creation."""
        builder = breprs.BrepBuilder()
        shell = builder.make_shell()
        assert shell is not None

    def test_make_solid(self):
        """Test solid creation."""
        builder = breprs.BrepBuilder()
        solid = builder.make_solid()
        assert solid is not None


class TestBooleanOperations:
    """Test boolean operations."""

    def test_boolean_ops_creation(self):
        """Test boolean operations creation."""
        ops = breprs.BooleanOperations()
        assert ops is not None

    def test_fuse(self):
        """Test fuse operation."""
        box = breprs.Box(10.0, 10.0, 10.0)
        sphere = breprs.Sphere(5.0)
        ops = breprs.BooleanOperations()
        result = ops.fuse(box.to_solid(), sphere.to_solid())
        assert result is not None

    def test_cut(self):
        """Test cut operation."""
        box = breprs.Box(10.0, 10.0, 10.0)
        sphere = breprs.Sphere(3.0)
        ops = breprs.BooleanOperations()
        result = ops.cut(box.to_solid(), sphere.to_solid())
        assert result is not None

    def test_common(self):
        """Test common (intersection) operation."""
        box = breprs.Box(10.0, 10.0, 10.0)
        sphere = breprs.Sphere(5.0)
        ops = breprs.BooleanOperations()
        result = ops.common(box.to_solid(), sphere.to_solid())
        assert result is not None


class TestFilletChamfer:
    """Test fillet and chamfer operations."""

    def test_fillet_chamfer_creation(self):
        """Test fillet/chamfer creation."""
        fc = breprs.FilletChamfer()
        assert fc is not None
        assert fc.radius() == 0.1
        assert fc.chamfer_distance() == 0.1

    def test_fillet_chamfer_with_radius(self):
        """Test creation with specific radius."""
        fc = breprs.FilletChamfer.with_radius(2.0)
        assert fc.radius() == 2.0

    def test_fillet_chamfer_with_chamfer(self):
        """Test creation with specific chamfer distance."""
        fc = breprs.FilletChamfer.with_chamfer_distance(1.5)
        assert fc.chamfer_distance() == 1.5

    def test_set_radius(self):
        """Test setting radius."""
        fc = breprs.FilletChamfer()
        fc.set_radius(3.0)
        assert fc.radius() == 3.0

    def test_set_chamfer_distance(self):
        """Test setting chamfer distance."""
        fc = breprs.FilletChamfer()
        fc.set_chamfer_distance(2.0)
        assert fc.chamfer_distance() == 2.0

    def test_apply_fillet(self):
        """Test applying fillet to solid."""
        box = breprs.Box(10.0, 10.0, 10.0)
        fc = breprs.FilletChamfer.with_radius(1.0)
        result = fc.apply_fillet(box.to_solid())
        assert result is not None

    def test_apply_chamfer(self):
        """Test applying chamfer to solid."""
        box = breprs.Box(10.0, 10.0, 10.0)
        fc = breprs.FilletChamfer.with_chamfer_distance(1.0)
        result = fc.apply_chamfer(box.to_solid())
        assert result is not None


class TestOffsetOperations:
    """Test offset operations."""

    def test_offset_ops_creation(self):
        """Test offset operations creation."""
        ops = breprs.OffsetOperations()
        assert ops is not None
        assert ops.offset_distance() == 0.1

    def test_offset_ops_with_distance(self):
        """Test creation with specific distance."""
        ops = breprs.OffsetOperations.with_offset_distance(2.0)
        assert ops.offset_distance() == 2.0

    def test_set_offset_distance(self):
        """Test setting offset distance."""
        ops = breprs.OffsetOperations()
        ops.set_offset_distance(5.0)
        assert ops.offset_distance() == 5.0

    def test_set_tolerance(self):
        """Test setting tolerance."""
        ops = breprs.OffsetOperations()
        ops.set_tolerance(0.01)
        assert ops.tolerance() == 0.01
