use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        
        draw_rectangle(10.0, 10.0, 300.0, 300.0, Color::from_hex(0x34AD34));
        next_frame().await; // BU JUDA MUHIM!
    }
}

