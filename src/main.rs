mod physics;
mod api;

use ggez::{conf, Context, ContextBuilder, GameError, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, Axis, Button, ErrorOrigin, EventHandler, GamepadId, MouseButton};
use ggez::event::winit_event::TouchPhase;
use ggez::input::keyboard::KeyInput;


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("CogLabs", "Cool Game Author")
        .window_mode(
            conf::WindowMode::default()
                //maximize by default
                .maximized(true)
                //keep window invisible until its fully configured
                .visible(false)
        )
        .window_setup(
            conf::WindowSetup::default()
                //window title
                .title("CogLabs")
        )
        .build()
        .expect("Could not create window graphics context!");

    //show window to monitor
    ctx.gfx.window().set_visible(true);
    //focus window
    ctx.gfx.window().focus_window();

    let my_game = CogLabs::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}
struct CogLabs {
    // Your state here...
}

impl CogLabs {
    pub fn new(_ctx: &mut Context) -> CogLabs {
        // Load/create resources such as images here.
        CogLabs {
            // ...
        }
    }

    pub fn tick(&self) {

    }
}

impl EventHandler for CogLabs {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.tick();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
        todo!()
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
        todo!()
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) -> Result<(), GameError> {
        todo!()
    }

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool) -> Result<(), GameError> {
        todo!()
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> Result<(), GameError> {
        todo!()
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        todo!()
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput) -> Result<(), GameError> {
        todo!()
    }

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> Result<(), GameError> {
        todo!()
    }

    fn touch_event(&mut self, ctx: &mut Context, phase: TouchPhase, x: f64, y: f64) -> Result<(), GameError> {
        todo!()
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) -> Result<(), GameError> {
        todo!()
    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) -> Result<(), GameError> {
        todo!()
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, _axis: Axis, _value: f32, _id: GamepadId) -> Result<(), GameError> {
        todo!()
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) -> Result<(), GameError> {
        todo!()
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        todo!()
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> Result<(), GameError> {
        todo!()
    }

    fn on_error(&mut self, _ctx: &mut Context, _origin: ErrorOrigin, _e: GameError) -> bool {
        todo!()
    }
}