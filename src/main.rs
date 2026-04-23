use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        let mut x = 0.0;
        let mut a = true;
        
        if x <= 1.0 {a = true;} 
        else if x >= 1000.0 {a = false;}

        if a{x+=10.0;}
        else{x-=10.0;}
        
        draw_rectangle(x, x, 300.0, 300.0, Color::from_hex(0x34AD34));
        next_frame().await; // BU JUDA MUHIM!
    }
}
