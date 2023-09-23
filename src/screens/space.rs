use bevy::app::App;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{GameState, util};
use crate::entities::{MainShip, MuteShots, Ship, Shot};
use crate::graphics::{FakeTransform, ScreenTransition, StarsSpeed, TextStyles};
use crate::graphics::sizes::Hitbox;
use crate::logic::{ShipBundle, ShipStatus, WaveCleared};
use crate::logic::damage::DamageEvent;
use crate::logic::route::{CurrentRoute, Level, RouteElement};
use crate::logic::upgrades::ShotUpgrades;
use crate::screens::{Fonts, Textures};
use crate::screens::hangar::SelectedShip;
use crate::util::{BORDER, HALF_HEIGHT, HALF_WIDTH, HEIGHT, space, star_field, WIDTH, z_pos};
use crate::util::hud::HEALTH_BAR_SIZE;

pub struct SpacePlugin;

#[derive(Component)]
struct SpaceUI;

#[derive(Component)]
struct LifeBar;

#[derive(Component)]
struct CreditsText;

#[derive(Component)]
struct PauseText;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (update, update_gui, update_life, on_cleared, update_next)
                .run_if(in_state(GameState::Space)),
            )
            .add_systems(PostUpdate, pause)
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnExit(GameState::Space), exit)
        ;
    }
}

fn update(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ship: Query<(&Ship, &Hitbox, &mut FakeTransform), Without<Rush>>,
) {
    for (s, hitbox, mut pos) in ship.iter_mut() {
        if !s.friendly { continue; }

        let hitbox_w = hitbox.0.x;
        let movement_x = s.speed * time.delta_seconds();
        let dx = movement_x + hitbox_w / 2. + BORDER;
        if keys.pressed(KeyCode::Left) {
            if pos.translation.x - dx >= 0. { pos.translation.x -= movement_x; }
        }
        if keys.pressed(KeyCode::Right) {
            if pos.translation.x + dx <= WIDTH as f32 { pos.translation.x += movement_x; }
        }
    }
}

fn enter(
    mut commands: Commands,
    selected_ship: Res<SelectedShip>,
    ship_status: Res<ShipStatus>,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
    route: Res<CurrentRoute>,
    mut stars_speed: ResMut<StarsSpeed>,
    mut time: ResMut<Time>,
) {
    stars_speed.set_by_level(route.level);
    time.set_relative_speed(space::time_ratio(route.level));

    let mut main_ship_bundle = ShipBundle::from(
        textures.ship.clone(),
        selected_ship.0.model(),
        vec2(HALF_WIDTH, 24.),
    );
    main_ship_bundle.ship.health = ship_status.health().0;
    main_ship_bundle.ship.max_health = ship_status.health().1;
    main_ship_bundle.ship.speed *= ship_status.speed_multiplier();
    main_ship_bundle.ship.damage_factor *= ship_status.damage_multiplier();
    main_ship_bundle.ship.shot_speed *= ship_status.shot_speed_multiplier();
    main_ship_bundle.ship.shot_frequency *= ship_status.shot_frequency_multiplier();

    commands
        .spawn(main_ship_bundle)
        .insert(MainShip)
        .insert(ShotUpgrades(ship_status.shot_upgrades()))
        .insert(SpaceUI)
    ;

    // GUI
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Life", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomLeft,
            transform: Transform::from_xyz(8., 4., z_pos::GUI),
            ..default()
        })
        .insert(SpaceUI)
    ;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: textures.bar.clone(),
            ..default()
        })
        .insert(LifeBar)
        .insert(FakeTransform::from_xyz_and_scale(
            8., 4., z_pos::GUI,
            HEALTH_BAR_SIZE as f32, 1.,
        ))
        .insert(SpaceUI)
    ;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(util::format_credits(ship_status.get_credits()), TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(WIDTH as f32 - 7., 4., z_pos::GUI),
            ..default()
        })
        .insert(CreditsText)
        .insert(SpaceUI)
    ;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomRight,
                ..default()
            },
            texture: textures.bar.clone(),
            transform: Transform {
                translation: vec3(WIDTH as f32 - 8., 4., z_pos::GUI),
                scale: vec3(55., 1., 1.),
                ..default()
            },
            ..default()
        })
        .insert(SpaceUI)
    ;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("Pause", TextStyles::Basic.style(&fonts)),
            transform: Transform::from_xyz(HALF_WIDTH, HALF_HEIGHT, z_pos::PAUSE),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(PauseText)
        .insert(SpaceUI)
    ;
}

fn update_gui(
    ship_status: Res<ShipStatus>,
    mut text: Query<&mut Text, With<CreditsText>>,
) {
    if ship_status.is_changed() {
        text.single_mut().sections[0].value = util::format_credits(ship_status.get_credits());
    }
}

fn update_life(
    ships: Query<&Ship, With<MainShip>>,
    mut bar_transform: Query<&mut FakeTransform, With<LifeBar>>,
    mut damaged: EventReader<DamageEvent>,
) {
    for &DamageEvent { ship, fatal: _ } in damaged.iter() {
        if let Ok(ship) = ships.get(ship) {
            bar_transform.single_mut().scale = Some(Vec2::new(
                ship.health / ship.max_health * HEALTH_BAR_SIZE as f32,
                1.,
            ))
        }
    }
}

#[derive(Eq, PartialEq)]
enum Position { Left, Center, Right }

#[derive(Component)]
struct NextLevelOption(Position, Level);

