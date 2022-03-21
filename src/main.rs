use raylib::prelude::*;
use rand::Rng;

mod person;

// Game constants
const WIDTH: i32 = 1280;
const HEIGHT: i32 = 960;
const TARGET_FPS: u32 = 60;
const FPS_TEXT_COLOR: Color = Color::WHITE;
const BACKGROUND_COLOR: Color = Color::BLACK;

// Person constants
const PERSON_INIT_COUNT: i32 = 20;
const PERSON_SPEED: f32 = 25.0;
const PERSON_RADIUS: f32 = 10.0;
const PERSON_MALE_COLOR: Color = Color::BLUE;
const PERSON_WOMAN_COLOR: Color = Color::PINK;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Ecosystem Simulation")
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut person_list: Vec<person::Person> = Vec::new();
    let mut rng = rand::thread_rng();

    // Generate person list
    for i in 0..PERSON_INIT_COUNT {
        let mut velocity_x: f32 = rng.gen_range(-1.0..1.0);
        let mut velocity_y: f32 = rng.gen_range(-1.0..1.0);

        if velocity_x == 0.0 { velocity_x = 1.0; };
        if velocity_y == 0.0 { velocity_y = 1.0; };

        let person: person::Person = if i < PERSON_INIT_COUNT / 2 {
            person::Person::Male {
                position: Vector2 {
                    x: rand::thread_rng().gen_range(0..WIDTH) as f32,
                    y: rand::thread_rng().gen_range(0..HEIGHT) as f32
                },
                velocity: Vector2 {
                    x: velocity_x,
                    y: velocity_y
                },
                speed: PERSON_SPEED,
                radius: PERSON_RADIUS,
                color: PERSON_MALE_COLOR
            }
        } else {
            person::Person::Woman {
                position: Vector2 {
                    x: rand::thread_rng().gen_range(0..WIDTH) as f32,
                    y: rand::thread_rng().gen_range(0..HEIGHT) as f32
                },
                velocity: Vector2 {
                    x: rand::thread_rng().gen_range(-1.0..1.0),
                    y: rand::thread_rng().gen_range(-1.0..1.0)
                },
                speed: PERSON_SPEED,
                radius: PERSON_RADIUS,
                color: PERSON_WOMAN_COLOR
            }
        };

        person_list.push(person);
    }

    // Draw to screen
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let mut fps_text: String = String::from("FPS: ");
        let fps_rate: String = d.get_fps().to_string();
        fps_text.push_str(&fps_rate);
        
        // Draw background and fps text
        d.clear_background(BACKGROUND_COLOR);
        d.draw_text("Ecosystem Simulation", 12, 12, 20, FPS_TEXT_COLOR);
        d.draw_text(&fps_text, 12, 40, 20, FPS_TEXT_COLOR);

        let f_time: f32 = d.get_frame_time() as f32;

        // Draw person list
        for i in 0..person_list.len() {
            match person_list.get(i) {
                Some(p) => {
                    match p {
                        person::Person::Male { position, velocity: _, speed: _, radius, color } => {
                            d.draw_circle(
                                position.x as i32, 
                                position.y as i32, 
                                *radius, 
                                color
                            );
                        },
                        person::Person::Woman { position, velocity: _, speed, radius, color } => {
                            d.draw_circle(
                                position.x as i32, 
                                position.y as i32, 
                                *radius, 
                                color
                            );
                        }
                    }
                },
                None => continue
            };
        }
    }
}
