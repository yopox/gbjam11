use bevy::app::App;
use bevy::math::{vec2, Vec3Swizzles};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::{GameState, util};
use crate::entities::{Blink, MainShip, MuteShots, Ship, Shot, Shots, Weapon};
use crate::graphics::{FakeTransform, Palette, ScreenTransition, StarsSpeed, TextStyles};
use crate::graphics::sizes::Hitbox;
use crate::logic::{Items, ShipBundle, ShipStatus, WaveCleared};
use crate::logic::damage::DamageEvent;
use crate::logic::route::{CurrentRoute, Level, Route, RouteElement};
use crate::logic::upgrades::{ShotUpgrades, Upgrades};
use crate::screens::{Fonts, Textures};
use crate::screens::hangar::SelectedShip;
use crate::screens::text::SimpleText;
use crate::util::{Angle, BORDER, HALF_HEIGHT, HALF_WIDTH, HEIGHT, in_states, space, star_field, WIDTH, z_pos};
use crate::util::hud::HEALTH_BAR_SIZE;

pub struct SpacePlugin;

#[derive(Component)]
struct SpaceUI;

#[derive(Component)]
struct LifeBar;

#[derive(Component)]
struct EliteLifeBar;

#[derive(Component)]
struct CreditsText;

#[derive(Component)]
struct PauseText;

#[derive(Component)]
struct ItemsText;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (update, update_gui, update_life, on_cleared, update_next, update_shield, update_missiles)
                .run_if(in_states(vec![GameState::Space, GameState::Elite, GameState::Boss])),
            )
            .add_systems(PostUpdate, pause
                .run_if(in_states(vec![GameState::Space, GameState::Elite, GameState::Boss]))
            )
            .add_systems(OnEnter(GameState::Space), enter)
            .add_systems(OnEnter(GameState::Elite), enter)
            .add_systems(OnEnter(GameState::Boss), enter)
            .add_systems(OnEnter(GameState::Dummy), on_enter_dummy)
            .add_systems(OnExit(GameState::Space), exit)
            .add_systems(OnExit(GameState::Elite), exit)
            .add_systems(OnExit(GameState::Boss), exit)
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
    state: Res<State<GameState>>,
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
            main_ship_bundle.ship.health / main_ship_bundle.ship.max_health * HEALTH_BAR_SIZE as f32, 1.,
        ))
        .insert(SpaceUI)
    ;

    let state = *state.get();
    if state == GameState::Elite || state == GameState::Boss {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(if state == GameState::Elite { "Elite" } else { "Boss" }, TextStyles::Basic.style(&fonts)),
                text_anchor: Anchor::BottomLeft,
                transform: Transform::from_xyz(8., HEIGHT as f32 - 16., z_pos::GUI),
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
            .insert(FakeTransform::from_xyz_and_scale(
                8., HEIGHT as f32 - 16., z_pos::GUI,
                WIDTH as f32 - 16., 1.,
            ))
            .insert(EliteLifeBar)
            .insert(SpaceUI)
        ;
    }

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
        .spawn(Text2dBundle {
            text: Text::from_section(util::format_credits(ship_status.get_credits()), TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(WIDTH as f32 - 7., 2., z_pos::GUI),
            ..default()
        })
        .insert(CreditsText)
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

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(util::format_items(&ship_status), TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(WIDTH as f32 - 7., 10., z_pos::GUI),
            ..default()
        })
        .insert(ItemsText)
        .insert(SpaceUI)
    ;

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(format!("{}-{}", route.act(), (route.level + 1 - (route.act() - 1) * Route::act_len())), TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomCenter,
            transform: Transform::from_xyz(HALF_WIDTH, 2., z_pos::GUI),
            ..default()
        })
        .insert(SpaceUI)
    ;
}

fn update_gui(
    ship_status: Res<ShipStatus>,
    mut text: Query<&mut Text, With<CreditsText>>,
    mut items: Query<&mut Text, (With<ItemsText>, Without<CreditsText>)>,
) {
    if ship_status.is_changed() {
        text.single_mut().sections[0].value = util::format_credits(ship_status.get_credits());
        items.single_mut().sections[0].value = util::format_items(&ship_status);
    }
}

fn update_life(
    main_ship: Query<&Ship, With<MainShip>>,
    mut bar_transform: Query<&mut FakeTransform, With<LifeBar>>,
    mut damaged: EventReader<DamageEvent>,
    enemies: Query<&Ship, Without<MainShip>>,
    mut elite_bar_transform: Query<&mut FakeTransform, (With<EliteLifeBar>, Without<LifeBar>)>,
) {
    for &DamageEvent { ship, fatal: _ } in damaged.iter() {
        if let Ok(ship) = main_ship.get(ship) {
            bar_transform.single_mut().scale = Some(vec2(
                ship.health / ship.max_health * HEALTH_BAR_SIZE as f32,
                1.,
            ));
        }

        if let Ok(enemy) = enemies.get(ship) {
            if !enemy.model.is_elite() { continue; }
            elite_bar_transform.single_mut().scale = Some(vec2(
                enemy.health / enemy.max_health * (WIDTH as f32 - 16.),
                1.,
            ));
        }
    }
}

