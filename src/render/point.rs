use crate::shape::Size;
use crate::{Point2, Vector2};
use std::ops::{Add, Mul};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn origin() -> Self {
        Self::default()
    }

    pub fn to_3d_vector(self) -> Point3D {
        Point3D {
            x: self.x * 2.0,
            y: -self.y * 2.0,
            z: 0.0,
        }
    }
}

impl From<Point3D> for Point2D {
    fn from(point: Point3D) -> Self {
        Self {
            x: (point.x + 1.0) / 2.0,
            y: (1.0 - point.y) / 2.0,
        }
    }
}

impl From<stretch::geometry::Point<f32>> for Point2D {
    fn from(point: stretch::geometry::Point<f32>) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<Size> for Point2D {
    fn from(size: Size) -> Self {
        Self {
            x: size.width,
            y: size.height,
        }
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Point2D {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Default, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn origin() -> Self {
        Self::default()
    }

    /// Converts a 2D point to wgpu vector coords.
    pub fn wgpu_vector(point: Point2) -> Self {
        Self {
            x: point.x * 2.0,
            y: -point.y * 2.0,
            z: 0.0,
        }
    }
}

impl From<Point2D> for Point3D {
    fn from(point: Point2D) -> Point3D {
        Self {
            x: (point.x * 2.0) - 1.0,
            y: 1.0 - (point.y * 2.0),
            z: 0.0,
        }
    }
}

impl From<Size> for Point3D {
    fn from(size: Size) -> Self {
        Self {
            x: size.width * 2.0,
            y: -size.height * 2.0,
            z: 0.0,
        }
    }
}

impl From<Vector2> for Point3D {
    fn from(vector2: Vector2) -> Self {
        Self {
            x: vector2.x * 2.0,
            y: -vector2.y * 2.0,
            z: 0.0,
        }
    }
}

impl From<Point2> for Point3D {
    fn from(point2: Point2) -> Self {
        Self {
            x: (point2.x * 2.0) - 1.0,
            y: 1.0 - (point2.y * 2.0),
            z: 0.0,
        }
    }
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
