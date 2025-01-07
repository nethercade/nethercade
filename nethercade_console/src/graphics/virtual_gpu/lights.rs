use bytemuck::{Pod, Zeroable};
use eframe::wgpu;
use glam::Vec4;

pub type LightUniformType = [f32; 12];
pub const MAX_LIGHTS: u64 = 4;

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Light {
    pub color_max_angle: Vec4,
    pub position_range: Vec4,
    pub direction_min_angle: Vec4,
}

impl Light {
    pub fn get_light_uniforms(&self) -> LightUniformType {
        bytemuck::cast(*self)
    }
}

pub struct Lights {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Lights {
    pub fn new(device: &wgpu::Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("light_bind_group_layout"),
        });

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("LightBuffer"),
            size: size_of::<Light>() as u64 * MAX_LIGHTS,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("light_bind_group"),
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }
}
