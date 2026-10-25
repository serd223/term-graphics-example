use cairo::ImageSurface;
use crossterm::event::KeyCode;
use std::time::Duration;

pub mod kitty;
pub mod microui;
use microui::{Command, rect};

fn main() -> std::io::Result<()> {
    let (width, height) = (1366, 768);
    let mut buf = vec![0u8; width as usize * height as usize * 4].into_boxed_slice();
    {
        let mut ctx = microui::Context::new();
        ctx.begin();
        if ctx.begin_window("Hello, World!", rect(0, 0, width, height)) {
            ctx.label("Test Label");
            ctx.label("Foo Bar Baz");
            ctx.label("Hello from the terminal!");
            ctx.end_window();
        }
        ctx.end();
        process_microui_commands(&mut ctx, &mut buf, width, height);
    };

    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    let (cols, rows) = crossterm::terminal::size()?;
    let mut kitty = kitty::KittyState::new(cols, rows);

    'main: loop {
        if let Ok(true) = crossterm::event::poll(Duration::from_millis(0)) {
            if let Ok(ev) = crossterm::event::read() {
                if let Some(ev) = ev.as_key_event() {
                    if ev.code == KeyCode::Char('q') && ev.is_press() {
                        break 'main;
                    }
                }
            }
        }
        kitty.send_frame(&mut stdout, &buf, width, height)?;
    }

    // cleanup
    kitty.cleanup(&mut stdout)?;
    crossterm::execute!(
        stdout,
        crossterm::cursor::Show,
        crossterm::terminal::LeaveAlternateScreen
    )?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn process_microui_commands(
    ctx: &mut microui::Context,
    framebuffer: &mut Box<[u8]>,
    width: i32,
    height: i32,
) {
    // Needed due to the weird cairo API. This is completely fine in our case.
    let buf: &'static mut [u8] =
        unsafe { core::slice::from_raw_parts_mut(framebuffer.as_mut_ptr(), framebuffer.len()) };
    let surface =
        ImageSurface::create_for_data(buf, cairo::Format::ARgb32, width, height, width * 4)
            .unwrap();
    let cairo = cairo::Context::new(&surface).unwrap();
    while let Some(cmd) = ctx.pop_command() {
        match cmd {
            Command::Rect { rect, color } => {
                _ = color;
                cairo.set_source_rgba(
                    color.r as f64 / 255.0,
                    color.g as f64 / 255.0,
                    color.b as f64 / 255.0,
                    color.a as f64 / 255.0,
                );
                cairo.rectangle(rect.x as f64, rect.y as f64, rect.w as f64, rect.h as f64);
                cairo.fill().unwrap();
            }
            Command::Text { post, color, str } => {
                cairo.set_source_rgba(
                    color.r as f64 / 255.0,
                    color.g as f64 / 255.0,
                    color.b as f64 / 255.0,
                    color.a as f64 / 255.0,
                );
                cairo.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
                cairo.set_font_size(18.0);
                cairo.move_to(post.x as f64, post.y as f64 + 18.0);
                let _ = cairo.show_text(&str);
            }
            Command::Icon { rect, id, color } => {
                // TODO: Draw icon
                _ = id;
                cairo.set_source_rgba(
                    color.r as f64 / 255.0,
                    color.g as f64 / 255.0,
                    color.b as f64 / 255.0,
                    color.a as f64 / 255.0,
                );
                cairo.rectangle(rect.x as f64, rect.y as f64, rect.w as f64, rect.h as f64);
                cairo.fill().unwrap();
            }
            _ => todo!(),
        }
    }
}
