use raylib::prelude::{Vector2, Color};

pub enum Person {
    Male {
        position: Vector2,
        velocity: Vector2,
        speed: f32,
        radius: f32,
        color: Color
    },
    Woman {
        position: Vector2,
        velocity: Vector2,
        speed: f32,
        radius: f32,
        color: Color
    }
}

impl Person {
    pub fn update_position(&mut self, f_time: f32) {
        match self {
            Person::Male { position, velocity, speed, radius: _, color: _ } => {
                position.x += *speed * velocity.x * f_time;
                position.y += *speed * velocity.y * f_time;
            },
            Person::Woman { position, velocity, speed, radius: _, color: _ } => {
                position.x += *speed * velocity.x * f_time;
                position.y += *speed * velocity.y * f_time;
            }
        }
    }
}