pub mod test;
pub mod widget;
pub mod style;
pub mod event;
pub mod error;
pub mod utils;
pub mod renderer;


pub mod window {
    use winit::{event::*, event_loop::{EventLoop, EventLoopBuilder}, window::WindowBuilder, dpi::Position, platform::windows::EventLoopBuilderExtWindows};

    use wgpu::{
        util::DeviceExt, Backends, BindGroupDescriptor, BindGroupLayoutDescriptor, BindingResource,
        Instance, InstanceDescriptor, RequestAdapterOptions,
    };

    const SHADER: &str = r#"

    struct VertexInput {
        @location(0) position: vec3<f32>,
        @location(1) color: vec3<f32>,
    };
    
    struct VertexOutput {
        @builtin(position) clip_position: vec4<f32>,
        @location(0) color: vec3<f32>,
    };

    
    @group(0) @binding(0)
    var<uniform> window_size: vec2<f32>;

    @vertex
    fn vs_main(
        model: VertexInput,
    ) -> VertexOutput {

        var out: VertexOutput;
        
        var normalizationMatrix = mat2x2<f32>(
            2.0 / window_size.x, 0.0,
            0.0, -2.0 / window_size.y
        );
        
        var normalizedPosition = normalizationMatrix * model.position.xy;
        normalizedPosition.y += 1.0;
        normalizedPosition.x -= 1.0;

        out.clip_position = vec4<f32>(normalizedPosition, model.position.z, 1.0);

        out.color = model.color;

        return out;
    }

    
    // Fragment shader
    
