mod environment_map;
pub mod frame_buffer;
mod immediate_renderer;
pub mod lights;
mod mesh;
pub mod pipeline;
mod preloaded_renderer;
mod quad_renderer;
mod textures;
mod vertex;
mod virtual_gpu_callback;
pub mod virtual_render_pass;

use std::sync::Arc;

use environment_map::ENVIRONMENT_MAP_BIND_GROUP;
use nethercade_core::Resolution;
use pipeline::Pipeline;
use textures::DepthTexture;
pub use virtual_gpu_callback::*;

pub const IMMEDIATE_BIND_GROUP_INDEX: u32 = 0;
pub const TEXTURE_BIND_GROUP_INDEX: u32 = 1;
pub const LIGHT_BIND_GROUP_INDEX: u32 = 2;

pub const VERTEX_BUFFER_INDEX: u32 = 0;
pub const INSTANCE_BUFFER_INDEX: u32 = 1;

use eframe::wgpu;
use virtual_render_pass::VirtualRenderPass;

pub struct VirtualGpu {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,

    pub render_pipelines: [wgpu::RenderPipeline; 6],
    pub lights: lights::Lights,
    pub environment_map: environment_map::EnvironmentMap,
    pub instance_buffer: wgpu::Buffer,
    pub frame_buffer: Arc<frame_buffer::FrameBuffer>,

    pub immediate_renderer: immediate_renderer::ImmediateRenderer,
    pub preloaded_renderer: preloaded_renderer::PreloadedRenderer,
    pub quad_renderer: quad_renderer::QuadRenderer,
    pub textures: textures::Textures,
}

impl VirtualGpu {
    pub fn new(
        resolution: Resolution,
        device: &Arc<wgpu::Device>,
        queue: &Arc<wgpu::Queue>,
        format: wgpu::TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Master Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let mut textures = textures::Textures::new(device, resolution);
        let lights = lights::Lights::new(device);
        let environment_map = environment_map::EnvironmentMap::new(device, queue);
        let immediate_renderer = immediate_renderer::ImmediateRenderer::new(device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &immediate_renderer.bind_group_layout,
                    &textures.bind_group_layout,
                    &lights.bind_group_layout,
                    &environment_map.bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        textures.load_texture_native(
            device,
            queue,
            "nethercade_console/assets/default texture.png",
        );

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            device: device.clone(),
            queue: queue.clone(),
            render_pipelines: generate_render_pipelines(
                device,
                &shader,
                &render_pipeline_layout,
                format,
            ),
            textures,
            quad_renderer: quad_renderer::QuadRenderer::new(device, queue),
            preloaded_renderer: preloaded_renderer::PreloadedRenderer::new(),
            immediate_renderer,
            lights,
            instance_buffer,
            environment_map,
            frame_buffer: Arc::new(frame_buffer::FrameBuffer::new(device, resolution, format)),
        }
    }

    pub fn render(&mut self, vrp: &VirtualRenderPass) {
        let view = &self.frame_buffer.view;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Render Encoder"),
            });

        // Game Render Pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.textures.depth_texture.borrow().view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(f32::NEG_INFINITY),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_bind_group(
                IMMEDIATE_BIND_GROUP_INDEX,
                &self.immediate_renderer.bind_group,
                &[],
            );
            render_pass.set_bind_group(LIGHT_BIND_GROUP_INDEX, &self.lights.bind_group, &[]);
            render_pass.set_bind_group(
                ENVIRONMENT_MAP_BIND_GROUP,
                &self.environment_map.bind_group,
                &[],
            );
            self.queue.write_buffer(
                &self.environment_map.uniforms_buffer,
                0,
                bytemuck::cast_slice(&self.environment_map.uniforms.get_uniforms()),
            );
            render_pass.set_vertex_buffer(INSTANCE_BUFFER_INDEX, self.instance_buffer.slice(..));

            vrp.execute(&mut render_pass, self);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn resize(&self, resolution: Resolution) {
        *self.textures.depth_texture.borrow_mut() =
            DepthTexture::create_depth_texture(&self.device, resolution);
    }

    pub fn load_static_mesh(&mut self, data: &[f32], pipeline: Pipeline) -> usize {
        self.preloaded_renderer
            .load_static_mesh(&self.device, &self.queue, data, pipeline)
    }
    pub fn load_static_mesh_indexed(
        &mut self,
        data: &[f32],
        indices: &[u16],
        pipeline: Pipeline,
    ) -> usize {
        self.preloaded_renderer.load_static_mesh_indexed(
            &self.device,
            &self.queue,
            data,
            indices,
            pipeline,
        )
    }
    pub fn load_texture_raw(&mut self, data: &[u8], has_alpha: bool) -> usize {
        self.textures
            .load_texture_raw(&self.device, &self.queue, data, has_alpha)
    }
}

fn generate_render_pipelines(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
) -> [wgpu::RenderPipeline; 6] {
    use pipeline::Pipeline;
    const PIPELINES: [Pipeline; 6] = [
        Pipeline::Color,
        Pipeline::Uv,
        Pipeline::ColorUv,
        Pipeline::ColorLit,
        Pipeline::UvLit,
        Pipeline::ColorUvLit,
        // TODO: Put this back later
        // Pipeline::Quad2d,
    ];

    std::array::from_fn(|i| {
        let pipeline = PIPELINES[i];

        create_render_pipeline(device, shader, layout, format, pipeline)
    })
}

fn create_render_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    pipeline: pipeline::Pipeline,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(pipeline.name()),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(pipeline.vertex_shader()),
            buffers: &pipeline.get_pipeline_buffers(),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(pipeline.fragment_shader()),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: textures::DepthTexture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::GreaterEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
