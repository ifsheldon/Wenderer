use crate::data::{Vertex2, Vertex3};
use crate::rendering::{Geometry, OPENGL_TO_WGPU_MATRIX};
use cgmath::{Matrix4, Vector2, Vector3, Vector4};
use rayon::prelude::*;
use std::mem::size_of;
use wgpu::{
    BufferAddress, IndexFormat, InputStepMode, VertexAttribute, VertexBufferLayout, VertexFormat,
};

const DEFAULT_VERTEX_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: size_of::<Vertex2>() as BufferAddress,
    step_mode: InputStepMode::Vertex,
    attributes: &[
        VertexAttribute {
            offset: 0,
            shader_location: 0, // corresponds to layout(location = 0) in shader
            format: VertexFormat::Float32x3,
        },
        VertexAttribute {
            offset: size_of::<[f32; 3]>() as BufferAddress,
            shader_location: 1,
            format: VertexFormat::Float32x2,
        },
    ],
};

pub type V2 = Vector2<f32>;
pub type V3 = Vector3<f32>;
pub type V4 = Vector4<f32>;
pub type Mat4 = Matrix4<f32>;

pub struct Mesh2 {
    vertices: Vec<Vertex2>,
    indices_u16: Vec<u16>,
    indices_u32: Vec<u32>,
    index_format: IndexFormat,
    index_length: usize,
}

impl Mesh2 {
    pub fn new(
        vertices: &Vec<V3>,
        indices: &Vec<usize>,
        attribs_2D: &Vec<V2>,
        transform_matrix: Option<Mat4>,
    ) -> Self {
        assert_eq!(vertices.len(), attribs_2D.len());
        let transform_matrix = if let Some(transform_mat) = transform_matrix {
            transform_mat * OPENGL_TO_WGPU_MATRIX
        } else {
            OPENGL_TO_WGPU_MATRIX
        };
        let vertices: Vec<Vertex2> = vertices
            .par_iter()
            .map(|v| {
                let v = transform_matrix * V4::new(v.x, v.y, v.z, 1.0);
                v.xyz() / v.w
            })
            .zip(attribs_2D)
            .map(|(v, a)| Vertex2 {
                position: [v.x, v.y, v.z],
                attrib: [a.x, a.y],
            })
            .collect();
        let index_length = indices.len();
        if vertices.len() <= u16::MAX as usize {
            let indices = indices.iter().map(|x| *x as u16).collect();
            return Self {
                vertices: vertices.clone(),
                indices_u32: vec![],
                indices_u16: indices,
                index_format: IndexFormat::Uint16,
                index_length,
            };
        } else {
            let indices = indices.iter().map(|x| *x as u32).collect();
            return Self {
                vertices: vertices.clone(),
                indices_u32: indices,
                indices_u16: vec![],
                index_format: IndexFormat::Uint32,
                index_length,
            };
        }
    }
}

impl Geometry for Mesh2 {
    fn vertex_desc(&self) -> VertexBufferLayout {
        DEFAULT_VERTEX_LAYOUT
    }

    fn get_vertex_raw(&self) -> &[u8] {
        bytemuck::cast_slice(self.vertices.as_slice())
    }

    fn get_index_raw(&self) -> &[u8] {
        match self.index_format {
            IndexFormat::Uint16 => bytemuck::cast_slice(self.indices_u16.as_slice()),
            IndexFormat::Uint32 => bytemuck::cast_slice(self.indices_u32.as_slice()),
        }
    }

    #[inline]
    fn get_index_format(&self) -> IndexFormat {
        self.index_format
    }

    #[inline]
    fn get_num_indices(&self) -> usize {
        self.index_length
    }
}

pub struct Mesh3 {
    vertices: Vec<Vertex3>,
    indices_u32: Vec<u32>,
    indices_u16: Vec<u16>,
    index_format: IndexFormat,
    index_length: usize,
}

