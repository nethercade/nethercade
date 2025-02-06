use eframe::wgpu;
use nethercade_core::Resolution;
use wgpu::TextureViewDescriptor;

pub struct FrameBuffer {
    pub view: wgpu::TextureView,
}

impl FrameBuffer {
    pub fn new(device: &wgpu::Device, resolution: Resolution, format: wgpu::TextureFormat) -> Self {
        let (width, height) = resolution.dimensions();

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("Frame Buffer Texture"),
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        Self { view }
    }
}
