use crate::engine::{SpriteRenderer, Texture};

use super::{game_object::Drawable, Collision, Direction, Game, GameObject};

pub struct BallObject {
    game_object: GameObject,
    radius: f32,
    stuck: bool,
}

impl BallObject {
    pub fn new() -> Self {
        Self {
            radius: 12.5,
            stuck: true,
            game_object: GameObject::new(),
        }
    }

    pub fn build(pos: glam::Vec2, radius: f32, velocity: glam::Vec2, sprite: Texture) -> Self {
        Self {
            radius: 12.5,
            stuck: true,
            game_object: GameObject::build(
                pos,
                glam::vec2(radius * 2.0, radius * 2.0),
                sprite,
                None,
                Some(velocity),
            ),
        }
    }

    pub fn update(&mut self, window_width: u32, player: &GameObject, delta_time: f32) {
        if !self.stuck {
            let velocity = *self.game_object.velocity();

            *self.game_object.mut_position() += velocity * delta_time;

            if self.game_object.mut_position().x <= 0.0 {
                self.game_object.mut_velocity().x = -self.game_object.mut_velocity().x;
                self.game_object.mut_position().x = 0.0;
            } else if self.game_object.mut_position().x + self.game_object.size().x
                >= window_width as f32
            {
                self.game_object.mut_velocity().x = -self.game_object.mut_velocity().x;
                self.game_object.mut_position().x = window_width as f32 - self.game_object.size().x;
            }

            if self.game_object.mut_position().y <= 0.0 {
                self.game_object.mut_velocity().y = -self.game_object.mut_velocity().y;
                self.game_object.mut_position().y = 0.0;
            }
        } else {
            self.game_object.mut_position().x =
                player.position().x + player.size().x / 2.0 - self.game_object.size().x / 2.0;
            self.game_object.mut_position().y = player.position().y - self.game_object.size().y;
        }
    }

    pub fn draw(&self, sprite_renderer: &SpriteRenderer) {
        self.game_object.draw(sprite_renderer)
    }

    pub fn reset(&mut self, position: glam::Vec2, velocity: glam::Vec2) {
        *self.game_object.mut_position() = position;
        *self.game_object.mut_velocity() = velocity;
        self.stuck = true;
    }

    pub fn stuck(&self) -> bool {
        self.stuck
    }

    pub fn set_stuck(&mut self, value: bool) {
        self.stuck = value;
    }

    pub fn collides(&self, rhs: &GameObject) -> Collision {
        let center = *self.game_object.position() + self.radius;
        let aabb_half_extents = *rhs.size() / 2.0;
        let aabb_center = *rhs.position() + aabb_half_extents;
        let difference = center - aabb_center;
        let clamped = difference.clamp(-aabb_half_extents, aabb_half_extents);
        let closest = aabb_center + clamped;
        let difference = closest - center;

        if difference.length() < self.radius {
            (true, self.vector_direction(difference), difference)
        } else {
            (false, Direction::Up, glam::vec2(0.0, 0.0))
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn game_object(&self) -> &GameObject {
        &self.game_object
    }

    pub fn mut_game_object(&mut self) -> &mut GameObject {
        &mut self.game_object
    }

    fn vector_direction(&self, target: glam::Vec2) -> Direction {
        let compass = vec![
            glam::vec2(0.0, 1.0),
            glam::vec2(1.0, 0.0),
            glam::vec2(0.0, -1.0),
            glam::vec2(-1.0, 0.0),
        ];

        let mut max: f32 = 0.0;
        let mut best_match: usize = 0;

        for (i, direction) in compass.iter().enumerate() {
            let dot_product = target.normalize().dot(*direction);

            if dot_product > max {
                max = dot_product;
                best_match = i;
            }
        }

        if best_match == 0 {
            Direction::Up
        } else if best_match == 1 {
            Direction::Right
        } else if best_match == 2 {
            Direction::Down
        } else {
            Direction::Left
        }
    }
}