impl Mesh3 {
    pub fn new(
        vertices: &Vec<V3>,
        indices: &Vec<usize>,
        attribs_3D: &Vec<V3>,
        transform_matrix: Option<Mat4>,
    ) -> Self {
        assert_eq!(vertices.len(), attribs_3D.len());
        let transform_matrix = if let Some(transform_mat) = transform_matrix {
            transform_mat * OPENGL_TO_WGPU_MATRIX
        } else {
            OPENGL_TO_WGPU_MATRIX
        };
        let vertices: Vec<Vertex3> = vertices
            .par_iter()
            .map(|v| {
                let v = transform_matrix * V4::new(v.x, v.y, v.z, 1.0);
                v.xyz() / v.w
            })
            .zip(attribs_3D)
            .map(|(v, a)| Vertex3 {
                position: [v.x, v.y, v.z],
                attrib: [a.x, a.y, a.z],
            })
            .collect();
        let index_length = indices.len();
        if vertices.len() <= u16::MAX as usize {
            let indices = indices.iter().map(|x| *x as u16).collect();
            return Self {
                vertices: vertices.clone(),
                indices_u32: vec![],
                indices_u16: indices,
                index_format: IndexFormat::Uint16,
                index_length,
            };
        } else {
            let indices = indices.iter().map(|x| *x as u32).collect();
            return Self {
                vertices: vertices.clone(),
                indices_u32: indices,
                indices_u16: vec![],
                index_format: IndexFormat::Uint32,
                index_length,
            };
        }
    }
}

impl Geometry for Mesh3 {
    fn vertex_desc(&self) -> VertexBufferLayout {
        VertexBufferLayout {
            array_stride: size_of::<Vertex3>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0, // corresponds to layout(location = 0) in shader
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }

    fn get_vertex_raw(&self) -> &[u8] {
        bytemuck::cast_slice(self.vertices.as_slice())
    }

    fn get_index_raw(&self) -> &[u8] {
        match self.index_format {
            IndexFormat::Uint16 => bytemuck::cast_slice(self.indices_u16.as_slice()),
            IndexFormat::Uint32 => bytemuck::cast_slice(self.indices_u32.as_slice()),
        }
    }

    #[inline]
    fn get_index_format(&self) -> IndexFormat {
        self.index_format
    }

    #[inline]
    fn get_num_indices(&self) -> usize {
        self.index_length
    }
}

pub struct Pentagon;

impl Pentagon {
    const VERTICES: &'static [Vertex2] = &[
        Vertex2 {
            position: [-0.0868241, 0.49240386, 0.0],
            attrib: [0.4131759, 0.00759614],
        },
        Vertex2 {
            position: [-0.49513406, 0.06958647, 0.0],
            attrib: [0.0048659444, 0.43041354],
        },
        Vertex2 {
            position: [-0.21918549, -0.44939706, 0.0],
            attrib: [0.28081453, 0.949397057],
        },
        Vertex2 {
            position: [0.35966998, -0.3473291, 0.0],
            attrib: [0.85967, 0.84732911],
        },
        Vertex2 {
            position: [0.44147372, 0.2347359, 0.0],
            attrib: [0.9414737, 0.2652641],
        },
    ];

    const INDICES: &'static [u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
}

impl Geometry for Pentagon {
    fn vertex_desc(&self) -> VertexBufferLayout {
        DEFAULT_VERTEX_LAYOUT
    }

    fn get_vertex_raw(&self) -> &[u8] {
        bytemuck::cast_slice(Self::VERTICES)
    }

    fn get_index_raw(&self) -> &[u8] {
        bytemuck::cast_slice(Self::INDICES)
    }

    fn get_index_format(&self) -> IndexFormat {
        IndexFormat::Uint16
    }

    fn get_num_indices(&self) -> usize {
        Self::INDICES.len()
    }
}

pub struct Rectangle {
    mesh: Mesh2,
}

impl Rectangle {
    const INDICES: &'static [usize] = &[0, 1, 2, 0, 2, 3];

