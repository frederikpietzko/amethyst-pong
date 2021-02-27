use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct PaddleStystem;

impl<'s> System<'s> for PaddleStystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transform, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transform).join() {
            let (movement, _) = match paddle.side {
                Side::Left => (input.axis_value("left_paddle"), "left"),
                Side::Right => (input.axis_value("right_paddle"), "right"),
            };

            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_movement = 1.2 * mv_amount;
                    let paddle_y = transform.translation().y;
                    transform.set_translation_y(
                        (paddle_y + scaled_movement)
                            .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                            .max(PADDLE_HEIGHT * 0.5),
                    );
                }
            }
        }
    }
}
