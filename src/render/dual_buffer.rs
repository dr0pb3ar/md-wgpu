use crate::render::{Point3D, Vertex};

/// Represents allocated blocks in a set of vertex and index buffers.
struct Allocation {
    vertex_offset: wgpu::BufferAddress,
    num_vertices: u32,
    index_offset: wgpu::BufferAddress,
    num_indices: u32,
}

/// Holds and controls access to a set of vertex and index buffers.
pub struct DualBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub vertices: Vec<Vertex>,
    max_vertices: u32,
    vertices_allocated: u32,
    pub index_buffer: wgpu::Buffer,
    pub indices: Vec<u32>,
    max_indices: u32,
    indices_allocated: u32,
    allocations: Vec<Allocation>,
    dirty: bool,
}

impl DualBuffer {
    pub fn new(device: &wgpu::Device, label: &str, max_vertices: u64, max_indices: u64) -> Self {
        let vertex_size = std::mem::size_of::<Vertex>() as u64;
        let mut usage = wgpu::BufferUsage::VERTEX;
        usage.insert(wgpu::BufferUsage::COPY_DST);
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{} Vertex Buffer", label)),
            size: vertex_size * max_vertices,
            usage,
            mapped_at_creation: false,
        });

        let mut usage = wgpu::BufferUsage::INDEX;
        usage.insert(wgpu::BufferUsage::COPY_DST);
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{} Index Buffer", label)),
            size: 4 * max_indices,
            usage,
            mapped_at_creation: false,
        });

        Self {
            vertex_buffer,
            vertices: vec![Vertex::default(); max_vertices as usize],
            max_vertices: max_vertices as u32,
            vertices_allocated: 0,
            index_buffer,
            indices: vec![0; max_indices as usize],
            max_indices: max_indices as u32,
            indices_allocated: 0,
            allocations: Vec::new(),
            dirty: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.vertices_allocated == 0 || self.indices_allocated == 0 {
            true
        } else {
            false
        }
    }

    pub fn indices_len(&self) -> u32 {
        self.indices_allocated
    }

    /// Allocate sections of the buffers and return a handle index.
    pub fn alloc(&mut self, num_vertices: u32, num_indices: u32) -> Result<usize, &'static str> {
        if num_vertices <= (self.max_vertices - self.vertices_allocated)
            && num_indices <= (self.max_indices - self.indices_allocated)
        {
            self.allocations.push(Allocation {
                vertex_offset: self.vertices_allocated as u64,
                num_vertices,
                index_offset: self.indices_allocated as u64,
                num_indices,
            });
            self.vertices_allocated += num_vertices;
            self.indices_allocated += num_indices;
            self.vertices
                .resize(self.vertices_allocated as usize, Vertex::default());
            self.indices.resize(self.indices_allocated as usize, 0);
            Ok(self.allocations.len() - 1)
        } else {
            Err("Not enough space for allocation.")
        }
    }

    pub fn vertex_offset(&self, index: usize) -> u32 {
        self.allocations[index].vertex_offset as u32
    }

    pub fn get_mut_slice(&mut self, index: usize) -> Option<(&mut [Vertex], &mut [u32])> {
        if index < self.allocations.len() {
            let alloc = &self.allocations[index];
            self.dirty = true;

            Some((
                &mut self.vertices[(alloc.vertex_offset as usize)
                    ..(alloc.vertex_offset as usize + alloc.num_vertices as usize)],
                &mut self.indices[(alloc.index_offset as usize)
                    ..(alloc.index_offset as usize + alloc.num_indices as usize)],
            ))
        } else {
            None
        }
    }

    /// Copy vertices into staging buffer and apply a translation.
    pub fn write_vertices_with_translation(
        &mut self,
        index: usize,
        vertices: &[Vertex],
        translation: Point3D,
    ) {
        if let Some((dst_vertices, _dst_indices)) = self.get_mut_slice(index) {
            for (src, dst) in vertices.iter().zip(dst_vertices.iter_mut()) {
                *dst = Vertex {
                    position: src.position + translation,
                    color: src.color,
                };
            }
        }
    }

    /// Copy indices into staging buffer.
    pub fn write_indices(&mut self, index: usize, indices: &[u32]) {
        let vo = self.vertex_offset(index);
        if let Some((_dst_vertices, dst_indices)) = self.get_mut_slice(index) {
            for (src, dst) in indices.iter().zip(dst_indices.iter_mut()) {
                *dst = src + vo;
            }
        }
    }

    /// Write the vertices and indices into GPU memory using a staging belt.
    pub fn write_buffer(
        &mut self,
        staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
    ) {
        if self.is_empty() || !self.dirty {
            return;
        }
        self.dirty = false;

        // Vertices
        {
            let size = self.vertices_allocated as u64 * std::mem::size_of::<Vertex>() as u64;
            let mut buf_view = staging_belt.write_buffer(
                &mut *encoder,
                &self.vertex_buffer,
                0,
                wgpu::BufferSize::new(size).unwrap(),
                &device,
            );

            buf_view.copy_from_slice(bytemuck::cast_slice(&self.vertices));
        }

        // Indices
        {
            let size = self.indices_allocated as u64 * 4;
            let mut buf_view = staging_belt.write_buffer(
                &mut *encoder,
                &self.index_buffer,
                0,
                wgpu::BufferSize::new(size).unwrap(),
                &device,
            );

            buf_view.copy_from_slice(bytemuck::cast_slice(&self.indices));
        }
    }
}
