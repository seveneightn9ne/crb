use rustbox;
use rustbox::{RustBox, Color};

use window::Window;
use buffer::Symbol;

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
    window.display(|cell| {
        let one_for_the_bar = 1;
        let y = (cell.y + window.topleft.y + one_for_the_bar) as usize;
        let x = (cell.x + window.topleft.x) as usize;
        let sty = rustbox::RB_NORMAL;
        let white = Color::White;
        let black = Color::Black;
        let (fg, bg) = match cursor_is_next {
            false => (white, black),
            true => (black, white),
        };
        match cell.symbol {
            Symbol::Void => {
                rb.print_char(x, y, sty, fg, bg, ' ');
                cursor_is_next = false;
            }
            Symbol::Skip => {}
            Symbol::Char(c) => {
                rb.print_char(x, y, sty, fg, bg, c);
                cursor_is_next = false;
            }
            Symbol::Anchor(_) => {
                // TODO check if the anchor is a cursor.
                cursor_is_next = true;
            }
            Symbol::ColorChar(c, color) => {
                rb.print_char(x, y, sty, color, bg, c);
                cursor_is_next = false;
            }
        }
    });
}
