use bevy::math::Vec4;
use bevy::prelude::{DetectChangesMut, NextState, Query, Res, ResMut, Resource};

use crate::GameState;
use crate::graphics::{CurrentPalette, GBShaderSettings};

#[derive(Eq, PartialEq)]
enum Transition {
    Out(GameState),
    In,
    None,
}

#[derive(Resource, Eq, PartialEq)]
pub struct ScreenTransition {
    transition: Transition,
    clock: usize,
}

impl Default for ScreenTransition {
    fn default() -> Self {
        Self { transition: Transition::None, clock: 0 }
    }
}

impl ScreenTransition {
    pub fn to(state: GameState) -> Self {
        Self { transition: Transition::Out(state), clock: 0 }
    }

    pub fn reveal() -> Self {
        Self { transition: Transition::In, clock: 0 }
    }

    pub fn is_none(&self) -> bool { self.transition == Transition::None }
}

pub fn update(
    mut transition: ResMut<ScreenTransition>,
    mut shader_options: Query<&mut GBShaderSettings>,
    mut game_state: ResMut<NextState<GameState>>,
    palette: Res<CurrentPalette>,
) {
    let Ok(mut shader) = shader_options.get_single_mut() else { return; };
    transition.clock += 1;
    match transition.transition {
        Transition::Out(state) => {
            match transition.clock {
                5 => { shader.color_1 = shader.color_0; },
                11 => { shader.color_2 = shader.color_0; },
                17 => { shader.color_3 = shader.color_0; },
                40 => { game_state.set(state); transition.set_if_neq(ScreenTransition::reveal()); }
                _ => {},
            }
        }
        Transition::In => {
            let colors = palette.0.colors();
            match transition.clock {
                4 => { shader.color_3 = Vec4::from_array(colors[3].as_linear_rgba_f32()); },
                8 => { shader.color_2 = Vec4::from_array(colors[2].as_linear_rgba_f32()); },
                12 => {
                    shader.color_1 = Vec4::from_array(colors[1].as_linear_rgba_f32());
                    transition.set_if_neq(ScreenTransition::default());
                },
                _ => {},
            }
        }
        _ => {}
    }
}