#[derive(Component)]
struct NextLevelSelectionSprite;

#[derive(Component)]
struct Rush;

fn update_next(
    mut commands: Commands,
    time: Res<Time>,
    mut route: ResMut<CurrentRoute>,
    mut ship: Query<(Entity, &mut FakeTransform, Option<&mut Rush>), (With<MainShip>, Without<NextLevelSelectionSprite>)>,
    mut next: Query<(&NextLevelOption, &mut FakeTransform, &mut Text), (Without<MainShip>, Without<NextLevelSelectionSprite>)>,
    mut bars: Query<&mut FakeTransform, (Without<MainShip>, With<NextLevelSelectionSprite>)>,
    mut stars_speed: ResMut<StarsSpeed>,
    mut transition: ResMut<ScreenTransition>,
    fonts: Res<Fonts>,
) {
    let Ok(mut bars_pos) = bars.get_single_mut() else { return; };
    let Ok((e, mut ship_pos, rush)) = ship.get_single_mut() else { return; };

    let mut do_transition = false;
    let mut next_state: Option<Level> = None;

    let bars_y = bars_pos.translation.y;
    let dy = space::NEXT_LEVEL_SPEED_Y * time.delta_seconds() * if rush.is_some() { 0. } else { 1. };
    if rush.is_some() {
        // Update ship
        let ship_y = ship_pos.translation.y;
        if ship_y > HEIGHT as f32 + 64. && transition.is_none() {
            // Transition to next state
            route.advance();
            do_transition = true;
        }
        ship_pos.translation.y += space::RUSH_SPEED_Y * time.delta_seconds();
    } else if bars_y > space::NEXT_LEVEL_CHOICE_Y && bars_y + dy <= space::NEXT_LEVEL_CHOICE_Y {
        // Ship starts rushing
        commands.entity(e).insert(Rush);
        stars_speed.0 = star_field::RUSH_SPEED;
    }
    bars_pos.translation.y += dy;

    for (option, mut pos, mut text) in next.iter_mut() {
        pos.translation.y += dy;

        if option.0 == Position::Left {
            if ship_pos.translation.x <= HALF_WIDTH {
                text.sections[0].style = TextStyles::Basic.style(&fonts);
                bars_pos.translation.x = WIDTH as f32 / 4.;
                next_state = Some(option.1);
            } else {
                text.sections[0].style = TextStyles::Gray.style(&fonts);
            }
        }

        if option.0 == Position::Right {
            if ship_pos.translation.x > HALF_WIDTH {
                text.sections[0].style = TextStyles::Basic.style(&fonts);
                bars_pos.translation.x = WIDTH as f32 / 4. * 3.;
                next_state = Some(option.1);
            } else {
                text.sections[0].style = TextStyles::Gray.style(&fonts);
            }
        }

        if option.0 == Position::Center {
            next_state = Some(option.1);
        }
    }

    if do_transition && next_state.is_some() {
        transition.set_if_neq(ScreenTransition::to(next_state.unwrap().state()));
    }
}

fn on_cleared(
    mut commands: Commands,
    mut cleared: EventReader<WaveCleared>,
    mut stars_speed: ResMut<StarsSpeed>,
    ship: Query<Entity, With<MainShip>>,
    route: Res<CurrentRoute>,
    fonts: Res<Fonts>,
    textures: Res<Textures>,
    keys: Res<Input<KeyCode>>,
) {
    let mut force = false;
    if keys.just_pressed(KeyCode::F12) {force = true; }

    if cleared.is_empty() && !force { return; }
    cleared.clear();

    info!("Wave cleared.");

    stars_speed.0.x /= 4.;
    stars_speed.0.y /= 4.;

    // Mute ship shots
    if let Ok(e) = ship.get_single() {
        commands.entity(e).insert(MuteShots);
    }

    // Spawn next level options
    for (name, x, position, level) in match route.route.0[route.level + 1] {
        RouteElement::Level(l) => vec![
            (l.name().to_owned(), HALF_WIDTH, Position::Center, l),
        ],
        RouteElement::Choice(l1, l2) => vec![
            (l1.name().to_owned(), WIDTH as f32 / 4., Position::Left, l1),
            (l2.name().to_owned(), WIDTH as f32 / 4. * 3., Position::Right, l2),
        ]
    } {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(name, TextStyles::Basic.style(&fonts)),
                text_anchor: Anchor::Center,
                ..default()
            })
            .insert(FakeTransform::from_xyz(x, HEIGHT as f32 + 16., z_pos::GUI))
            .insert(NextLevelOption(position, level))
            .insert(SpaceUI)
        ;
    }

    commands
        .spawn(SpriteBundle {
            texture: textures.option_bars.clone(),
            ..default()
        })
        .insert(FakeTransform::from_xyz(HALF_WIDTH, HEIGHT as f32 + 16., z_pos::GUI))
        .insert(NextLevelSelectionSprite)
        .insert(SpaceUI)
    ;
}

fn pause(
    mut time: ResMut<Time>,
    mut pause: Query<&mut Visibility, With<PauseText>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let Ok(mut pause) = pause.get_single_mut() else { return; };
        if time.is_paused() {
            time.unpause();
            pause.set_if_neq(Visibility::Hidden);
        } else {
            time.pause();
            pause.set_if_neq(Visibility::Inherited);
        }
    }
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, Or<(With<SpaceUI>, With<Ship>, With<Shot>)>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}