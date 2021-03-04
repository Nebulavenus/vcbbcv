#![feature(min_specialization)]

use glsp::prelude::*;
use macroquad::prelude::*;

// draw wrappers

fn clear_background_wrapper(color: [u8; 4]) -> GResult<()> {
    let c = color_u8!(color[0], color[1], color[2], color[3]);
    clear_background(c);
    Ok(())
}

fn draw_circle_wrapper(x: f32, y: f32, r: f32, color: [u8; 4]) -> GResult<()> {
    let c = color_u8!(color[0], color[1], color[2], color[3]);
    draw_circle(x, y, r, c);
    Ok(())
}

fn draw_triangle_lines_wrapper(
    v1: [f32; 2],
    v2: [f32; 2],
    v3: [f32; 2],
    thickness: f32,
    color: [u8; 4],
) -> GResult<()> {
    let c = color_u8!(color[0], color[1], color[2], color[3]);
    draw_triangle_lines(v1.into(), v2.into(), v3.into(), thickness, c);
    Ok(())
}

fn draw_poly_lines_wrapper(
    x: f32,
    y: f32,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: [u8; 4],
) -> GResult<()> {
    let c = color_u8!(color[0], color[1], color[2], color[3]);
    draw_poly_lines(x, y, sides, radius, rotation, thickness, c);
    Ok(())
}

fn draw_text_wrapper(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    color: (u8, u8, u8, u8),
) -> GResult<()> {
    let c = color_u8!(color.0, color.1, color.2, color.3);
    draw_text(text, x, y, font_size, c);
    Ok(())
}

// inputs wrappers

struct GlspKeyCode(KeyCode);

impl IntoVal for GlspKeyCode {
    fn into_val(self) -> GResult<Val> {
        let sym = match self.0 {
            KeyCode::Space => sym!("key:space"),
            KeyCode::Key0 => sym!("key:0"),
            KeyCode::Key1 => sym!("key:1"),
            KeyCode::Key2 => sym!("key:2"),
            KeyCode::Key3 => sym!("key:3"),
            KeyCode::Key4 => sym!("key:4"),
            KeyCode::Key5 => sym!("key:5"),
            KeyCode::Key6 => sym!("key:6"),
            KeyCode::Key7 => sym!("key:7"),
            KeyCode::Key8 => sym!("key:8"),
            KeyCode::Key9 => sym!("key:9"),
            KeyCode::A => sym!("key:a"),
            KeyCode::B => sym!("key:b"),
            KeyCode::C => sym!("key:c"),
            KeyCode::D => sym!("key:d"),
            KeyCode::E => sym!("key:e"),
            KeyCode::F => sym!("key:f"),
            KeyCode::G => sym!("key:g"),
            KeyCode::H => sym!("key:h"),
            KeyCode::I => sym!("key:i"),
            KeyCode::J => sym!("key:j"),
            KeyCode::K => sym!("key:k"),
            KeyCode::L => sym!("key:l"),
            KeyCode::M => sym!("key:m"),
            KeyCode::N => sym!("key:n"),
            KeyCode::O => sym!("key:o"),
            KeyCode::P => sym!("key:p"),
            KeyCode::Q => sym!("key:q"),
            KeyCode::R => sym!("key:r"),
            KeyCode::S => sym!("key:s"),
            KeyCode::T => sym!("key:t"),
            KeyCode::U => sym!("key:u"),
            KeyCode::V => sym!("key:v"),
            KeyCode::W => sym!("key:w"),
            KeyCode::X => sym!("key:x"),
            KeyCode::Y => sym!("key:y"),
            KeyCode::Z => sym!("key:z"),
            KeyCode::Escape => sym!("key:escape"),
            KeyCode::Enter => sym!("key:enter"),
            KeyCode::Tab => sym!("key:tab"),
            KeyCode::Right => sym!("key:right"),
            KeyCode::Left => sym!("key:left"),
            KeyCode::Down => sym!("key:down"),
            KeyCode::Up => sym!("key:up"),
            KeyCode::End => sym!("key:end"),
            KeyCode::PrintScreen => sym!("key:print-screen"),
            KeyCode::LeftShift => sym!("key:left-shift"),
            KeyCode::LeftControl => sym!("key:left-ctrl"),
            KeyCode::LeftAlt => sym!("key:left-alt"),
            KeyCode::LeftSuper => sym!("key:left-super"),
            KeyCode::RightShift => sym!("key:right-shift"),
            KeyCode::RightControl => sym!("key:right-ctrl"),
            KeyCode::RightAlt => sym!("key:right-alt"),
            KeyCode::RightSuper => sym!("key:right-super"),
            _ => sym!("unknown"),
        };

        if sym == sym!("unknown") {
            return bail!("unsupported keycode for now");
        }

        sym.into_val()
    }
}

