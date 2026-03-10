"""
BrepRs - Python bindings for Rust CAD kernel

This package provides Python bindings for the BrepRs library,
a Rust implementation of boundary representation for CAD/CAE/CAM applications.

Example usage:
    >>> import breprs
    >>> 
    >>> # Create a box
    >>> box = breprs.Box(10.0, 10.0, 10.0)
    >>> print(f"Box volume: {box.volume()}")
    >>> 
    >>> # Create a sphere
    >>> sphere = breprs.Sphere(5.0)
    >>> print(f"Sphere volume: {sphere.volume()}")
    >>> 
    >>> # Boolean operations
    >>> boolean_ops = breprs.BooleanOperations()
    >>> result = boolean_ops.fuse(box.to_solid(), sphere.to_solid())
"""

# Import all classes from the Rust extension module
try:
    from .breprs import (
        # Version
        __version__,
        
        # Geometry
        Point,
        Vector,
        Direction,
        Axis,
        Plane,
        
        # Topology
        Vertex,
        Edge,
        Wire,
        Face,
        Shell,
        Solid,
        Compound,
        
        # Primitives
        Box,
        Sphere,
        Cylinder,
        Cone,
        Torus,
        
        # Modeling
        BrepBuilder,
        BooleanOperations,
        FilletChamfer,
        OffsetOperations,
        
        # Utilities
        version,
        set_tolerance,
        get_tolerance,
    )
except ImportError:
    # Fallback for development without compiled extension
    import warnings
    warnings.warn(
        "BrepRs extension module not found. "
        "Please build the extension with: maturin develop",
        RuntimeWarning,
        stacklevel=2,
    )
    
    # Provide stub implementations for development
    __version__ = "0.0.0-dev"
    
    def version() -> str:
        return __version__
    
    _tolerance = 1e-6

    def set_tolerance(tol: float) -> None:
        global _tolerance
        _tolerance = tol

    def get_tolerance() -> float:
        return _tolerance

__all__ = [
    # Version
    "__version__",
    "version",
    
    # Geometry
    "Point",
    "Vector",
    "Direction",
    "Axis",
    "Plane",
    
    # Topology
    "Vertex",
    "Edge",
    "Wire",
    "Face",
    "Shell",
    "Solid",
    "Compound",
    
    # Primitives
    "Box",
    "Sphere",
    "Cylinder",
    "Cone",
    "Torus",
    
    # Modeling
    "BrepBuilder",
    "BooleanOperations",
    "FilletChamfer",
    "OffsetOperations",
    
    # Utilities
    "set_tolerance",
    "get_tolerance",
]
