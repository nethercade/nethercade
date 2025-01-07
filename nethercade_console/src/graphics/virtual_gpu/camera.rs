use eframe::wgpu;
use glam::{Mat4, Vec3A};
use nethercade_core::Rom;

pub type CameraUniformType = [f32; 52];

pub struct Camera {
    pub eye: Vec3A,
    pub yaw: f32,
    up: Vec3A,
    aspect: f32,
    fovy: f32,
    z_near: f32,
    width: u32,
    height: u32,

    // Wgpu Stuff:
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn new(device: &wgpu::Device, rom: &Rom) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: size_of::<CameraUniformType>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let (width, height) = rom.resolution.dimensions();

        Self {
            eye: Vec3A::new(0.0, 1.0, 5.0),
            yaw: 0.0,
            up: Vec3A::Y,
            aspect: width as f32 / height as f32,
            fovy: 45.0,
            z_near: 0.1,
            width,
            height,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn get_forward(&self) -> Vec3A {
        Vec3A::new(self.yaw.sin(), 0.0, -self.yaw.cos())
    }

    pub fn get_view(&self) -> Mat4 {
        Mat4::look_to_rh(self.eye.into(), self.get_forward().into(), self.up.into())
    }

    fn get_projection_3d(&self) -> Mat4 {
        Mat4::perspective_infinite_reverse_rh(self.fovy.to_radians(), self.aspect, self.z_near)
    }

    pub fn get_camera_uniforms(&self) -> CameraUniformType {
        let mut out = [0.0; 52];

        let view = self.get_view();
        let projection = self.get_projection_3d();
        let ortho = self.get_projection_2d();

        view.write_cols_to_slice(&mut out[0..16]);
        projection.write_cols_to_slice(&mut out[16..32]);
        ortho.write_cols_to_slice(&mut out[32..48]);
        self.eye.write_to_slice(&mut out[48..52]);
        out
    }

    pub fn get_projection_2d(&self) -> Mat4 {
        Mat4::orthographic_rh(0.0, self.width as f32, self.height as f32, 0.0, 1.0, -1.0)
    }
}
