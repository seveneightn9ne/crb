use rustbox;
use rustbox::{RustBox, Color};
use logging;

use window::Window;

pub fn render(rb: &RustBox, window: &Window) {
    // Write file name to top bar
    rb.print(window.topleft.x as usize,
             window.topleft.y as usize,
             rustbox::RB_NORMAL,
             Color::Black,
             Color::White,
             &window.title());
    // Write rest of top bar
    for i in (window.title().len() as i32)..window.size.width {
        rb.print((window.topleft.x + i) as usize,
                 window.topleft.y as usize,
                 rustbox::RB_NORMAL,
                 Color::Black,
                 Color::White,
                 " ");

    }
    // Write buffer contents
    for rel_y in 1..window.size.height {
        let abs_y = window.topleft.y + rel_y;
        for rel_x in 0..window.size.width {
            let abs_x = window.topleft.x + rel_x;

            let cursor_at = window.cursor_at(rel_x, rel_y);
            if let Some(ch) = window.char_at(rel_x, rel_y) {
                let (fg, bg) = match cursor_at {
                    true => (Color::Black, Color::White),
                    false => (Color::White, Color::Black),
                };
                rb.print_char(abs_x as usize,
                              abs_y as usize,
                              rustbox::RB_NORMAL,
                              fg,
                              bg,
                              ch);
            }
        }
    }
}
