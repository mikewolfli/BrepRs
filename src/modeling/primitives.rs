//! Primitive creation algorithms
//!
//! This module provides functions for creating basic geometric primitives.

use crate::foundation::handle::Handle;
use crate::geometry::{Axis, Cylinder, Direction, Plane, Point, Sphere};
use crate::topology::{TopoDsEdge, TopoDsFace, TopoDsShell, TopoDsSolid, TopoDsVertex, TopoDsWire};
use std::sync::Arc;

/// Create a box with given dimensions
///
/// # Arguments
/// * `width` - Width of the box (X direction)
/// * `height` - Height of the box (Y direction)
/// * `depth` - Depth of the box (Z direction)
/// * `center` - Center point of the box (default: origin)
///
/// # Returns
/// A solid representing the box
pub fn make_box(width: f64, height: f64, depth: f64, center: Option<Point>) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());

    // Calculate corner points
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    let half_depth = depth / 2.0;

    let p1 = Point::new(
        center.x - half_width,
        center.y - half_height,
        center.z - half_depth,
    );
    let p2 = Point::new(
        center.x + half_width,
        center.y - half_height,
        center.z - half_depth,
    );
    let p3 = Point::new(
        center.x + half_width,
        center.y + half_height,
        center.z - half_depth,
    );
    let p4 = Point::new(
        center.x - half_width,
        center.y + half_height,
        center.z - half_depth,
    );
    let p5 = Point::new(
        center.x - half_width,
        center.y - half_height,
        center.z + half_depth,
    );
    let p6 = Point::new(
        center.x + half_width,
        center.y - half_height,
        center.z + half_depth,
    );
    let p7 = Point::new(
        center.x + half_width,
        center.y + half_height,
        center.z + half_depth,
    );
    let p8 = Point::new(
        center.x - half_width,
        center.y + half_height,
        center.z + half_depth,
    );

    // Create vertices
    let v1 = Handle::new(Arc::new(TopoDsVertex::new(p1)));
    let v2 = Handle::new(Arc::new(TopoDsVertex::new(p2)));
    let v3 = Handle::new(Arc::new(TopoDsVertex::new(p3)));
    let v4 = Handle::new(Arc::new(TopoDsVertex::new(p4)));
    let v5 = Handle::new(Arc::new(TopoDsVertex::new(p5)));
    let v6 = Handle::new(Arc::new(TopoDsVertex::new(p6)));
    let v7 = Handle::new(Arc::new(TopoDsVertex::new(p7)));
    let v8 = Handle::new(Arc::new(TopoDsVertex::new(p8)));

    // Create edges
    let e1 = Handle::new(Arc::new(TopoDsEdge::new(v1.clone(), v2.clone())));
    let e2 = Handle::new(Arc::new(TopoDsEdge::new(v2.clone(), v3.clone())));
    let e3 = Handle::new(Arc::new(TopoDsEdge::new(v3.clone(), v4.clone())));
    let e4 = Handle::new(Arc::new(TopoDsEdge::new(v4.clone(), v1.clone())));
    let e5 = Handle::new(Arc::new(TopoDsEdge::new(v5.clone(), v6.clone())));
    let e6 = Handle::new(Arc::new(TopoDsEdge::new(v6.clone(), v7.clone())));
    let e7 = Handle::new(Arc::new(TopoDsEdge::new(v7.clone(), v8.clone())));
    let e8 = Handle::new(Arc::new(TopoDsEdge::new(v8.clone(), v5.clone())));
    let e9 = Handle::new(Arc::new(TopoDsEdge::new(v1.clone(), v5.clone())));
    let e10 = Handle::new(Arc::new(TopoDsEdge::new(v2.clone(), v6.clone())));
    let e11 = Handle::new(Arc::new(TopoDsEdge::new(v3.clone(), v7.clone())));
    let e12 = Handle::new(Arc::new(TopoDsEdge::new(v4.clone(), v8.clone())));

    // Create wires (faces)
    let mut w1 = TopoDsWire::new();
    w1.add_edge(e1.clone());
    w1.add_edge(e2.clone());
    w1.add_edge(e3.clone());
    w1.add_edge(e4.clone());

    let mut w2 = TopoDsWire::new();
    w2.add_edge(e5.clone());
    w2.add_edge(e6.clone());
    w2.add_edge(e7.clone());
    w2.add_edge(e8.clone());

    let mut w3 = TopoDsWire::new();
    w3.add_edge(e1.clone());
    w3.add_edge(e9.clone());
    w3.add_edge(e5.clone());
    w3.add_edge(e10.clone());

    let mut w4 = TopoDsWire::new();
    w4.add_edge(e2.clone());
    w4.add_edge(e10.clone());
    w4.add_edge(e6.clone());
    w4.add_edge(e11.clone());

    let mut w5 = TopoDsWire::new();
    w5.add_edge(e3.clone());
    w5.add_edge(e11.clone());
    w5.add_edge(e7.clone());
    w5.add_edge(e12.clone());

    let mut w6 = TopoDsWire::new();
    w6.add_edge(e4.clone());
    w6.add_edge(e12.clone());
    w6.add_edge(e8.clone());
    w6.add_edge(e9.clone());

    // Create faces
    let f1 = TopoDsFace::with_outer_wire(w1);
    let f2 = TopoDsFace::with_outer_wire(w2);
    let f3 = TopoDsFace::with_outer_wire(w3);
    let f4 = TopoDsFace::with_outer_wire(w4);
    let f5 = TopoDsFace::with_outer_wire(w5);
    let f6 = TopoDsFace::with_outer_wire(w6);

    // Create shell
    let mut shell = TopoDsShell::new();
    shell.add_face(Handle::new(Arc::new(f1)));
    shell.add_face(Handle::new(Arc::new(f2)));
    shell.add_face(Handle::new(Arc::new(f3)));
    shell.add_face(Handle::new(Arc::new(f4)));
    shell.add_face(Handle::new(Arc::new(f5)));
    shell.add_face(Handle::new(Arc::new(f6)));

    // Create solid
    let mut solid = TopoDsSolid::new();
    solid.add_shell(Handle::new(Arc::new(shell)));

    solid
}

