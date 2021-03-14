use crate::{Color, Point2, Point3D, Renderer, Shape, Vector2, Vertex};

#[derive(Debug)]
pub struct Rectangle {
    position: Vector2,
    size: Vector2,
    vertices: [Vertex; 4],
    buffer_handle: usize,
    dirty: bool,
}

impl Rectangle {
    pub fn new(renderer: &mut Renderer, size: Vector2) -> Self {
        let mut vertices = [Vertex::origin(); 4];
        let size_3d = Point3D::from(size);
        vertices[1].position.y = size_3d.y;
        vertices[2].position = size_3d;
        vertices[3].position.x = size_3d.x;
        vertices[0].color = Color::RED;
        vertices[1].color = Color::GREEN;
        vertices[2].color = Color::BLUE;
        vertices[3].color = Color::GREEN;

        let buffer_handle = renderer.triangles_buffer.alloc(4, 6).unwrap();
        renderer
            .triangles_buffer
            .write_indices(buffer_handle, &[0, 1, 2, 0, 2, 3]);

        Self {
            position: Vector2::new(0.0, 0.0),
            size,
            vertices,
            buffer_handle,
            dirty: true,
        }
    }
}

impl Shape for Rectangle {
    fn set_position(&mut self, position: Vector2) {
        self.position = position;
        self.dirty = true;
    }

    fn resize(&mut self, size: Vector2) {
        self.size = size;
        let size_3d = Point3D::from(self.size);
        self.vertices[1].position.y = size_3d.y;
        self.vertices[2].position = size_3d;
        self.vertices[3].position.x = size_3d.x;
        self.dirty = true;
    }

    fn set_color(&mut self, color: Color) {
        for vertex in &mut self.vertices {
            vertex.color = color;
        }
        self.dirty = true;
    }

    fn draw(&mut self, renderer: &mut Renderer, parent_pos: Point2) {
        if !self.dirty {
            return;
        }
        self.dirty = false;

        renderer.triangles_buffer.write_vertices_with_translation(
            self.buffer_handle,
            &self.vertices,
            Point3D::from(parent_pos + self.position),
        );
    }
}
