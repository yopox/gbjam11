pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub const SCALE: f32 = 4.;

/// Min distance between the player and the screen border
pub const BORDER: f32 = 2.;

pub mod z_pos {
    pub const STAR_FIELD: f32 = 10.;
    pub const GUI: f32 = 20.;
    pub const SHIPS: f32 = 30.;
    pub const SHOTS: f32 = 31.;
}