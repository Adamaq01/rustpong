use tetra::graphics::{Texture, Drawable, DrawParams};
use tetra::math::Vec2;
use tetra::{Context, input};
use crate::updatable::Updatable;
use tetra::input::Key;
use crate::WINDOW_HEIGHT;
use crate::entities::Entity;

const PLAYER_SPEED: f32 = 8.0;

pub struct Player {
    pub entity: Entity,
    pub up_key: Key,
    pub down_key: Key,
}

impl Player {
    pub fn new(texture: Texture, position: Vec2<f32>, up_key: Key, down_key: Key) -> Player {
        Player { entity: Entity { position, texture }, up_key, down_key }
    }
}

impl Drawable for Player {
    fn draw<P>(&self, ctx: &mut Context, params: P) where
        P: Into<DrawParams> {
        self.entity.draw(ctx, params)
    }
}

impl Updatable for Player {
    fn update(&mut self, ctx: &mut Context) {
        if input::is_key_down(ctx, self.up_key) {
            self.entity.position.y -= PLAYER_SPEED;
        }

        if input::is_key_down(ctx, self.down_key) {
            self.entity.position.y += PLAYER_SPEED;
        }

        if self.entity.position.y <= 16.0 {
            self.entity.position.y = 16.0
        } else if self.entity.position.y >= (WINDOW_HEIGHT - self.entity.texture.height()) as f32 - 16.0 {
            self.entity.position.y = (WINDOW_HEIGHT - self.entity.texture.height()) as f32 - 16.0;
        }
    }
}
