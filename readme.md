Computer Vector Graphics Math Library
=====================================

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/cvmath.svg)](https://crates.io/crates/cvmath)
[![docs.rs](https://docs.rs/cvmath/badge.svg)](https://docs.rs/cvmath)
[![Build status](https://github.com/CasualX/cvmath/workflows/CI/badge.svg)](https://github.com/CasualX/cvmath/actions)

Because everyone should at least attempt to write one.

What is supported
-----------------

All types are exported at the crate root. While many operations are implemented on generic types, their generic traits are not exported.

### Vectors and points

`Vec2<T>`, `Vec3<T>`, `Vec4<T>`, `Point2<T>`, `Point3<T>`

The vector types are versatile and have many uses. They can be used to represent points, directions, colors, masks and more.

The point types are simply aliases for their respective vector types. The types are interchangeable and are for visual clarity only.

### Matrices and transformations

`Mat2<T>`, `Mat3<T>`, `Mat4<T>`, `Transform2<T>`, `Transform3<T>`

Square and affine transformation matrices.

### Shapes

`Bounds<V>`, `Rect<T>`, `Cuboid<T>`, `Line<V>`, `Line2<T>`, `Line3<T>`, `Sphere<T>`, `Plane<T>`, `Ray<T>`

Rect and Cuboid are 2D and 3D specialized cases of Bounds. Bounds are defined by their minimum and maximum points. They are axis-aligned and can be used to represent any shape that can be enclosed in a bounding box.

Line2 and Line3 are 2D and 3D specialized cases of Line. Lines are defined by two points.

Sphere is defined by a center point and a radius.

Plane is defined by a normal vector and a distance from the origin.

Ray is defined by an origin point and a direction vector. Rays are used to compute intersections with other shapes.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
