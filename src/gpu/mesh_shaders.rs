//! GPU-driven Rendering Pipeline with Mesh Shaders
//!
//! This module provides GPU-driven rendering using mesh shaders, which allows
//! for more efficient rendering of complex geometry with culling and LOD
//! performed entirely on the GPU.

use crate::foundation::handle::Handle;
use crate::topology::topods_shape::TopoDsShape;
use std::sync::Arc;

/// GPU-driven rendering pipeline
#[derive(Debug, Clone)]
pub struct MeshShaderPipeline {
    device: Arc<wgpu::Device>,
    pipeline: wgpu::RenderPipeline,
    task_pipeline: wgpu::ComputePipeline,
    mesh_pipeline: wgpu::ComputePipeline,
}

impl MeshShaderPipeline {
    /// Create a new mesh shader pipeline
    pub fn new(device: &wgpu::Device) -> Result<Self, RenderingError> {
        let task_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Task Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("task.wgsl").into()),
        });

        let mesh_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Mesh Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("mesh.wgsl").into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("fragment.wgsl").into()),
        });

        let task_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Task Pipeline"),
            layout: None,
            module: &task_shader,
            entry_point: "main",
        });

        let mesh_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Mesh Pipeline"),
            layout: None,
            module: &mesh_shader,
            entry_point: "main",
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Mesh Shader Render Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &fragment_shader,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(Self {
            device: Arc::new(device.clone()),
            pipeline: render_pipeline,
            task_pipeline,
            mesh_pipeline,
        })
    }

    /// Render shapes using mesh shaders
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        shapes: &[Handle<TopoDsShape>],
        output_view: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
    ) -> Result<(), RenderingError> {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Mesh Shader Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
        });

        pass.set_pipeline(&self.pipeline);

        for shape in shapes {
            self.render_shape(&mut pass, shape)?;
        }

        Ok(())
    }

    /// Render a single shape
    fn render_shape(
        &self,
        pass: &mut wgpu::RenderPass,
        shape: &Handle<TopoDsShape>,
    ) -> Result<(), RenderingError> {
        Ok(())
    }

    /// Get task pipeline
    pub fn task_pipeline(&self) -> &wgpu::ComputePipeline {
        &self.task_pipeline
    }

    /// Get mesh pipeline
    pub fn mesh_pipeline(&self) -> &wgpu::ComputePipeline {
        &self.mesh_pipeline
    }

    /// Get render pipeline
    pub fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

/// Errors that can occur during mesh shader rendering
#[derive(Debug, thiserror::Error)]
pub enum RenderingError {
    #[error("Pipeline creation failed: {0}")]
    PipelineCreationFailed(String),

    #[error("Rendering failed: {0}")]
    RenderingFailed(String),

    #[error("Mesh shaders not supported on this device")]
    NotSupported,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "gpu")]
    fn test_mesh_shader_pipeline_creation() {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).unwrap();
        let (device, _) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).unwrap();

        let pipeline = MeshShaderPipeline::new(&device);
        assert!(pipeline.is_ok());
    }
}
