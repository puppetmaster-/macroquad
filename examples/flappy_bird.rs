use macroquad::*;

use glam::{vec2, Vec2};

struct Bird {
    pos: Vec2,
    vel: Vec2,
}

struct Obstacle {
    x: f32,
    top: f32,
    bottom: f32,
    width: f32,
}

struct Label {
    msg: String,
    x: f32,
    y: f32,
}

async fn intro() {
    let label = scene::GameObject::new(Label {
        msg: "hello!".to_owned(),
        x: screen_width() / 2. - 70.,
        y: screen_height() / 2. - 25.,
    });

    wait_seconds(0.7).await;

    label.get().delete();

    let left_bar = scene::GameObject::new(Obstacle {
        x: 0.,
        top: 0.,
        width: 10.,
        bottom: screen_height(),
    });
    let right_bar = scene::GameObject::new(Obstacle {
        x: screen_width() - 10.,
        top: 0.,
        width: 10.,
        bottom: screen_height(),
    });

    for i in 0..30 {
        {
            let mut left_bar = left_bar.get();
            left_bar.x = screen_width() / 30. * i as f32 / 2.;

            let mut right_bar = right_bar.get();
            right_bar.x = screen_width() - screen_width() / 30. * i as f32 / 2.;
        }
        next_frame().await;
    }

    wait_seconds(0.3).await;

    for i in (0..30).rev() {
        {
            let mut left_bar = left_bar.get();
            left_bar.x = screen_width() / 30. * i as f32 / 2.;

            let mut right_bar = right_bar.get();
            right_bar.x = screen_width() - screen_width() / 30. * i as f32 / 2. - 10.;
        }
        next_frame().await;
    }

    scene::GameObject::new(Label {
        msg: "Press spacebar to play!".to_owned(),
        x: screen_width() / 2. - 330.,
        y: screen_height() / 2. - 25.,
    });

    // for for space bar press
    while next_event().await != Some(Event::KeyDown(KeyCode::Space)) {}

    scene::clear();
}

async fn game_core() {
    set_screen_coordinates(ScreenCoordinates::PixelPerfect);
    let w = 20.;
    let h = 20.;

    let bird = scene::GameObject::new(Bird {
        pos: Vec2::new(0., 0.),
        vel: Vec2::new(0., 0.),
    });

    set_screen_coordinates(ScreenCoordinates::Fixed(-w / 2., w / 2., -h / 2., h / 2.));

    let input_coroutine = start_coroutine(async move {
        while let Some(event) = next_event().await {
            match event {
                Event::KeyDown(KeyCode::Space) => {
                    let mut bird = bird.get();
                    bird.vel = vec2(0., 0.2);
                }
                _ => {}
            }
        }
    });

    let mut interval = 18.;

    let mut obstacles: Vec<scene::GameObject<Obstacle>> = vec![];

    'main_loop: loop {
        obstacles.retain(|obstacle| {
            let mut obstacle = obstacle.get();
            if obstacle.x < -w / 2. - 1. {
                obstacle.delete();
                return false;
            }
            return true;
        });

        let mut need_obstacle = true;
        if let Some(obstacle) = obstacles.last() {
            let obstacle = obstacle.get();
            if w / 2. - obstacle.x < interval {
                need_obstacle = false
            }
        }

        if need_obstacle {
            interval = (4f32).max(interval * 0.8);

            let mid = rand::gen_range(-h / 2., 0.);
            obstacles.push(scene::GameObject::new(Obstacle {
                x: w / 2.,
                top: -h / 2.,
                bottom: mid,
                width: 1.,
            }));

            obstacles.push(scene::GameObject::new(Obstacle {
                x: w / 2.,
                top: mid + 5.,
                bottom: h / 2.,
                width: 1.,
            }))
        }
        for obstacle in &obstacles {
            let mut obstacle = obstacle.get();
            obstacle.x -= 0.1;

            let bird = bird.get();

            if bird.pos.x() >= obstacle.x
                && bird.pos.x() < obstacle.x + obstacle.width
                && bird.pos.y() > obstacle.top
                && bird.pos.y() < obstacle.bottom
            {
                break 'main_loop; //dead
            }
        }

        {
            let bird = &mut *bird.get();

            bird.vel -= vec2(0., 0.01);
            bird.pos += bird.vel;

            // dead
            if bird.pos.y() < -h / 2. {
                break;
            }
        }

        next_frame().await
    }

    stop_coroutine(input_coroutine);
}

async fn death_animation() {
    for i in 0..70 {
        for gameobject in scene::all_scene_objects() {
            if let Some(obstacle) = gameobject.downcast_mut::<Obstacle>() {
                obstacle.x *= 0.8;
                let h = obstacle.bottom - obstacle.top;

                if obstacle.top <= -9.5 {
                    obstacle.bottom = -10. + h * (100 - i) as f32 / 100.;
                } else {
                    obstacle.top = 10. - h * (100 - i) as f32 / 100.;
                }
            }
            if let Some(bird) = gameobject.downcast_mut::<Bird>() {
                bird.pos *= Vec2::new(0.9, 0.9);
            }
        }
        next_frame().await;
    }

    wait_seconds(0.1).await;

    scene::clear();
}

#[macroquad::main("Flappy bird")]
async fn main() {
    start_coroutine(async {
        intro().await;

        loop {
            game_core().await;
            death_animation().await;
        }
    });

    loop {
        clear_background(RED);

        for gameobject in scene::all_scene_objects() {
            if let Some(obstacle) = gameobject.downcast_mut::<Obstacle>() {
                draw_rectangle(
                    obstacle.x,
                    obstacle.top,
                    obstacle.width,
                    obstacle.bottom - obstacle.top,
                    BLACK,
                );
            }
            if let Some(bird) = gameobject.downcast_mut::<Bird>() {
                draw_circle(bird.pos.x(), bird.pos.y(), 0.2, BLACK);
            }
            if let Some(label) = gameobject.downcast_mut::<Label>() {
                draw_text(&label.msg, label.x, label.y, 30., BLACK);
            }
        }

        next_frame().await
    }
}
