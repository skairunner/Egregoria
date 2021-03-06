use crate::engine::{AudioContext, GfxContext, InputContext};
use crate::game_loop;
use futures::executor;
use wgpu::{Color, SwapChainOutput};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[allow(dead_code)]
pub struct Context {
    pub gfx: GfxContext,
    pub input: InputContext,
    pub audio: AudioContext,
    pub el: Option<EventLoop<()>>,
}

impl Context {
    pub fn new() -> Self {
        let el = EventLoop::new();

        let size = el.primary_monitor().size();

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(
                size.width as f32 * 0.8,
                size.height as f32 * 0.8,
            ))
            .with_title("Egregoria 0.1")
            .build(&el)
            .expect("Failed to create window");

        let gfx = executor::block_on(GfxContext::new(window));
        let input = InputContext::default();
        let audio = AudioContext::new();

        Self {
            gfx,
            input,
            audio,
            el: Some(el),
        }
    }

    pub fn start(mut self, mut state: game_loop::State) {
        let clear_color = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };

        let mut frame: Option<SwapChainOutput> = None;
        let mut new_size: Option<PhysicalSize<u32>> = None;

        self.el.take().unwrap().run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            state.event(&self.gfx, &event);
            match event {
                Event::WindowEvent { event, .. } => {
                    let managed = self.input.handle(&event);

                    if !managed {
                        match event {
                            WindowEvent::Resized(physical_size) => {
                                new_size = Some(physical_size);
                            }
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            _ => (),
                        }
                    }
                }
                Event::MainEventsCleared => match frame.take() {
                    None => {
                        if let Some(new_size) = new_size.take() {
                            self.gfx.resize(new_size);
                            state.resized(&mut self, new_size);
                        }
                        frame = Some(
                            self.gfx
                                .swapchain
                                .get_next_texture()
                                .expect("Timeout getting texture"),
                        );
                    }
                    Some(sco) => {
                        self.input.mouse.unprojected = state.unproject(self.input.mouse.screen);

                        state.update(&mut self);

                        self.gfx.render_frame(&mut state, &clear_color, sco);

                        self.input.end_frame();
                    }
                },
                _ => (),
            }
        })
    }
}
