use sdl2::{keyboard::Scancode, EventPump};

use crate::engine::{Renderer, ResourceManager, SpriteRenderer};

mod ball_object;
mod game_level;
mod game_object;

pub use ball_object::BallObject;
pub use game_level::GameLevel;
pub use game_object::GameObject;

use self::game_object::Drawable;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

type Collision = (bool, Direction, glam::Vec2);

#[derive(PartialEq)]
enum GameState {
    PLAYING,
    MENU,
    WIN,
}

pub struct Game {
    state: GameState,
    width: u32,
    height: u32,
    resource_manager: ResourceManager,
    sprite_renderer: SpriteRenderer,
    levels: Vec<GameLevel>,
    level: usize,
    player: GameObject,
    ball: BallObject,
}

const PLAYER_SIZE: glam::Vec2 = glam::vec2(100.0, 20.0);
const PLAYER_VELOCITY: f32 = 0.75;
const PADDING: f32 = 10.0;
const INITIAL_BALL_VELOCITY: glam::Vec2 = glam::vec2(0.15, -0.45);
const BALL_RADIUS: f32 = 12.5;

impl Game {
    pub fn build(width: u32, height: u32) -> Result<Self, String> {
        let mut resource_manager = ResourceManager::new();

        // load textures
        resource_manager.load_texture("./src/game/textures/background.jpeg", false, "background");
        resource_manager.load_texture("./src/game/textures/awesomeface.png", true, "face");
        resource_manager.load_texture("./src/game/textures/block.png", false, "block");
        resource_manager.load_texture("./src/game/textures/block_solid.png", false, "block_solid");
        resource_manager.load_texture("./src/game/textures/paddle.png", true, "paddle");

        // load levels
        let one = GameLevel::build(
            "./src/game/levels/1.level",
            width,
            height / 2,
            &resource_manager,
        )
        .unwrap();
        let two = GameLevel::build(
            "./src/game/levels/2.level",
            width,
            height / 2,
            &resource_manager,
        )
        .unwrap();
        let three = GameLevel::build(
            "./src/game/levels/3.level",
            width,
            height / 2,
            &resource_manager,
        )
        .unwrap();
        let four = GameLevel::build(
            "./src/game/levels/4.level",
            width,
            height / 2,
            &resource_manager,
        )
        .unwrap();
        let five = GameLevel::build(
            "./src/game/levels/5.level",
            width,
            height / 2,
            &resource_manager,
        )
        .unwrap();

        let player_pos = glam::vec2(
            width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            height as f32 - PLAYER_SIZE.y - PADDING,
        );

        let player = GameObject::build(
            player_pos,
            PLAYER_SIZE,
            resource_manager.get_texture("paddle").unwrap(),
            None,
            None,
        );

        let ball_pos =
            player_pos + glam::vec2(PLAYER_SIZE.x / 2.0 - BALL_RADIUS, -BALL_RADIUS * 2.0);

        let ball = BallObject::build(
            ball_pos,
            BALL_RADIUS,
            INITIAL_BALL_VELOCITY,
            resource_manager.get_texture("face").unwrap(),
        );

        Ok(Self {
            width,
            height,
            resource_manager,
            player,
            ball,
            state: GameState::PLAYING,
            sprite_renderer: SpriteRenderer::new(width, height),
            levels: vec![one, two, three, four, five],
            level: 0,
        })
    }

    pub fn play(&mut self, renderer: &mut Renderer) {
        renderer.game_loop(&mut |event_pump, delta_time| {
            self.process_input(event_pump, delta_time);
            self.update(delta_time);
            self.draw();
        });
    }

    fn process_input(&mut self, event_pump: &EventPump, delta_time: f32) {
        if self.state == GameState::PLAYING {
            let velocity = PLAYER_VELOCITY * delta_time;

            for scancode in event_pump.keyboard_state().pressed_scancodes() {
                match scancode {
                    Scancode::A => {
                        if self.player.mut_position().x > PADDING {
                            self.player.mut_position().x -= velocity;
                        } else {
                            self.player.mut_position().x = PADDING;
                        }
                    }
                    Scancode::D => {
                        if self.player.mut_position().x
                            < self.width as f32 - self.player.size().x - PADDING
                        {
                            self.player.mut_position().x += velocity;
                        } else {
                            self.player.mut_position().x =
                                self.width as f32 - self.player.size().x - PADDING;
                        }
                    }
                    Scancode::Space => {
                        self.ball.set_stuck(false);
                    }
                    _ => {}
                }
            }
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.ball.update(self.width, &self.player, delta_time);
        self.collide();

        if self.ball.game_object().position().y >= self.height as f32 {
            self.reset_level();
            self.reset_player();
        }
    }

    fn draw(&mut self) {
        if self.state == GameState::PLAYING {
            // draw background
            self.sprite_renderer.draw_sprite(
                self.resource_manager.get_texture("background").unwrap(),
                glam::vec2(0.0, 0.0),
                glam::vec2(self.width as f32, self.height as f32),
                0.0,
                glam::vec3(1.0, 1.0, 1.0),
            );
            // draw level
            self.levels[self.level].draw(&self.sprite_renderer);
            // draw player
            self.player.draw(&self.sprite_renderer);
            // draw ball
            self.ball.draw(&self.sprite_renderer);
        }
    }

    fn reset_level(&mut self) {
        let current_level_path = format!("./src/game/levels/{}.level", self.level + 1);

        self.levels[self.level] = GameLevel::build(
            &current_level_path,
            self.width,
            self.height / 2,
            &self.resource_manager,
        )
        .unwrap();
    }

    fn reset_player(&mut self) {
        let player_pos = glam::vec2(
            self.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            self.height as f32 - PLAYER_SIZE.y - PADDING,
        );

        *self.player.mut_position() = player_pos;

        let ball_pos =
            player_pos + glam::vec2(PLAYER_SIZE.x / 2.0 - BALL_RADIUS, -BALL_RADIUS * 2.0);

        self.ball.set_stuck(true);
        *self.ball.mut_game_object().mut_position() = ball_pos;
    }

    fn collide(&mut self) {
        for brick in self.levels[self.level].mut_bricks() {
            if !brick.destroyed() {
                let collision = self.ball.collides(brick);

                if collision.0 {
                    if !brick.is_solid() {
                        brick.set_destroyed(true);
                    }
                    let dir = collision.1;
                    let diff_vector = collision.2;

                    if dir == Direction::Left || dir == Direction::Right {
                        self.ball.mut_game_object().mut_velocity().x =
                            -self.ball.mut_game_object().velocity().x;
                        let penetration = self.ball.radius() - diff_vector.x.abs();

                        if dir == Direction::Left {
                            self.ball.mut_game_object().mut_position().x += penetration;
                        } else {
                            self.ball.mut_game_object().mut_position().x -= penetration;
                        }
                    } else {
                        self.ball.mut_game_object().mut_velocity().y =
                            -self.ball.mut_game_object().mut_velocity().y;

                        let pentration = self.ball.radius() - diff_vector.y.abs();

                        if dir == Direction::Up {
                            self.ball.mut_game_object().mut_position().y -= pentration;
                        } else {
                            self.ball.mut_game_object().mut_position().y += pentration;
                        }
                    }
                }
            }
        }

        let result = self.ball.collides(&self.player);

        if !self.ball.stuck() && result.0 {
            self.ball.mut_game_object().mut_velocity().y =
                -1.0 * self.ball.game_object().velocity().y.abs();
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {}
}
