use rustbox;
use rustbox::{RustBox, Color};

use window::Window;

pub fn render(rb: &RustBox, window: &Window) {
    // Write file name to top bar
    rb.print(window.topleft.x as usize, window.topleft.y as usize, rustbox::RB_NORMAL,
    	    Color::Black,
    	    Color::White,
    	    &window.title());
    // Write rest of top bar
    for i in (window.title().len() as i32)..window.size.width {
    	rb.print((window.topleft.x + i) as usize, window.topleft.y as usize,
    		rustbox::RB_NORMAL,
    		Color::Black,
    		Color::White,
    		" ");

    }
    // Write buffer contents
    for i in 1..window.size.height {
        rb.print(window.topleft.x as usize, (window.topleft.y + i) as usize,
                    rustbox::RB_NORMAL,
                    Color::White,
                    Color::Black,
                    window.line(i-1));
    }
}
