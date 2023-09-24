use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::entities::{MuteShots, Ship, Ships, Shot};
use crate::GameState;
use crate::graphics::{CurrentPalette, Palette, ScreenTransition, StarsSpeed, TextStyles};
use crate::logic::route::CurrentRoute;
use crate::logic::ShipBundle;
use crate::screens::{Fonts, Textures};
use crate::util::{star_field, z_pos};

pub struct HangarPlugin;

#[derive(Component)]
struct HangarUI;

impl Plugin for HangarPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectedShip(PlayableShips::Ship1))
            .add_event::<UpdateGUI>()
            .add_systems(Update, (update, update_text, update_shooting)
                .run_if(in_state(GameState::Hangar))
            )
            .add_systems(OnEnter(GameState::Hangar), enter)
            .add_systems(OnExit(GameState::Hangar), exit)
        ;
    }
}

#[derive(Resource)]
pub struct SelectedShip(pub PlayableShips);

#[derive(Component)]
struct Legend;

#[derive(Component)]
struct ShipName;

#[derive(Component)]
struct ShipDescription(u8);

#[derive(Event)]
struct UpdateGUI;

#[derive(Copy, Clone)]
pub enum PlayableShips {
    Ship1,
    Ship2,
    Ship3,
    Ship4,
}

impl PlayableShips {
    fn name(&self) -> &str {
        match self {
            PlayableShips::Ship1 => "Starfight",
            PlayableShips::Ship2 => "ShuttleNight",
            PlayableShips::Ship3 => "ArmyNation",
            PlayableShips::Ship4 => "Ranger-3B",
        }
    }

    fn description(&self) -> (&str, &str) {
        match self {
            PlayableShips::Ship1 => ("Dual shots", "Balanced"),
            PlayableShips::Ship2 => ("Large shots", "Health +"),
            PlayableShips::Ship3 => ("Triple shots", "Speed +"),
            PlayableShips::Ship4 => ("Laser", "Health -"),
        }
    }

    pub(crate) fn model(&self) -> Ships {
        match self {
            PlayableShips::Ship1 => Ships::Player(0),
            PlayableShips::Ship2 => Ships::Player(1),
            PlayableShips::Ship3 => Ships::Player(2),
            PlayableShips::Ship4 => Ships::Player(3),
        }
    }

    fn palette(&self) -> Palette {
        match self {
            PlayableShips::Ship1 => Palette::YellowPurple,
            PlayableShips::Ship2 => Palette::Greyscale,
            PlayableShips::Ship3 => Palette::GameBoy,
            PlayableShips::Ship4 => Palette::YellowPurple,
        }
    }

    fn next(&self) -> Self {
        match self {
            PlayableShips::Ship1 => PlayableShips::Ship2,
            PlayableShips::Ship2 => PlayableShips::Ship3,
            PlayableShips::Ship3 => PlayableShips::Ship4,
            PlayableShips::Ship4 => PlayableShips::Ship1,
        }
    }

    fn previous(&self) -> Self {
        match self {
            PlayableShips::Ship1 => PlayableShips::Ship4,
            PlayableShips::Ship2 => PlayableShips::Ship1,
            PlayableShips::Ship3 => PlayableShips::Ship2,
            PlayableShips::Ship4 => PlayableShips::Ship3,
        }
    }
}

fn update(
    mut commands: Commands,
    mut transition: ResMut<ScreenTransition>,
    mut selection: ResMut<SelectedShip>,
    mut update_gui: EventWriter<UpdateGUI>,
    keys: Res<Input<KeyCode>>,
) {
    if !transition.is_none() { return; }

    if keys.just_pressed(KeyCode::Left) {
        selection.0 = selection.0.previous();
        update_gui.send(UpdateGUI);
    }

    if keys.just_pressed(KeyCode::Right) {
        selection.0 = selection.0.next();
        update_gui.send(UpdateGUI);
    }

    if keys.just_pressed(KeyCode::Space) {
        let route = CurrentRoute::new();
        transition.set_if_neq(ScreenTransition::to(route.state()));
        commands.insert_resource(route);
    }
}