impl FromVal for GlspKeyCode {
    fn from_val(val: &Val) -> GResult<Self> {
        Ok(match *val {
            Val::Sym(s) if s == sym!("key:space") => GlspKeyCode(KeyCode::Space),
            Val::Sym(s) if s == sym!("key:0") => GlspKeyCode(KeyCode::Key0),
            Val::Sym(s) if s == sym!("key:1") => GlspKeyCode(KeyCode::Key1),
            Val::Sym(s) if s == sym!("key:2") => GlspKeyCode(KeyCode::Key2),
            Val::Sym(s) if s == sym!("key:3") => GlspKeyCode(KeyCode::Key3),
            Val::Sym(s) if s == sym!("key:4") => GlspKeyCode(KeyCode::Key4),
            Val::Sym(s) if s == sym!("key:5") => GlspKeyCode(KeyCode::Key5),
            Val::Sym(s) if s == sym!("key:6") => GlspKeyCode(KeyCode::Key6),
            Val::Sym(s) if s == sym!("key:7") => GlspKeyCode(KeyCode::Key7),
            Val::Sym(s) if s == sym!("key:8") => GlspKeyCode(KeyCode::Key8),
            Val::Sym(s) if s == sym!("key:9") => GlspKeyCode(KeyCode::Key9),
            Val::Sym(s) if s == sym!("key:a") => GlspKeyCode(KeyCode::A),
            Val::Sym(s) if s == sym!("key:b") => GlspKeyCode(KeyCode::B),
            Val::Sym(s) if s == sym!("key:c") => GlspKeyCode(KeyCode::C),
            Val::Sym(s) if s == sym!("key:d") => GlspKeyCode(KeyCode::D),
            Val::Sym(s) if s == sym!("key:e") => GlspKeyCode(KeyCode::E),
            Val::Sym(s) if s == sym!("key:f") => GlspKeyCode(KeyCode::F),
            Val::Sym(s) if s == sym!("key:g") => GlspKeyCode(KeyCode::G),
            Val::Sym(s) if s == sym!("key:h") => GlspKeyCode(KeyCode::H),
            Val::Sym(s) if s == sym!("key:i") => GlspKeyCode(KeyCode::I),
            Val::Sym(s) if s == sym!("key:j") => GlspKeyCode(KeyCode::J),
            Val::Sym(s) if s == sym!("key:k") => GlspKeyCode(KeyCode::K),
            Val::Sym(s) if s == sym!("key:l") => GlspKeyCode(KeyCode::L),
            Val::Sym(s) if s == sym!("key:m") => GlspKeyCode(KeyCode::M),
            Val::Sym(s) if s == sym!("key:n") => GlspKeyCode(KeyCode::N),
            Val::Sym(s) if s == sym!("key:o") => GlspKeyCode(KeyCode::O),
            Val::Sym(s) if s == sym!("key:p") => GlspKeyCode(KeyCode::P),
            Val::Sym(s) if s == sym!("key:q") => GlspKeyCode(KeyCode::Q),
            Val::Sym(s) if s == sym!("key:r") => GlspKeyCode(KeyCode::R),
            Val::Sym(s) if s == sym!("key:s") => GlspKeyCode(KeyCode::S),
            Val::Sym(s) if s == sym!("key:t") => GlspKeyCode(KeyCode::T),
            Val::Sym(s) if s == sym!("key:u") => GlspKeyCode(KeyCode::U),
            Val::Sym(s) if s == sym!("key:v") => GlspKeyCode(KeyCode::V),
            Val::Sym(s) if s == sym!("key:w") => GlspKeyCode(KeyCode::W),
            Val::Sym(s) if s == sym!("key:x") => GlspKeyCode(KeyCode::X),
            Val::Sym(s) if s == sym!("key:y") => GlspKeyCode(KeyCode::Y),
            Val::Sym(s) if s == sym!("key:z") => GlspKeyCode(KeyCode::Z),
            Val::Sym(s) if s == sym!("key:escape") => GlspKeyCode(KeyCode::Escape),
            Val::Sym(s) if s == sym!("key:enter") => GlspKeyCode(KeyCode::Enter),
            Val::Sym(s) if s == sym!("key:tab") => GlspKeyCode(KeyCode::Enter),
            Val::Sym(s) if s == sym!("key:right") => GlspKeyCode(KeyCode::Right),
            Val::Sym(s) if s == sym!("key:left") => GlspKeyCode(KeyCode::Left),
            Val::Sym(s) if s == sym!("key:down") => GlspKeyCode(KeyCode::Down),
            Val::Sym(s) if s == sym!("key:up") => GlspKeyCode(KeyCode::Up),
            Val::Sym(s) if s == sym!("key:end") => GlspKeyCode(KeyCode::End),
            Val::Sym(s) if s == sym!("key:print-screen") => GlspKeyCode(KeyCode::PrintScreen),
            Val::Sym(s) if s == sym!("key:left-shift") => GlspKeyCode(KeyCode::LeftShift),
            Val::Sym(s) if s == sym!("key:left-ctrl") => GlspKeyCode(KeyCode::LeftControl),
            Val::Sym(s) if s == sym!("key:left-alt") => GlspKeyCode(KeyCode::LeftAlt),
            Val::Sym(s) if s == sym!("key:left-super") => GlspKeyCode(KeyCode::LeftSuper),
            Val::Sym(s) if s == sym!("key:right-shift") => GlspKeyCode(KeyCode::RightShift),
            Val::Sym(s) if s == sym!("key:right-ctrl") => GlspKeyCode(KeyCode::RightControl),
            Val::Sym(s) if s == sym!("key:right-alt") => GlspKeyCode(KeyCode::RightAlt),
            Val::Sym(s) if s == sym!("key:right-super") => GlspKeyCode(KeyCode::RightSuper),
            ref val => bail!("unsupported keycode: {}", val),
        })
    }
}

