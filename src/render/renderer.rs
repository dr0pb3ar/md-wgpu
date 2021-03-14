use crate::{ DualBuffer, Vertex, Point2, Color};
use futures::task::SpawnExt;
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};
use winit::{event::*, window::Window};

const TRIANGLES_MAX_VERTICES: u64 = 100;
const TRIANGLES_MAX_INDICES: u64 = 320;

const LINES_MAX_VERTICES: u64 = 50;
const LINES_MAX_INDICES: u64 = 100;

/// Create a new render pipeline with shaders and primitive topology.
fn create_pipeline(
    device: &wgpu::Device,
    vs_module: &wgpu::ShaderModule,
    fs_module: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    primitive_topology: wgpu::PrimitiveTopology,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: fs_module,
            entry_point: "main",
        }),
        rasterization_state: None,
        primitive_topology,
        color_states: &[wgpu::ColorStateDescriptor {
            format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: &[Vertex::desc()],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    render_pipeline
}

struct RenderInProgress {
    frame: wgpu::SwapChainTexture,
    encoder: wgpu::CommandEncoder,
}

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    triangles_pipeline: wgpu::RenderPipeline,
    lines_pipeline: wgpu::RenderPipeline,
    pub triangles_buffer: DualBuffer,
    pub lines_buffer: DualBuffer,
    staging_belt: wgpu::util::StagingBelt,
    glyph_brush: GlyphBrush<(), ab_glyph::FontArc>,
    local_pool: futures::executor::LocalPool,
    local_spawner: futures::executor::LocalSpawner,
    rip: Option<RenderInProgress>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                // Request an adapter which can render to our surface.
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    //label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Used for writing data to GPU buffers.
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        // Async
        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();

        let format = wgpu::TextureFormat::Bgra8UnormSrgb;

        // Prepare glyph brush
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("UbuntuMono-R.ttf"))
            .expect("Failed to load font.");

        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(&device, format);

        // Swapchain
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Load shaders
        let vs_module = device.create_shader_module(wgpu::include_spirv!("shaders/shader.vert.spv"));
        let fs_module = device.create_shader_module(wgpu::include_spirv!("shaders/shader.frag.spv"));

        // Create render pipelines
        let triangles_pipeline = create_pipeline(
            &device,
            &vs_module,
            &fs_module,
            sc_desc.format,
            wgpu::PrimitiveTopology::TriangleList,
        );
        let lines_pipeline = create_pipeline(
            &device,
            &vs_module,
            &fs_module,
            sc_desc.format,
            wgpu::PrimitiveTopology::LineList,
        );

        // Create buffers for render pipelines
        let triangles_buffer = DualBuffer::new(
            &device,
            "Triangles",
            TRIANGLES_MAX_VERTICES,
            TRIANGLES_MAX_INDICES,
        );
        let lines_buffer = DualBuffer::new(&device, "Lines", LINES_MAX_VERTICES, LINES_MAX_INDICES);


        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            triangles_pipeline,
            lines_pipeline,
            triangles_buffer,
            lines_buffer,
            staging_belt,
            glyph_brush,
            local_pool,
            local_spawner,
            rip: None,
        }
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        //self.vertices[0].color[2] = 0.5;
        /*self.queue.write_buffer(
            &self.dual_buffer.vertex_buffer,
            0,
            bytemuck::cast_slice(&[self.vertices]),
        );*/
        /*self.queue.write_buffer(
            &self.triangles_buffer.vertex_buffer,
            0,
            bytemuck::cast_slice(&self.triangles_buffer.vertices)
            );

        self.queue.write_buffer(
            &self.triangles_buffer.index_buffer,
            0,
            bytemuck::cast_slice(&self.triangles_buffer.indices)
            );*/
    }

    pub fn render_start(&mut self) {
        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Failed to get current swap chain frame.")
            .output;

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.rip = Some(RenderInProgress { frame, encoder });
    }

    pub fn render_finish(&mut self) {
        if let Some(RenderInProgress { frame, mut encoder }) = self.rip.take() {
            self.triangles_buffer
                .write_buffer(&mut self.staging_belt, &mut encoder, &self.device);
            self.lines_buffer
                .write_buffer(&mut self.staging_belt, &mut encoder, &self.device);

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    //label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&self.triangles_pipeline);

                //if let Some(vertex_buffer, index_buffer) = self.dual_buffer.get_slice(0) {}
                render_pass.set_vertex_buffer(0, self.triangles_buffer.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.triangles_buffer.index_buffer.slice(..));
                //render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
                render_pass.draw_indexed(0..self.triangles_buffer.indices_len(), 0, 0..1);
                //render_pass.draw(0..3, 0..1);
            }

            if !self.lines_buffer.is_empty() {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&self.lines_pipeline);
                render_pass.set_vertex_buffer(0, self.lines_buffer.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.lines_buffer.index_buffer.slice(..));
                render_pass.draw_indexed(0..self.lines_buffer.indices_len(), 0, 0..1);
            }

            self.glyph_brush
                .draw_queued(
                    &self.device,
                    &mut self.staging_belt,
                    &mut encoder,
                    &frame.view,
                    self.size.width,
                    self.size.height,
                )
                .expect("Draw queued");

            self.staging_belt.finish();
            self.queue.submit(std::iter::once(encoder.finish()));

            // Recall unused staging buffers
            self.local_spawner
                .spawn(self.staging_belt.recall())
                .expect("Recall staging belt");
            self.local_pool.run_until_stalled();
        }
    }

    pub fn draw_text(&mut self, text: &str, position: Point2, color: Color ,scale: f32) {
        self.glyph_brush.queue(Section {
            screen_position: (position.x, position.y),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![Text::new(text)
                .with_color([color.r, color.g, color.b, 1.0])
                .with_scale(scale)],
            ..Section::default()
        });
    }
}
