use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        let mut x = 1.0;
        while 100.0 > x{
            x+= 1.0;
        }
        draw_rectangle(x, x, 300.0, 300.0, Color::from_hex(0x34AD34));
        next_frame().await; // BU JUDA MUHIM!
    }
}

