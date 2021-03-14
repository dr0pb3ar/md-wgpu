use md_wgpu::render::{Color, Point2, Renderer, Vector2};
use md_wgpu::shape::*;
/*use stretch::geometry::Size;
use stretch::node::{Node, Stretch};
use stretch::number::Number;
use stretch::style::*;*/

pub trait Widget {
    fn set_position(&mut self, position: Point2);
    fn resize(&mut self, size: Vector2);
    fn draw(&mut self, renderer: &mut Renderer);
}

pub struct Test {
    position: Point2,
    size: Vector2,
    polygon: Polygon,
    rectangle: Rectangle,
}

impl Test {
    pub fn new(renderer: &mut Renderer, size: Vector2) -> Self {
        let mut rectangle = Rectangle::new(&mut *renderer, size);
        let polygon = Polygon::new(&mut *renderer, size, 10);
        //polygon.set_position(size * 0.5);
        //rectangle.set_position(size * 0.5);

        Self {
            position: Point2::new(0.0, 0.0),
            size,
            polygon,
            rectangle,
        }
    }
}

impl Widget for Test {
    fn set_position(&mut self, position: Point2) {
        self.position = position;
        //self.polygon.set_position(position);
    }

    fn resize(&mut self, size: Vector2) {
        self.size = size;
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        self.polygon.draw(&mut *renderer, self.position);
        self.rectangle.draw(&mut *renderer, self.position);
    }
}

pub struct TargetPinpoint {
    position: Point2,
    size: Vector2,
    border: Polygon,
    target_locator: Polygon,
    crosshair: Lines,
    //stretch_node: Option<Node>,
}

impl TargetPinpoint {
    pub fn new(renderer: &mut Renderer, size: Vector2) -> Self {
        let mut border = Polygon::new(&mut *renderer, size, 30);
        border.set_position(size * 0.5);
        border.set_color(Color::BLACK);
        let mut target_locator = Polygon::new(&mut *renderer, size / 10.0, 15);
        target_locator.set_position(size * 0.5);
        target_locator.set_color(Color::GREEN);
        let mut crosshair = Lines::new(&mut *renderer, size, 2);
        crosshair.set_line_position(
            0,
            Point2::new(0.0, size.y / 2.0),
            Point2::new(size.x, size.y / 2.0),
        );
        crosshair.set_line_position(
            1,
            Point2::new(size.x / 2.0, 0.0),
            Point2::new(size.x / 2.0, size.y),
        );
        //rectangle.set_position(size * 0.5);

        Self {
            position: Point2::new(0.0, 0.0),
            size,
            border,
            target_locator,
            crosshair,
            //stretch_node: None,
        }
    }

    /*pub fn node(&mut self, stretch: &mut Stretch) -> Node {
        if let Some(node) = self.stretch_node {
            node
        } else {
            let node = stretch
                .new_leaf(
                    Style {
                        align_self: AlignSelf::Center,
                        aspect_ratio: Number::Defined(1.0),
                        ..Default::default()
                    },
                    Box::new(|constraint| {
                        if let Number::Defined(width) = constraint.width {
                            Ok(Size {
                                width,
                                height: width,
                            })
                        } else if let Number::Defined(height) = constraint.height {
                            Ok(Size {
                                width: height,
                                height,
                            })
                        } else {
                            Ok(Size {
                                width: 0.0,
                                height: 0.0,
                            })
                        }
                    }),
                )
                .unwrap();

            self.stretch_node = Some(node);
            node
        }
    }

    pub fn layout(&mut self, stretch: &Stretch) {
        if let Some(node) = self.stretch_node {
            let layout = stretch.layout(node);
        }
    }*/
}

impl Widget for TargetPinpoint {
    fn set_position(&mut self, position: Point2) {
        self.position = position;
        //self.polygon.set_position(position);
    }

    fn resize(&mut self, size: Vector2) {
        self.size = size;
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        self.border.draw(&mut *renderer, self.position);
        self.target_locator.draw(&mut *renderer, self.position);
        self.crosshair.draw(&mut *renderer, self.position);
    }
}
