use std::fs;

use crate::engine::{ResourceManager, SpriteRenderer};

use super::{game_object::Drawable, GameObject};

#[derive(Debug)]
pub struct GameLevel {
    bricks: Vec<GameObject>,
}

impl GameLevel {
    pub fn build(
        level_file: &str,
        level_width: u32,
        level_height: u32,
        resource_manager: &ResourceManager,
    ) -> Result<Self, String> {
        let buffer = match fs::read_to_string(level_file) {
            Ok(b) => b,
            Err(_) => return Err(String::from("Couldn't open/find provided level file.")),
        };

        let mut tile_data: Vec<Vec<u8>> = vec![];

        for line in buffer.lines() {
            let brick_line: Vec<u8> = line
                .trim()
                .split_ascii_whitespace()
                .map(|brick| brick.trim().parse::<u8>().unwrap())
                .collect();

            tile_data.push(brick_line);
        }

        // calculate dimensions if we have any tile data available

        if tile_data.len() == 0 {
            return Err(String::from("No tile data in level file."));
        }

        let mut bricks = vec![];

        let height = tile_data.len();
        let width = tile_data[0].len();
        let unit_width = level_width as f32 / width as f32;
        let unit_height = level_height as f32 / height as f32;

        for y in 0..height {
            for x in 0..width {
                if tile_data[y][x] == 1 {
                    let pos = glam::vec2(unit_width * x as f32, unit_height * y as f32);
                    let size = glam::vec2(unit_width, unit_height);
                    let mut obj = GameObject::build(
                        pos,
                        size,
                        resource_manager.get_texture("block_solid").unwrap(),
                        Some(glam::vec3(0.8, 0.8, 0.7)),
                        None,
                    );
                    obj.set_is_solid(true);
                    bricks.push(obj);
                } else if tile_data[y][x] > 1 {
                    let mut color = glam::vec3(1.0, 1.0, 1.0);

                    if tile_data[y][x] == 2 {
                        color = glam::vec3(0.2, 0.6, 1.0);
                    } else if tile_data[y][x] == 3 {
                        color = glam::vec3(0.0, 0.7, 0.0);
                    } else if tile_data[y][x] == 4 {
                        color = glam::vec3(0.8, 0.8, 0.4);
                    } else if tile_data[y][x] == 5 {
                        color = glam::vec3(1.0, 0.8, 0.0);
                    }

                    let pos = glam::vec2(unit_width * x as f32, unit_height * y as f32);
                    let size = glam::vec2(unit_width, unit_height);

                    bricks.push(GameObject::build(
                        pos,
                        size,
                        resource_manager.get_texture("block").unwrap(),
                        Some(color),
                        None,
                    ));
                }
            }
        }

        Ok(Self { bricks })
    }

    pub fn draw(&self, sprite_renderer: &SpriteRenderer) {
        for tile in self.bricks.iter() {
            if !tile.destroyed() {
                tile.draw(sprite_renderer);
            }
        }
    }

    pub fn is_completed(&self) -> bool {
        self.bricks
            .iter()
            .all(|brick| *brick.is_solid() || *brick.destroyed())
    }

    pub fn mut_bricks(&mut self) -> &mut Vec<GameObject> {
        &mut self.bricks
    }
}
