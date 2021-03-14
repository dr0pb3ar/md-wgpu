use crate::{Color, Point2, Point3D, Renderer, Shape, Vector2, Vertex};
use std::f32::consts::PI;

#[derive(Debug)]
pub struct Polygon {
    position: Vector2,
    size: Vector2,
    point_count: u8,
    vertices: Vec<Vertex>,
    buffer_handle: usize,
    dirty: bool,
}

impl Polygon {
    pub fn new(renderer: &mut Renderer, size: Vector2, point_count: u8) -> Self {
        let vertices = Self::gen_vertices(size, point_count);
        let mut indices = Vec::with_capacity(point_count as usize * 3);
        for i in 1..(vertices.len() as u32) {
            if i == 1 {
                indices.push(0);
                indices.push(vertices.len() as u32 - 1);
                indices.push(1);
            } else {
                indices.push(0);
                indices.push(i - 1);
                indices.push(i);
            }
        }

        let buffer_handle = renderer
            .triangles_buffer
            .alloc(vertices.len() as u32, indices.len() as u32)
            .unwrap();

        renderer
            .triangles_buffer
            .write_indices(buffer_handle, &indices);

        Self {
            position: Vector2::new(0.0, 0.0),
            size,
            point_count,
            vertices,
            buffer_handle,
            dirty: true,
        }
    }

    fn gen_vertices(size: Vector2, point_count: u8) -> Vec<Vertex> {
        let vertex_count = point_count as usize + 1;
        let mut vertices = Vec::with_capacity(vertex_count);
        vertices.push(Vertex::origin());

        for i in 1..(vertex_count as u32) {
            let angle = i as f32 * 2.0 * PI / point_count as f32 - PI / 2.0;
            vertices.push(Vertex {
                position: Point3D {
                    x: angle.cos() * size.x,
                    y: angle.sin() * size.y,
                    z: 0.0,
                },
                color: Color::default(),
            });
        }

        vertices
    }
}

impl Shape for Polygon {
    fn set_position(&mut self, position: Vector2) {
        self.position = position;
        self.dirty = true;
    }

    fn resize(&mut self, size: Vector2) {
        self.size = size;
        self.vertices = Self::gen_vertices(self.size, self.point_count);
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
            //(parent_pos + self.position + self.size / 2.0).into(),
            (parent_pos + self.position).into(),
        );
    }
}