#[derive(Component)]
struct Shield(f32);

fn update_shield(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut shield: Query<(Entity, &mut FakeTransform, &mut Shield), Without<MainShip>>,
    player: Query<&FakeTransform, With<MainShip>>,
    mut ship_status: ResMut<ShipStatus>,
    time: Res<Time>,
    textures: Res<Textures>,
) {
    let Ok(player_pos) = player.get_single() else { return; };

    // Update existing shield
    if let Ok((e, mut pos, mut shield)) = shield.get_single_mut() {
        pos.translation.x = player_pos.translation.x;
        if shield.0 > space::BLINK_DURATION && shield.0 - time.delta_seconds() <= space::BLINK_DURATION {
            commands.entity(e).insert(Blink(space::BLINK_DURATION));
        }
        shield.0 -= time.delta_seconds();
        if shield.0 <= 0. { commands.entity(e).despawn_recursive(); }
    } else if keys.just_pressed(KeyCode::Down) {
        if ship_status.remove(&Items::Shield) {
            // Spawn new shield
            commands
                .spawn(SpriteBundle {
                    texture: textures.shield.clone(),
                    ..default()
                })
                .insert(FakeTransform::from_xyz(player_pos.translation.x, player_pos.translation.y + 8., player_pos.translation.y))
                .insert(Hitbox(vec2(16., 2.)))
                .insert(Shield(space::SHIELD_DURATION * if ship_status.has_upgrade(Upgrades::BetterShields) { 2. } else { 1. }))
                .insert(Ship::shield())
                .insert(SpaceUI)
            ;
        }
    }
}

#[derive(Component)]
struct Missile;

fn update_missiles(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    player: Query<(&FakeTransform, &Ship), (With<MainShip>, Without<Missile>)>,
    mut ship_status: ResMut<ShipStatus>,
    mut missiles: Query<&mut FakeTransform, With<Missile>>,
    mut enemies: Query<(&FakeTransform, &Ship), (Without<MainShip>, Without<Missile>)>,
    time: Res<Time>,
    textures: Res<Textures>,
) {
    let Ok((ship_pos, ship)) = player.get_single() else { return; };

    // Spawn missiles
    if keys.just_pressed(KeyCode::Up) && ship_status.remove(&Items::Missile) {
        for offset in [vec2(0., 4.)] {
            let weapon = Weapon::new(Shots::Missile, &ship, offset, Angle(90.));
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: Shots::Missile.sprite_atlas_index(),
                        color: Palette::Greyscale.colors()[2],
                        ..default()
                    },
                    texture_atlas: textures.shots.clone(),
                    ..default()
                })
                .insert(Shot { weapon, friendly: true })
                .insert(Shots::Missile.hitbox())
                .insert(ShotUpgrades(0))
                .insert(FakeTransform::from_xyz(
                    ship_pos.translation.x + weapon.offset.x,
                    ship_pos.translation.y + weapon.offset.y,
                    z_pos::SHOTS,
                ))
                .insert(Missile)
            ;
        }
    }

    // Auto aim
    for mut pos in missiles.iter_mut() {
        if let Some(closest) = enemies
            .iter()
            .filter(|(p, s)| !s.friendly && (p.translation.xy().distance(pos.translation.xy()) as usize) < space::MISSILE_RANGE)
            .map(|(p, _)| p.translation.xy())
            .min_by_key(|p| p.distance(pos.translation.xy()) as usize) {
            pos.translation.x += time.delta_seconds() * space::MISSILE_SPEED * if closest.x > pos.translation.x { 1. } else { -1. };
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
        let mut state = next_state.unwrap().state();
        if state == GameState::Shop && route.are_shopkeepers_angry() { state = GameState::Elite; }
        if state == GameState::Space { state = GameState::Dummy; }
        transition.set_if_neq(ScreenTransition::to(state));
    }
}

fn on_enter_dummy(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Space);
}

fn on_cleared(
    mut commands: Commands,
    mut cleared: EventReader<WaveCleared>,
    mut stars_speed: ResMut<StarsSpeed>,
    state: Res<State<GameState>>,
    ship: Query<Entity, With<MainShip>>,
    route: Res<CurrentRoute>,
    fonts: Res<Fonts>,
    textures: Res<Textures>,
    keys: Res<Input<KeyCode>>,
    mut text: ResMut<SimpleText>,
    mut transition: ResMut<ScreenTransition>,
) {
    let mut force = false;
    if keys.just_pressed(KeyCode::F12) { force = true; }

    if cleared.is_empty() && !force { return; }
    cleared.clear();

    info!("Wave cleared.");

    match state.get() {
        GameState::Elite => {
            text.0 = "Elite defeated!".to_string();
            transition.set_if_neq(ScreenTransition::to(GameState::SimpleText));
            return;
        }
        GameState::Boss => {
            let act = route.act();
            if act < 3 {
                text.0 = format!("Act {} cleared!", act);
                transition.set_if_neq(ScreenTransition::to(GameState::SimpleText));
                return;
            }
        }
        _ => {}
    }

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