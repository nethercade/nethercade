use eframe::wgpu;

use super::vertex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Pipeline {
    Color,
    Uv,
    ColorUv,
    Quad2d,
    Matcap,
    MatcapColor,
    MatcapUv,
    MatcapColorUv,
}

impl TryFrom<i32> for Pipeline {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Color),
            1 => Ok(Self::Uv),
            2 => Ok(Self::ColorUv),
            3 => Ok(Self::Quad2d),
            4 => Ok(Self::Matcap),
            5 => Ok(Self::MatcapColor),
            6 => Ok(Self::MatcapUv),
            7 => Ok(Self::MatcapColorUv),
            _ => Err("Invalid Pipeline"),
        }
    }
}

impl Pipeline {
    pub fn name(&self) -> &'static str {
        match self {
            Pipeline::Color => "color",
            Pipeline::Uv => "uv",
            Pipeline::ColorUv => "color uv",
            Pipeline::Quad2d => "quad 2d",
            Pipeline::Matcap => "matcap",
            Pipeline::MatcapColor => "matcap color",
            Pipeline::MatcapUv => "matcap uv",
            Pipeline::MatcapColorUv => "matcap color uv",
        }
    }

    pub fn vertex_shader(&self) -> &'static str {
        match self {
            Pipeline::Color => "vs_color",
            Pipeline::Uv => "vs_uv",
            Pipeline::ColorUv => "vs_color_uv",
            Pipeline::Quad2d => "vs_quad_2d",
            Pipeline::Matcap => "vs_matcap",
            Pipeline::MatcapColor => "vs_matcap_color",
            Pipeline::MatcapUv => "vs_matcap_uv",
            Pipeline::MatcapColorUv => "vs_matcap_color_uv",
        }
    }

    pub fn fragment_shader(&self) -> &'static str {
        match self {
            Pipeline::Color => "fs_color",
            Pipeline::Uv | Pipeline::Quad2d => "fs_uv",
            Pipeline::ColorUv => "fs_color_uv",
            Pipeline::Matcap => "fs_matcap",
            Pipeline::MatcapColor => "fs_matcap_color",
            Pipeline::MatcapUv => "fs_matcap_uv",
            Pipeline::MatcapColorUv => "fs_matcap_color_uv",
        }
    }

    pub fn get_pipeline_buffers(&self) -> [wgpu::VertexBufferLayout<'static>; 2] {
        [
            self.get_vertex_buffer_layout(),
            vertex::instance_vertex_buffer_layout(),
        ]
    }

    pub fn get_vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout<'static> {
        match self {
            Pipeline::Color => vertex::color(),
            Pipeline::Uv => vertex::uv(),
            Pipeline::ColorUv => vertex::color_uv(),
            Pipeline::Quad2d => vertex::uv(),
            Pipeline::Matcap => vertex::matcap(),
            Pipeline::MatcapColor => vertex::matcap_color(),
            Pipeline::MatcapUv => vertex::matcap_uv(),
            Pipeline::MatcapColorUv => vertex::matcap_color_uv(),
        }
    }

    // pub fn can_reduce(&self, into: Self) -> bool {
    //     let color = !into.has_color() || self.has_color();
    //     let uv = !into.has_uv() || self.has_uv();
    //     let lighting = !into.has_lighting() || self.has_lighting();

    //     color && uv && lighting
    // }

    // pub fn has_color(&self) -> bool {
    //     match self {
    //         Pipeline::Color => true,
    //         Pipeline::Uv => false,
    //         Pipeline::ColorUv => true,
    //         Pipeline::Quad2d => true,
    //         Pipeline::Matcap => false,
    //         Pipeline::MatcapColor => true,
    //         Pipeline::MatcapUv => false,
    //         Pipeline::MatcapColorUv => true,
    //     }
    // }

    // pub fn has_uv(&self) -> bool {
    //     match self {
    //         Pipeline::Color => false,
    //         Pipeline::Uv => true,
    //         Pipeline::ColorUv => true,
    //         Pipeline::Quad2d => true,
    //         Pipeline::Matcap => false,
    //         Pipeline::MatcapColor => false,
    //         Pipeline::MatcapUv => true,
    //         Pipeline::MatcapColorUv => true,
    //     }
    // }

    // pub fn has_lighting(&self) -> bool {
    //     match self {
    //         Pipeline::Color => false,
    //         Pipeline::Uv => false,
    //         Pipeline::ColorUv => false,
    //         Pipeline::Quad2d => false,
    //         Pipeline::Matcap => false,
    //         Pipeline::MatcapColor => false,
    //         Pipeline::MatcapUv => false,
    //         Pipeline::MatcapColorUv => false,
    //     }
    // }

    // pub fn has_normals(&self) -> bool {
    //     match self {
    //         Pipeline::Color => false,
    //         Pipeline::Uv => false,
    //         Pipeline::ColorUv => false,
    //         Pipeline::Quad2d => false,
    //         Pipeline::Matcap => true,
    //         Pipeline::MatcapColor => true,
    //         Pipeline::MatcapUv => true,
    //         Pipeline::MatcapColorUv => true,
    //     }
    // }

    // pub fn matcap(&self) -> Self {
    //     match self {
    //         Pipeline::Color => Pipeline::MatcapColor,
    //         Pipeline::Uv => Pipeline::MatcapUv,
    //         Pipeline::ColorUv => Pipeline::MatcapColorUv,
    //         Pipeline::Quad2d => panic!("Quad2d can't be a matcap"),
    //         Pipeline::Matcap => *self,
    //         Pipeline::MatcapColor => *self,
    //         Pipeline::MatcapUv => *self,
    //         Pipeline::MatcapColorUv => *self,
    //     }
    // }

    pub fn get_shader(&self) -> usize {
        match self {
            Pipeline::Color => 0,
            Pipeline::Uv => 1,
            Pipeline::ColorUv => 2,
            Pipeline::Quad2d => 3,
            Pipeline::Matcap => 4,
            Pipeline::MatcapColor => 5,
            Pipeline::MatcapUv => 6,
            Pipeline::MatcapColorUv => 7,
        }
    }

    pub fn get_attribute_count(&self) -> usize {
        3 + match self {
            Pipeline::Color => 3,
            Pipeline::Uv => 2,
            Pipeline::ColorUv | Pipeline::Quad2d => 5,
            Pipeline::Matcap => 3,
            Pipeline::MatcapColor => 6,
            Pipeline::MatcapUv => 5,
            Pipeline::MatcapColorUv => 8,
        }
    }

    pub fn get_vertex_size(&self) -> usize {
        self.get_attribute_count() * 4
    }
}
