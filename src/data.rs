use crate::rendering::Camera;
use bytemuck::{Pod, Zeroable};
use glam::Mat4;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex2 {
    pub position: [f32; 3],
    pub attrib: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex3 {
    pub position: [f32; 3],
    pub attrib: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    model_view_proj: Mat4,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            model_view_proj: Mat4::IDENTITY,
        }
    }
}

impl Uniforms {
    pub fn update_model_view_proj(&mut self, camera: &Camera, model_transformation: Mat4) {
        self.model_view_proj = camera.build_view_projection_matrix(model_transformation);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CanvasShaderUniforms {
    pub step_size: f32,
    pub base_distance: f32,
    pub opacity_threshold: f32,
    pub ambient_intensity: f32,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub shininess: f32,
}

impl Default for CanvasShaderUniforms {
    fn default() -> Self {
        Self {
            step_size: 0.0025,
            base_distance: 0.0025,
            opacity_threshold: 0.95,
            ambient_intensity: 0.5,
            diffuse_intensity: 0.5,
            specular_intensity: 0.5,
            shininess: 32.0,
        }
    }
}
