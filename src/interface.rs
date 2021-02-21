use crate::error::*;
use crate::conf::*;

pub struct InterfaceBuilder {
    pub(crate) game_name: String,
    pub(crate) author: String,
    pub(crate) configuration: Conf,
}

impl InterfaceBuilder {
    pub fn new(game_name: &str, author: &str) -> Self {
        let configuration = Conf::new(game_name);
        Self {
            game_name: game_name.to_string(),
            author: author.to_string(),
            configuration: configuration,
        }
    }

    pub fn build(self) -> GameResult<(Interface, winit::event_loop::EventLoop<()>)> {
        Interface::from_conf(self.configuration)
    }

    pub fn window_setup(mut self, setup: WindowSetup) -> Self {
        self.configuration.window_setup = setup;
        self
    }

    pub fn window_mode(mut self, mode: WindowMode) -> Self {
        self.configuration.window_mode = mode;
        self
    }
}

pub struct Interface {
    pub(crate) graphics_context: crate::graphics::context::GraphicsContext,
    pub(crate) keyboard_context: crate::input::keyboard::KeyboardInterface, 
}

impl Interface {
    pub fn from_conf(instance_conf: Conf) -> GameResult<(Self, winit::event_loop::EventLoop<()>)> {
        let event_loop = winit::event_loop::EventLoop::new();
        let interface_ctx = Interface {
            graphics_context: crate::graphics::context::GraphicsContext::new(&event_loop, instance_conf),
            keyboard_context: crate::input::keyboard::KeyboardInterface::new(),
        };

        Ok((interface_ctx, event_loop))
    }

    pub fn process_event(&mut self, event: &winit::event::Event<()>) {
        match event {
            // Window events.
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {

                },
                winit::event::WindowEvent::CursorMoved {
                    position: position,
                    ..
                } => {

                },
                winit::event::WindowEvent::MouseInput { button, state, ..} => {

                },
                winit::event::WindowEvent::ModifiersChanged(mods) => {

                },
                winit::event::WindowEvent::KeyboardInput {
                    input: winit::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                    ..
                } => {
                    let pressed = match state {
                        winit::event::ElementState::Pressed => true,
                        winit::event::ElementState::Released => false,
                    };
                    self.keyboard_context.set_key(*keycode, pressed);
                    println!("{:?} {}", keycode, pressed);
                },
                _ => {}
            },
            // Device events.
            winit::event::Event::DeviceEvent { 
                event: winit::event::DeviceEvent::MouseMotion { delta: (x, y) },
                ..
            } => {},
            // Others.
            _ => {}
        }
    }
}