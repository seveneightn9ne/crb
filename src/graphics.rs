use rustbox;
use rustbox::{RustBox, Color};

use window::Window;
use buffer::{Symbol};

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
    let mut cursor_is_next = false;
    for cell in window.display() {
        match cell.symbol {
            Symbol::Void => (),
            Symbol::Char(c) => {
                let (fg, bg) = match cursor_is_next {
                    false => (Color::White, Color::Black),
                    true => (Color::Black, Color::White),
                };
                rb.print_char((cell.x + window.topleft.x) as usize,
                              (cell.y + window.topleft.y + 1) as usize,
                              rustbox::RB_NORMAL,
                              fg,
                              bg,
                              c);
                cursor_is_next = false;
            }
            Symbol::Anchor(_) => {
                // TODO check if the anchor is a cursor.
                cursor_is_next = true;
            }
        }
    }
}
