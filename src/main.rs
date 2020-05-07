extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video = ctx.video()?;
    let window = video
        .window("Rust with SDL2", 16 * 40, 9 * 40)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = ctx.event_pump()?;

    'main_loop: loop {
        for e in event_pump.poll_iter() {
            match e {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }

        let window = canvas.window_mut();
        let size = window.size();
        let size_i = (size.0 as i32, size.1 as i32);

        canvas.set_draw_color(Color::RGB(0x2b, 0x2b, 0x2b));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0xf3, 0xf3, 0xf3));
        canvas.fill_rect(Rect::new(
            size_i.0 / 4,
            size_i.1 / 4,
            size.0 / 2,
            size.1 / 2,
        ))?;
        canvas.present();
    }

    Ok(())
}
