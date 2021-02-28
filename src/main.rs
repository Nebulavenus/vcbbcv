use glsp::prelude::*;
use macroquad::prelude::*;

fn clear_background_wrapper(color: (u8, u8, u8, u8)) -> GResult<()> {
    let c = color_u8!(color.0, color.1, color.2, color.3);
    clear_background(c);
    Ok(())
}

fn draw_circle_wrapper(x: f32, y: f32, r: f32, color: (u8, u8, u8, u8)) -> GResult<()> {
    let c = color_u8!(color.0, color.1, color.2, color.3);
    draw_circle(x, y, r, c);
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

#[macroquad::main("Something")]
async fn main() {
    let runtime = Runtime::new();

    runtime.run(|| {
        // Load scripts
        glsp::load("scripts/main.glsp")?;

        // Bind functions
        glsp::bind_rfn("clear-background", &clear_background_wrapper)?;
        glsp::bind_rfn("draw-circle", &draw_circle_wrapper)?;
        glsp::bind_rfn("draw-text", &draw_text_wrapper)?;

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