fn enter(
    mut commands: Commands,
    mut star_speed: ResMut<StarsSpeed>,
    mut update_gui: EventWriter<UpdateGUI>,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
) {
    star_speed.0 = star_field::HANGAR_SPEED;
    update_gui.send(UpdateGUI);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: textures.hangar.clone(),
            transform: Transform::from_xyz(0., 0., z_pos::HANGAR),
            ..default()
        })
        .insert(HangarUI)
    ;

    for (ship, x) in [
        (PlayableShips::Ship1, 44.),
        (PlayableShips::Ship2, 44. + 24.),
        (PlayableShips::Ship3, 44. + 24. * 2.),
        (PlayableShips::Ship4, 44. + 24. * 3.),
    ] {
        commands
            .spawn(ShipBundle::from(textures.ship.clone(), ship.model(), vec2(x, 50.)))
            .insert(HangarUI)
        ;
    }

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..default()
            },
            texture: textures.legend.clone(),
            transform: Transform::from_xyz(44., 44., z_pos::HANGAR_TEXT),
            ..default()
        })
        .insert(Legend)
        .insert(HangarUI)
    ;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(44. + 81., 44. - 24. + 2., z_pos::HANGAR_TEXT),
            ..default()
        })
        .insert(ShipName)
        .insert(HangarUI)
    ;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(44. + 81., 44. - 24. + 2. - 10., z_pos::HANGAR_TEXT),
            ..default()
        })
        .insert(ShipDescription(1))
        .insert(HangarUI)
    ;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(44. + 81., 44. - 24. + 2. - 18., z_pos::HANGAR_TEXT),
            ..default()
        })
        .insert(ShipDescription(2))
        .insert(HangarUI)
    ;
}

fn update_text(
    selected_ship: Res<SelectedShip>,
    mut ev: EventReader<UpdateGUI>,
    mut palette: ResMut<CurrentPalette>,
    mut legend: Query<(&mut Sprite, &mut Transform), (With<Legend>, Without<ShipName>, Without<ShipDescription>)>,
    mut name: Query<(&mut Text, &mut Anchor, &mut Transform), (With<ShipName>, Without<ShipDescription>)>,
    mut description: Query<(&mut Text, &ShipDescription, &mut Anchor, &mut Transform), (With<ShipDescription>, Without<ShipName>)>,
) {
    if ev.is_empty() { return; }
    ev.clear();

    let ship = selected_ship.0;

    // Update palette
    palette.0 = ship.palette();

    // Update legend line
    let (mut legend_sprite, mut legend_pos) = legend.single_mut();
    let (anchor, x, flip) = match ship {
        PlayableShips::Ship1 => (Anchor::TopLeft, 44., false),
        PlayableShips::Ship2 => (Anchor::TopLeft, 44. + 24., false),
        PlayableShips::Ship3 => (Anchor::TopRight, 44. + 24. * 2., true),
        PlayableShips::Ship4 => (Anchor::TopRight, 44. + 24. * 3., true),
    };
    legend_sprite.anchor = anchor;
    legend_sprite.flip_x = flip;
    legend_pos.translation.x = x;

    // Text pos
    let (anchor, x) = match ship {
        PlayableShips::Ship1 => (Anchor::BottomRight, 44. + 81.),
        PlayableShips::Ship2 => (Anchor::BottomRight, 44. + 24. + 81.),
        PlayableShips::Ship3 => (Anchor::BottomLeft, 44. + 24. * 2. - 80.),
        PlayableShips::Ship4 => (Anchor::BottomLeft, 44. + 24. * 3. - 80.),
    };

    // Update ship name
    let (mut name_text, mut text_anchor, mut text_pos) = name.single_mut();
    name_text.sections[0].value = ship.name().to_string();
    *text_anchor = anchor.clone();
    text_pos.translation.x = x;

    // Update description
    let (line_1, line_2) = ship.description();
    for (mut description_text, info, mut text_anchor, mut text_pos) in description.iter_mut() {
        match info.0 {
            1 => description_text.sections[0].value = line_1.to_string(),
            2 => description_text.sections[0].value = line_2.to_string(),
            _ => {}
        }
        *text_anchor = anchor.clone();
        text_pos.translation.x = x;
    }
}

fn update_shooting(
    mut commands: Commands,
    selected_ship: Res<SelectedShip>,
    mut ev: EventReader<UpdateGUI>,
    ships: Query<(Entity, &Ship)>
) {
    if ev.is_empty() { return; }
    ev.clear();

    for (e, ship) in &ships {
        if ship.model == selected_ship.0.model() { commands.entity(e).remove::<MuteShots>(); }
        else { commands.entity(e).insert(MuteShots); }
    }
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, Or<(With<HangarUI>, With<Shot>)>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}