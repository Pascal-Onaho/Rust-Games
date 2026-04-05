use macroquad::prelude::*;

#[macroquad::main("GAME TEST")]

async fn main() {

    struct Shape {
        size: f32,
        speed: f32,
        x: f32,
        y: f32,
    }
     const MOVEMENT_SPEED: f32 = 200.0;
     let mut x = screen_width() / 2.0;
     let mut y = screen_height() / 2.0;
    rand::srand(miniquad::date::now() as u64);
    let mut squares = vec![];
    let mut _circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
   

    let delta_time = get_frame_time();
    loop {
        clear_background(RED);

        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }

        draw_circle(x, y, 16.0, WHITE);

        x = clamp(x, 0.0, screen_width());
        y = clamp(y, 0.0, screen_height());
         if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }

         for square in &mut squares {
            square.y += square.speed * delta_time;
        }
        squares.retain(|square| square.y < screen_height() + square.size);
         for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                BLUE,
            );
        }

        next_frame().await

    }


}