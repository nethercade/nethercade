use eframe::wgpu;
use glam::Vec4;
use image::ImageReader;

pub const ENVIRONMENT_MAP_BIND_GROUP: u32 = 3;

pub struct EnvironmentMap {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniforms_buffer: wgpu::Buffer,
    pub uniforms: EnvironmentUniforms,
}

pub struct EnvironmentUniforms {
    pub environment_color_strength: Vec4,
}

impl EnvironmentUniforms {
    pub fn new() -> Self {
        Self {
            environment_color_strength: Vec4::ONE,
        }
    }

    pub fn get_uniforms(&self) -> [f32; 4] {
        self.environment_color_strength.into()
    }
}

// Right
// Left
// Top
// Bottom
// Front
// Back

impl EnvironmentMap {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        const IMAGES: [&str; 6] = [
            "nethercade_console/assets/skybox/right.png",
            "nethercade_console/assets/skybox/left.png",
            "nethercade_console/assets/skybox/top.png",
            "nethercade_console/assets/skybox/bottom.png",
            "nethercade_console/assets/skybox/front.png",
            "nethercade_console/assets/skybox/back.png",
        ];

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Environment Map Texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                // A cube has 6 sides, so we need 6 layers
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Environment Map View"),
            dimension: Some(wgpu::TextureViewDimension::Cube),
            array_layer_count: Some(6),
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Environment Map Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let uniforms_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Environment Map Uniforms Buffer"),
            size: size_of::<EnvironmentUniforms>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(bind_group_layout_desc());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniforms_buffer.as_entire_binding(),
                },
            ],
            label: Some("Environment Map Bind Group"),
        });

        let uniforms = EnvironmentUniforms::new();

        for (index, path) in IMAGES.iter().enumerate() {
            let image = ImageReader::open(path).unwrap().decode().unwrap();
            let image = image.to_rgba8();
            let dimensions = image.dimensions();
            let size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: index as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &image,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                size,
            );
        }

        Self {
            bind_group,
            bind_group_layout,
            uniforms_buffer,
            uniforms,
        }
    }
}

pub const fn bind_group_layout_desc() -> &'static wgpu::BindGroupLayoutDescriptor<'static> {
    &wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
        label: Some("environment map bind group layout"),
    }
}
