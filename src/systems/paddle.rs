use amethyst::core::{math::RealField, Float, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle")
            };

            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + Float::from(scaled_amount))
                        .min(Float::from(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5))
                        .max(Float::from(PADDLE_HEIGHT * 0.5)));
            }
        }
    }
}