fn is_key_down_wrapper(key: GlspKeyCode) -> GResult<bool> {
    Ok(is_key_down(key.0))
}

fn is_key_pressed_wrapper(key: GlspKeyCode) -> GResult<bool> {
    Ok(is_key_pressed(key.0))
}

// window wrappers

fn screen_size_wrapper() -> GResult<(f32, f32)> {
    let width = screen_width();
    let height = screen_height();
    Ok((width, height))
}

fn get_time_wrapper() -> GResult<f32> {
    Ok(get_time() as f32)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Something".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main("window_conf")]
async fn main() {
    // Window settings

    // Scripting
    let runtime = Runtime::new();

    runtime.run(|| {
        // Bind window specific functions
        glsp::bind_rfn("screen-size", &screen_size_wrapper)?;
        glsp::bind_rfn("get-time", &get_time_wrapper)?;

        // Bind draw functions
        glsp::bind_rfn("clear-background", &clear_background_wrapper)?;
        glsp::bind_rfn("draw-circle", &draw_circle_wrapper)?;
        glsp::bind_rfn("draw-poly-lines", &draw_poly_lines_wrapper)?;
        glsp::bind_rfn("draw-triangle-lines", &draw_triangle_lines_wrapper)?;
        glsp::bind_rfn("draw-text", &draw_text_wrapper)?;

        // Bind input functions
        glsp::bind_rfn("down?", &is_key_down_wrapper)?;
        glsp::bind_rfn("pressed?", &is_key_pressed_wrapper)?;

        // Load scripts
        glsp::load("scripts/asteroids.glsp")?;

        Ok(())
    });

    loop {
        runtime.run(|| {
            // Call scripting update
            let update: Root<GFn> = match glsp::global::<_, Val>("engine:update") {
                Ok(Val::GFn(update)) => update,
                Ok(val) => {
                    let msg = format!("invalid engine:update value {}", val);
                    return Err(GError::from_str(msg.as_str()));
                }
                Err(err) => {
                    let msg = format!("engine:update is not defined {}", err);
                    return Err(GError::from_str(msg.as_str()));
                }
            };

            let dt = get_frame_time();
            let _: Val = match glsp::call(&update, (dt,)) {
                Ok(val) => val,
                Err(err) => return Err(err),
            };

            // Call scripting draw
            let draw: Root<GFn> = match glsp::global::<_, Val>("engine:draw") {
                Ok(Val::GFn(draw)) => draw,
                Ok(val) => {
                    let msg = format!("invalid engine:draw value {}", val);
                    return Err(GError::from_str(msg.as_str()));
                }
                Err(err) => {
                    let msg = format!("engine:draw is not defined {}", err);
                    return Err(GError::from_str(msg.as_str()));
                }
            };

            let _: Val = match glsp::call(&draw, ()) {
                Ok(val) => val,
                Err(err) => return Err(err),
            };

            glsp::gc();

            Ok(())
        });
        next_frame().await
    }
}
