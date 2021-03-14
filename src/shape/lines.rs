use crate::{Color, Point2, Point3D, Renderer, Shape, Vector2, Vertex};
use cgmath::ElementWise;

#[derive(Debug)]
pub struct Lines {
    position: Vector2,
    size: Vector2,
    vertices: Vec<Vertex>,
    buffer_handle: usize,
    dirty: bool,
}

impl Lines {
    pub fn new(renderer: &mut Renderer, size: Vector2, line_count: usize) -> Self {
        let vertex_count = line_count * 2;
        let index_count = vertex_count;

        let buffer_handle = renderer
            .lines_buffer
            .alloc(vertex_count as u32, index_count as u32)
            .unwrap();

        renderer.lines_buffer.write_indices(
            buffer_handle,
            &(0..(index_count as u32)).collect::<Vec<u32>>(),
        );

        Self {
            position: Vector2::new(0.0, 0.0),
            size,
            vertices: vec![Vertex::default(); vertex_count],
            buffer_handle,
            dirty: true,
        }
    }

    pub fn set_line_position(&mut self, mut line_index: usize, start: Point2, end: Point2) {
        line_index *= 2;
        self.vertices[line_index].position = start.into();
        self.vertices[line_index + 1].position = end.into();
        self.dirty = true;
    }

    pub fn set_line_color(&mut self, mut line_index: usize, color: Color) {
        line_index *= 2;
        self.vertices[line_index].color = color;
        self.vertices[line_index + 1].color = color;
        self.dirty = true;
    }
}

impl Shape for Lines {
    fn set_position(&mut self, position: Vector2) {
        self.position = position;
        self.dirty = true;
    }

    fn resize(&mut self, size: Vector2) {
        let scale = size.div_element_wise(self.size);
        self.size = size;
        for vertex in &mut self.vertices {
            vertex.position.x *= scale.x;
            vertex.position.y *= scale.y;
        }
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

        //renderer.lines_buffer.copy_from_slice_with_position(
        renderer.lines_buffer.write_vertices_with_translation(
            self.buffer_handle,
            &self.vertices,
            Point3D::wgpu_vector(parent_pos + self.position),
        );
    }
}
