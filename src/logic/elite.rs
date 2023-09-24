use bevy::math::vec2;
use rand::{Rng, thread_rng};

use crate::entities::Ships;
use crate::logic::movement::Moves;
use crate::logic::wave::SpecialEvent;
use crate::util::{HALF_HEIGHT, HALF_WIDTH, HEIGHT};

pub fn gen_elite_wave(_level: usize) -> Vec<SpecialEvent> {
    match thread_rng().gen_range(0..4) {
        0 => vec![
            SpecialEvent::Spawn(
                Ships::Elite(0),
                Moves::DownUntil(
                    vec2(HALF_WIDTH, HEIGHT as f32 + 16.),
                    HALF_HEIGHT * 6. / 5.,
                    0.,
                    Box::new(Moves::Lemniscate(vec2(HALF_WIDTH, HALF_HEIGHT * 6. / 5.), 1.2, 32.)))
            ),
            SpecialEvent::InfiniteWave(11000, HALF_HEIGHT - 4., true, 0.),
        ],
        1 => vec![
            SpecialEvent::Spawn(
                Ships::Elite(1),
                Moves::DownUntil(
                    vec2(HALF_WIDTH, HEIGHT as f32 + 16.),
                    HALF_HEIGHT + 24.,
                    0.,
                    Box::new(Moves::Ellipsis(vec2(HALF_WIDTH, HALF_HEIGHT + 24.), 2.0, 32., 20.)))
            ),
            SpecialEvent::InfiniteWave(8000, HEIGHT as f32 - 24., false, 0.),
        ],
        2 => vec![
            SpecialEvent::Spawn(
                Ships::Elite(2),
                Moves::DownUntil(
                    vec2(HALF_WIDTH, HEIGHT as f32 + 16.),
                    HALF_HEIGHT + 24.,
                    0.,
                    Box::new(Moves::Astroid(vec2(HALF_WIDTH, HALF_HEIGHT + 24.), 1.2, 32., 12.))
                )
            ),
            SpecialEvent::InfiniteWave(10000, HEIGHT as f32 - 24., false, 0.),
        ],
        _ => vec![
            SpecialEvent::Spawn(
                Ships::Elite(3),
                Moves::DownUntil(
                    vec2(HALF_WIDTH, HEIGHT as f32 + 16.),
                    HALF_HEIGHT + 20.,
                    0.,
                    Box::new(Moves::Ellipsis(vec2(HALF_WIDTH, HALF_HEIGHT + 20.), 0.8, 48., 16.))
                )
            ),
            SpecialEvent::InfiniteWave(10000, HEIGHT as f32 - 32., false, 0.),
        ],
    }
}

pub fn gen_boss_wave(level: usize) -> Vec<SpecialEvent> {
    vec![]
}