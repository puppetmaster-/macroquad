use macroquad::*;

use glam::{vec2, Vec2};

struct Bird {
    pos: Vec2,
    vel: Vec2,
}

async fn bird_input(bird: GameObject<Bird>) {
    while let Some(event) = next_event().await {
        match event {
            Event::KeyDown(KeyCode::Space) => {
                let mut bird = bird.get();
                bird.vel = vec2(0., -5.);
            }
            _ => {}
        }
    }
}

async fn bird_physics(bird: GameObject<Bird>) {
    loop {
        {
            let bird = &mut *bird.get();

            bird.vel += vec2(0., 0.15);
            bird.pos += bird.vel;
        }
        next_frame().await
    }
}

#[macroquad::main("Texture")]
async fn main() {
    let bird = GameObject::new(Bird {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        vel: Vec2::new(0., 0.),
    });

    start_coroutine(bird_input(bird));
    start_coroutine(bird_physics(bird));

    loop {
        clear_background(WHITE);

        {
            let bird = bird.get();
            draw_circle(bird.pos.x(), bird.pos.y(), 15.0, BLACK);
        }

        next_frame().await
    }
}