    @fragment
    fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
        return vec4<f32>(in.color, 1.0);
    }

    "#;

    struct State {
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
        size: winit::dpi::PhysicalSize<u32>,
        window: winit::window::Window,
        render_pipeline: wgpu::RenderPipeline,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        num_vertices: u32,
        num_indices: u32,
        window_size_buffer: wgpu::Buffer,
        window_bind_group: wgpu::BindGroup,
        position: [f32; 2],
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f32; 3],
        color: [f32; 3],
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct WindowSize {
        size: [f32; 2],
    }

    unsafe impl bytemuck::Pod for Vertex {}
    unsafe impl bytemuck::Zeroable for Vertex {}

    unsafe impl bytemuck::Pod for WindowSize {}
    unsafe impl bytemuck::Zeroable for WindowSize {}

    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0],
        }, // Top-left (red) 0 
        Vertex {
            position: [0.0, 100.0, 0.0],
            color: [0.0, 1.0, 0.0],
        }, // Bottom-left (green) 1
        Vertex {
            position: [100.0, 100.0, 0.0],
            color: [0.0, 0.0, 1.0],
        }, // Bottom-right (blue) 2
        Vertex {
            position: [100.0, 0.0, 0.0],
            color: [1.0, 1.0, 0.0],
        }, // Top-right (yellow) 3
    ];

    const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

    impl Vertex {
        const ATTRIBS: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        fn desc() -> wgpu::VertexBufferLayout<'static> {
            use std::mem;

            wgpu::VertexBufferLayout {
                array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &Self::ATTRIBS,
            }
        }
    }

    impl WindowSize {
        const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![2 => Float32x2];

        fn desc() -> wgpu::VertexBufferLayout<'static> {
            use std::mem;

            wgpu::VertexBufferLayout {
                array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &Self::ATTRIBS,
            }
        }
    }

    impl State {
        fn update(&mut self) {}
        fn input(&mut self, event: &WindowEvent) -> bool {
            match event{
                WindowEvent::CursorMoved { device_id, position, modifiers } => {
                    self.window.request_redraw();
                    self.position = [position.x as f32 - 50.0, position.y as f32 - 50.0];
                    return true;
                }
                _ => return false,
            };
        }
        fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
            let output = self.surface.get_current_texture()?;
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
                
                render_pass.set_pipeline(&self.render_pipeline); 
                render_pass.set_bind_group(0, &self.window_bind_group, &[]);
                render_pass.set_viewport(
                    0.0,
                    0.0,
                    self.size.width as f32,
                    self.size.height as f32,
                    0.0,
                    1.0,
                );
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }
            
            let vertices = [
                Vertex {
                    position: [0.0 + self.position[0], 0.0 + self.position[1], 0.0],
                    color: [1.0, 0.0, 0.0],
                }, // Top-left (red) 0 
                Vertex {
                    position: [0.0 + self.position[0], 100.0 + self.position[1], 0.0],
                    color: [0.0, 1.0, 0.0],
                }, // Bottom-left (green) 1
                Vertex {
                    position: [100.0 + self.position[0], 100.0 + self.position[1], 0.0],
                    color: [0.0, 0.0, 1.0],
                }, // Bottom-right (blue) 2
                Vertex {
                    position: [100.0 + self.position[0], 0.0 + self.position[1], 0.0],
                    color: [1.0, 1.0, 0.0],
                },
            ];

            self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            
            self.queue.write_buffer(&self.window_size_buffer, 0, bytemuck::cast_slice(&[WindowSize{size: [self.size.width as f32, self.size.height as f32]}]));

            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();

            Ok(())
        }
        fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
            if new_size.width == 0 && new_size.height == 0 {
                return;
            }
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
        
        async fn new(window: winit::window::Window) -> Self {
            let size = window.inner_size();
            let num_vertices = VERTICES.len() as u32;
            
            let instance = Instance::new(InstanceDescriptor {
                backends: Backends::PRIMARY,
                ..InstanceDescriptor::default()
            });

            let surface = unsafe { instance.create_surface(&window) }.unwrap();

            let options = RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            };
            
            let adapter = instance.request_adapter(&options).await;

            let adapter = match adapter {
                Some(adapter) => adapter,
                None => instance
                    .request_adapter(&wgpu::RequestAdapterOptions::default())
                    .await
                    .expect("Failed to find any suitable adapter"),
                };
                let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        // Just in case I want wasm support later
                        limits: if cfg!(target_arch = "wasm32") {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        } else {
                            wgpu::Limits::default()
                        },
                        label: None,
                    },
                    None,
                )
                .await
                .unwrap();
            
            let surface_caps = surface.get_capabilities(&adapter);
            
            let surface_format = surface_caps
            .formats
                .iter()
                .copied()
                .filter(|f| f.is_srgb())
                .next()
                .unwrap_or(surface_caps.formats[0]);
            
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
            };
            
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER.into()),
            });
            
            
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
            
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
    
            let window_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Window Size Buffer"),
                contents: bytemuck::cast_slice(&[WindowSize {
                    size: [size.width as f32, size.height as f32],
                }]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
            
            let window_bind_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Bind Group Layout"),
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
                });
                
                let window_bind_group = device.create_bind_group(&BindGroupDescriptor {
                    label: Some("Bind Group"),
                    layout: &window_bind_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                            window_size_buffer.as_entire_buffer_binding(),
                        ),
                    }],
                });
                
                let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&window_bind_layout],
                    push_constant_ranges: &[],
                });

                let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
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
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });



            let num_indices = INDICES.len() as u32;
            let position = [0.0, 0.0];

            surface.configure(&device, &config);

            Self {
                window,
                surface,
                device,
                queue,
                config,
                size,
                render_pipeline,
                vertex_buffer,
                index_buffer,
                num_vertices,
                num_indices,
                window_size_buffer,
                window_bind_group,
                position
            }
        }
    }

    pub async fn run(title: &str) {
        env_logger::init();
        
        #[cfg(test)]
        let event_loop = EventLoop::from(EventLoopBuilder::new().with_any_thread(true).build());

        #[cfg(not(test))]
        let event_loop = EventLoop::new();


        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .expect("Window could not be created");

        let mut state = State::new(window).await;

        let _ = event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(window_id) if window_id == state.window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // redraw loop
                // state.window.request_redraw();
            }
            Event::WindowEvent { window_id, event } if window_id == state.window.id() => {
                if !state.input(&event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        } => {
                            println!("Closed window!");
                            control_flow.set_exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(*new_inner_size);
                        }
                        _ => {}
                    };
                };
            }
            _ => (),
        });
    }
}