/// Create a sphere with given radius
///
/// # Arguments
/// * `radius` - Radius of the sphere
/// * `center` - Center point of the sphere (default: origin)
///
/// # Returns
/// A solid representing the sphere
pub fn make_sphere(radius: f64, center: Option<Point>) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());

    // Create sphere surface
    let sphere = Sphere::new(center, radius);

    // Create face with sphere surface
    let face = TopoDsFace::with_surface(Handle::new(Arc::new(sphere)));

    // Create shell
    let mut shell = TopoDsShell::new();
    shell.add_face(Handle::new(Arc::new(face)));

    // Create solid
    let mut solid = TopoDsSolid::new();
    solid.add_shell(Handle::new(Arc::new(shell)));

    solid
}

/// Create a cylinder with given radius and height
///
/// # Arguments
/// * `radius` - Radius of the cylinder
/// * `height` - Height of the cylinder
/// * `center` - Center point of the cylinder base (default: origin)
/// * `axis` - Axis of the cylinder (default: Z-axis)
///
/// # Returns
/// A solid representing the cylinder
pub fn make_cylinder(
    radius: f64,
    height: f64,
    center: Option<Point>,
    axis: Option<Axis>,
) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());
    let axis = axis.unwrap_or(Axis::z_axis());

    // Create cylinder surface
    let cylinder = Cylinder::new(*axis.location(), *axis.direction(), radius);

    // Create faces
    // 1. Side face (cylinder surface)
    let side_face = TopoDsFace::with_surface(Handle::new(Arc::new(cylinder)));

    // 2. Bottom face (circle)
    let bottom_center = center;
    let bottom_plane = Plane::new(bottom_center, *axis.direction(), Direction::x_axis());
    let bottom_face = TopoDsFace::with_surface(Handle::new(Arc::new(bottom_plane)));

    // 3. Top face (circle)
    let top_center = Point::new(
        axis.location().x + axis.direction().x * height,
        axis.location().y + axis.direction().y * height,
        axis.location().z + axis.direction().z * height,
    );
    let top_plane = Plane::new(top_center, *axis.direction(), Direction::x_axis());
    let top_face = TopoDsFace::with_surface(Handle::new(Arc::new(top_plane)));

    // Create shell
    let mut shell = TopoDsShell::new();
    shell.add_face(Handle::new(Arc::new(side_face)));
    shell.add_face(Handle::new(Arc::new(bottom_face)));
    shell.add_face(Handle::new(Arc::new(top_face)));

    // Create solid
    let mut solid = TopoDsSolid::new();
    solid.add_shell(Handle::new(Arc::new(shell)));

    solid
}

/// Create a cone with given radii and height
///
/// # Arguments
/// * `radius1` - Radius of the cone at the base
/// * `radius2` - Radius of the cone at the top
/// * `height` - Height of the cone
/// * `center` - Center point of the cone base (default: origin)
/// * `axis` - Axis of the cone (default: Z-axis)
///
/// # Returns
/// A solid representing the cone
pub fn make_cone(
    radius1: f64,
    _radius2: f64,
    height: f64,
    center: Option<Point>,
    axis: Option<Axis>,
) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());
    let axis = axis.unwrap_or(Axis::z_axis());

    // TODO: Implement cone creation
    // This will require a Cone surface type

    // For now, return a cylinder as a placeholder
    make_cylinder(radius1, height, Some(center), Some(axis))
}

/// Create a torus with given radii
///
/// # Arguments
/// * `major_radius` - Major radius of the torus (distance from center to tube center)
/// * `minor_radius` - Minor radius of the torus (tube radius)
/// * `center` - Center point of the torus (default: origin)
///
/// # Returns
/// A solid representing the torus
pub fn make_torus(_major_radius: f64, minor_radius: f64, center: Option<Point>) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());

    // TODO: Implement torus creation
    // This will require a Torus surface type

    // For now, return a sphere as a placeholder
    make_sphere(minor_radius, Some(center))
}

/// Create a prism by extruding a wire along a vector
///
/// # Arguments
/// * `wire` - Wire to extrude
/// * `vector` - Extrusion vector
///
/// # Returns
/// A solid representing the prism
pub fn make_prism(_wire: &TopoDsWire, _vector: &crate::geometry::Vector) -> TopoDsSolid {
    // TODO: Implement prism creation

    // For now, return a box as a placeholder
    make_box(1.0, 1.0, 1.0, None)
}

/// Create a revolution by rotating a wire around an axis
///
/// # Arguments
/// * `wire` - Wire to rotate
/// * `axis` - Rotation axis
/// * `angle` - Rotation angle in radians
///
/// # Returns
/// A solid representing the revolution
pub fn make_revolution(_wire: &TopoDsWire, axis: &Axis, _angle: f64) -> TopoDsSolid {
    // TODO: Implement revolution creation

    // For now, return a cylinder as a placeholder
    make_cylinder(1.0, 1.0, None, Some(*axis))
}
