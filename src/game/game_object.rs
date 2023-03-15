use crate::engine::{SpriteRenderer, Texture};

pub trait Drawable {
    fn draw(&self, sprite_renderer: &SpriteRenderer);
}

#[derive(Debug)]
pub struct GameObject {
    position: glam::Vec2,
    size: glam::Vec2,
    velocity: glam::Vec2,
    color: glam::Vec3,
    rotation: f32,
    is_solid: bool,
    destroyed: bool,
    sprite: Option<Texture>,
}

impl GameObject {
    pub fn new() -> Self {
        Self {
            position: glam::vec2(0.0, 0.0),
            size: glam::vec2(1.0, 1.0),
            velocity: glam::vec2(0.0, 0.0),
            color: glam::vec3(1.0, 1.0, 1.0),
            rotation: 0.0,
            sprite: None,
            is_solid: false,
            destroyed: false,
        }
    }

    pub fn build(
        pos: glam::Vec2,
        size: glam::Vec2,
        sprite: Texture,
        color: Option<glam::Vec3>,
        velocity: Option<glam::Vec2>,
    ) -> Self {
        Self {
            position: pos,
            size,
            rotation: 0.0,
            sprite: Some(sprite),
            is_solid: false,
            destroyed: false,
            color: color.unwrap_or_else(|| glam::vec3(1.0, 1.0, 1.0)),
            velocity: velocity.unwrap_or_else(|| glam::vec2(0.0, 0.0)),
        }
    }

    pub fn mut_position(&mut self) -> &mut glam::Vec2 {
        &mut self.position
    }

    pub fn position(&self) -> &glam::Vec2 {
        &self.position
    }

    pub fn size(&self) -> &glam::Vec2 {
        &self.size
    }

    pub fn mut_velocity(&mut self) -> &mut glam::Vec2 {
        &mut self.velocity
    }

    pub fn velocity(&self) -> &glam::Vec2 {
        &self.velocity
    }

    pub fn set_is_solid(&mut self, value: bool) {
        self.is_solid = value;
    }

    pub fn is_solid(&self) -> &bool {
        &self.is_solid
    }

    pub fn set_destroyed(&mut self, value: bool) {
        self.destroyed = value;
    }

    pub fn destroyed(&self) -> &bool {
        &self.destroyed
    }

    pub fn collides(&self, rhs: &GameObject) -> bool {
        let collision_x = self.position.x + self.size.x >= rhs.position.x
            && rhs.position.x + rhs.size.x >= self.position.x;

        let collision_y = self.position.y + self.size.y >= rhs.position.y
            && rhs.position.y + rhs.size.y >= self.position.y;

        collision_x && collision_y
    }
}

impl Drawable for GameObject {
    fn draw(&self, sprite_renderer: &SpriteRenderer) {
        if let Some(sprite) = self.sprite {
            sprite_renderer.draw_sprite(sprite, self.position, self.size, self.rotation, self.color)
        }
    }
}
