use crossterm::event::KeyCode;
use framebrush::{Canvas, RGBu32};
use std::time::{Duration, Instant};
mod kitty;

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    let (cols, rows) = crossterm::terminal::size()?;

    let mut kitty = kitty::KittyState::new(cols, rows);

    let (width, height) = (1366, 768);
    let mut canvas = Canvas::new(vec![0u32; width * height], (width, height), (width, height));

    let cube_vertices = [
        (-1., -1., -1.),
        (1., -1., -1.),
        (-1., -1., 1.),
        (1., -1., 1.),
        (-1., 1., -1.),
        (1., 1., -1.),
        (-1., 1., 1.),
        (1., 1., 1.),
    ];
    let (mut rot_x, mut rot_y, mut rot_z) = (0., 0., 0.);

    let mut last_frame = Instant::now();
    'main: loop {
        let delta = {
            let now = Instant::now();
            let res = now.duration_since(last_frame).as_secs_f32();
            last_frame = now;
            res
        };

        if let Ok(true) = crossterm::event::poll(Duration::from_millis(0)) {
            if let Ok(ev) = crossterm::event::read() {
                if let Some(ev) = ev.as_key_event() {
                    if ev.code == KeyCode::Char('q') && ev.is_press() {
                        break 'main;
                    }
                }
            }
        }

        rot_x += 1. * delta;
        rot_y += 5. * delta;
        rot_z += 3. * delta;

        let cube_transform = cube_vertices.map(|(x, y, z)| {
            let (offset, scale) = (5., 80.);
            let (x, y, z) = rotate_x(x, y, z, rot_x);
            let (x, y, z) = rotate_y(x, y, z, rot_y);
            let (x, y, z) = rotate_z(x, y, z, rot_z);

            let (x, y, z) = (
                (x + offset) * scale,
                (y + offset / 2.) * scale,
                (z + offset) * scale,
            );
            (x as i32, y as i32, z as i32)
        });

        let mut canvas = canvas.borrowed();
        canvas.fill(0xff000000);
        for (x0, y0, _) in cube_transform {
            for (x1, y1, _) in cube_transform {
                canvas.line(x0, y0, x1, y1, &rgb(255, 0, 0));
            }
        }

        kitty.send_frame(&mut stdout, transmute_slice(canvas.buf()), width, height)?;
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

fn transmute_slice<T, U>(s: &[T]) -> &[U] {
    unsafe {
        std::slice::from_raw_parts(
            s.as_ptr() as *const U,
            s.len() * size_of::<T>() / size_of::<U>(),
        )
    }
}

fn rgb(r: u8, g: u8, b: u8) -> RGBu32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    // ABGR
    RGBu32::Pixel((255 << 24) | (b << 16) | (g << 8) | r)
}

pub fn rotate_x(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        x,
        (y * rot.cos()) - (z * rot.sin()),
        (y * rot.sin()) + (z * rot.cos()),
    )
}

pub fn rotate_y(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        (z * rot.sin()) + (x * rot.cos()),
        y,
        (z * rot.cos()) - (x * rot.sin()),
    )
}

pub fn rotate_z(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        (x * rot.cos()) - (y * rot.sin()),
        (x * rot.sin()) + (y * rot.cos()),
        z,
    )
}