    pub fn new_standard_rectangle() -> Self {
        let pos = vec![
            V3::new(-1.0, -1.0, 0.0),
            V3::new(1.0, -1.0, 0.0),
            V3::new(1.0, 1.0, 0.0),
            V3::new(-1.0, 1.0, 0.0),
        ];
        let attribs = vec![
            V2::new(0.0, 1.0),
            V2::new(1.0, 1.0),
            V2::new(1.0, 0.0),
            V2::new(0.0, 0.0),
        ];
        let indices = Self::INDICES.to_vec();
        Self {
            mesh: Mesh2::new(&pos, &indices, &attribs, None),
        }
    }

    pub fn new_unit_rectangle() -> Self {
        let pos = vec![
            V3::new(0.0, 0.0, 0.0),
            V3::new(1.0, 0.0, 0.0),
            V3::new(1.0, 1.0, 0.0),
            V3::new(0.0, 1.0, 0.0),
        ];
        let attribs = vec![
            V2::new(0.0, 1.0),
            V2::new(1.0, 1.0),
            V2::new(1.0, 0.0),
            V2::new(0.0, 0.0),
        ];
        let indices = Self::INDICES.to_vec();
        Self {
            mesh: Mesh2::new(&pos, &indices, &attribs, None),
        }
    }
}

impl Geometry for Rectangle {
    fn vertex_desc(&self) -> VertexBufferLayout {
        self.mesh.vertex_desc()
    }

    fn get_vertex_raw(&self) -> &[u8] {
        self.mesh.get_vertex_raw()
    }

    fn get_index_raw(&self) -> &[u8] {
        self.mesh.get_index_raw()
    }

    fn get_index_format(&self) -> IndexFormat {
        self.mesh.get_index_format()
    }

    fn get_num_indices(&self) -> usize {
        self.mesh.get_num_indices()
    }
}

pub struct Cube;

impl Cube {
    const SIDE: f32 = 0.5;
    const VERTICES: &'static [Vertex2] = &[
        // 4 vertices on z = 0.5
        Vertex2 {
            position: [-Self::SIDE, -Self::SIDE, Self::SIDE],
            attrib: [0.0, 0.0],
        },
        Vertex2 {
            position: [Self::SIDE, -Self::SIDE, Self::SIDE],
            attrib: [0.0, 1.0],
        },
        Vertex2 {
            position: [Self::SIDE, Self::SIDE, Self::SIDE],
            attrib: [1.0, 0.0],
        },
        Vertex2 {
            position: [-Self::SIDE, Self::SIDE, Self::SIDE],
            attrib: [1.0, 1.0],
        },
        // 4 vertices on z = -0.5
        Vertex2 {
            position: [-Self::SIDE, -Self::SIDE, -Self::SIDE],
            attrib: [0.0, 0.0],
        },
        Vertex2 {
            position: [Self::SIDE, -Self::SIDE, -Self::SIDE],
            attrib: [0.0, 1.0],
        },
        Vertex2 {
            position: [Self::SIDE, Self::SIDE, -Self::SIDE],
            attrib: [1.0, 0.0],
        },
        Vertex2 {
            position: [-Self::SIDE, Self::SIDE, -Self::SIDE],
            attrib: [1.0, 1.0],
        },
    ];

    #[rustfmt::skip]
    const INDICES: &'static [u16] = &[
        0, 1, 3, 3, 1, 2,
        2, 1, 5, 2, 5, 6,
        3, 2, 7, 7, 2, 6,
        4, 0, 3, 4, 3, 7,
        4, 1, 0, 4, 5, 1,
        7, 6, 5, 7, 5, 4
    ];
}

impl Geometry for Cube {
    fn vertex_desc(&self) -> VertexBufferLayout {
        DEFAULT_VERTEX_LAYOUT
    }

    fn get_vertex_raw(&self) -> &[u8] {
        bytemuck::cast_slice(Self::VERTICES)
    }

    fn get_index_raw(&self) -> &[u8] {
        bytemuck::cast_slice(Self::INDICES)
    }

    fn get_index_format(&self) -> IndexFormat {
        IndexFormat::Uint16
    }

    fn get_num_indices(&self) -> usize {
        Self::INDICES.len()
    }
}
