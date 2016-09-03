use rustbox;
use rustbox::{RustBox, Color};

use window::Window;

pub fn render(rb: &RustBox, window: &Window) {
    for i in 0..window.size.width {
        rb.print(window.topleft.x as usize, (window.topleft.y + i) as usize,
                    rustbox::RB_NORMAL,
                    Color::White,
                    Color::Black,
                    window.line(i));
    }
}
