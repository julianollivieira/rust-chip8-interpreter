use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SCALE: u32 = 10;
pub const FOREGROUND_COLOR: Color = Color::RGB(255, 255, 255);
pub const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);

pub struct Display {
    canvas: Canvas<Window>,
    background_color: Color,
    foreground_color: Color,
}

impl Display {
    pub fn new(sdl_context: Sdl) -> Display {
        // Create an SDL2 window with canvas
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "Rust CHIP-8 interpreter",
                WIDTH as u32 * SCALE,
                HEIGHT as u32 * SCALE,
            )
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Display {
            canvas,
            background_color: BACKGROUND_COLOR,
            foreground_color: FOREGROUND_COLOR,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
    }

    pub fn draw(&mut self, pixels: &[[bool; WIDTH]; HEIGHT]) {
        // Clear the canvas
        self.clear();

        // Draw pixels to the display
        self.canvas.set_draw_color(self.foreground_color);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if pixels[y][x] {
                    let x = ((x as u32) * SCALE) as i32;
                    let y = ((y as u32) * SCALE) as i32;
                    self.canvas
                        .fill_rect(Rect::new(x, y, SCALE, SCALE))
                        .expect("Failed to draw pixel");
                }
            }
        }
        self.canvas.present();
    }
}
