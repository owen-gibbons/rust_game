use macroquad::prelude::*;

struct Player {
    x: f32,
    y: f32,
    speed: f32,
}

impl Player {
        fn new() -> Self {
        Self { x: 400.0, y: 300.0, speed: 5.0 }
    }
    fn update(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.x -= self.speed;
        }
        if is_key_down(KeyCode::Right) {
            self.x += self.speed;
        }
    }
    fn draw(&self) {
        draw_rectangle(self.x, self.y, 50.0, 50.0, BLUE);
    }
}
#[macroquad::main("Player Movement")]
async fn main() {
    loop {
        clear_background(BLACK);
        let mut player = Player::new();   
        // Handle exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        player.update();
        player.draw();
        next_frame().await;
    }
}

