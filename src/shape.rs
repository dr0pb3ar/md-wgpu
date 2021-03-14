mod lines;
pub use lines::Lines;
mod polygon;
pub use polygon::Polygon;
mod rectangle;
pub use rectangle::Rectangle;
mod text;
pub use text::Text;

use crate::{Color, Point2, Renderer, Vector2};
use std::ops::{Div, Mul};

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Mul for Size {
    type Output = Size;

    fn mul(self, other: Self) -> Self {
        Self {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl Div for Size {
    type Output = Size;

    fn div(self, other: Self) -> Self {
        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}

impl Div<f32> for Size {
    type Output = Size;

    fn div(self, scalar: f32) -> Self {
        Self {
            width: self.width / scalar,
            height: self.height / scalar,
        }
    }
}

impl From<stretch::geometry::Size<f32>> for Size {
    fn from(stretch_size: stretch::geometry::Size<f32>) -> Self {
        Self {
            width: stretch_size.width,
            height: stretch_size.height,
        }
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for Size {
    fn from(physical_size: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            width: physical_size.width as f32,
            height: physical_size.height as f32,
        }
    }
}

pub trait Shape {
    fn set_position(&mut self, position: Vector2);
    fn resize(&mut self, size: Vector2);
    fn set_color(&mut self, color: Color);
    fn draw(&mut self, renderer: &mut Renderer, parent_pos: Point2);
}
