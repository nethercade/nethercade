use eframe::wgpu;

use super::{mesh, textures};

// TODO: Could do something for immediate Textures

pub struct ImmediateRenderer {
    pub vertex_buffer: wgpu::Buffer,

    // Instance Data
    pub model_buffer: wgpu::Buffer,
    pub view_buffer: wgpu::Buffer,
    pub proj_buffer: wgpu::Buffer,
    pub camera_pos_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,

    pub texture_sampler: wgpu::Sampler,
    pub matcap_sampler: wgpu::Sampler,
}

impl ImmediateRenderer {
    pub fn new(device: &wgpu::Device) -> Self {
        let vertex_buffer = device.create_buffer(&mesh::vertex_buffer_descriptor(
            1024 * 1024 * 8, // 8mb
            Some("Immediate Vertex Buffer"),
        ));

        let model_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Model Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("View Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let proj_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Projection Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let position_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Position Buffer"),
            size: 8 * 1024 * 1024, // 8mb
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer_type = wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        };

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // Model
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: buffer_type,
                    count: None,
                },
                // Views
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: buffer_type,
                    count: None,
                },
                // Positions
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: buffer_type,
                    count: None,
                },
                // Projections
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: buffer_type,
                    count: None,
                },
                // Texture Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
                // Matcap Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("instance data bind group layout"),
        });

        let texture_sampler = device.create_sampler(&textures::texture_sampler_descriptor());
        let matcap_sampler = device.create_sampler(&textures::matcap_sampler_descriptor());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: view_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: proj_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: position_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: position_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: position_buffer.as_entire_binding(),
                },
            ],
            label: Some("instance data bind group"),
        });

        Self {
            vertex_buffer,
            model_buffer,
            view_buffer,
            proj_buffer,
            camera_pos_buffer: position_buffer,
            bind_group,
            bind_group_layout,
            texture_sampler,
            matcap_sampler,
        }
    }
}
