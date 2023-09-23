use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::{RngCore, thread_rng};

use crate::{GameState, util};
use crate::graphics::{ScreenTransition, StarsSpeed, TextStyles};
use crate::logic::{Items, ShipStatus};
use crate::logic::route::CurrentRoute;
use crate::screens::{Fonts, Textures};
use crate::util::{shop, z_pos};

pub struct ShopPlugin;

#[derive(Component)]
struct ShopUI;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(GameState::Shop)))
            .add_systems(OnEnter(GameState::Shop), enter)
            .add_systems(OnExit(GameState::Shop), exit)
        ;
    }
}

fn update(
    mut text: Query<&mut Text, With<CreditsText>>,
    mut item_texts: Query<(&mut Text, &ShopOption), (Without<CreditsText>)>,
    mut ship_status: ResMut<ShipStatus>,
    keys: Res<Input<KeyCode>>,
    mut options: ResMut<Select<ShopOption>>,
    mut dot: Query<&mut Transform, With<SelectionDot>>,
    mut transition: ResMut<ScreenTransition>,
    mut route: ResMut<CurrentRoute>,
    fonts: Res<Fonts>,
) {
    // Update credits text
    let Ok(mut text) = text.get_single_mut() else { return; };
    let Ok(mut dot_pos) = dot.get_single_mut() else { return; };

    text.sections[0].value = util::format_credits(ship_status.get_credits());

    // Select previous / next option
    if keys.just_pressed(KeyCode::Up) {
        options.selected = (options.items.len() + options.selected - 1) % options.items.len();
        dot_pos.translation.x = options.items[options.selected].0.x - 2.;
        dot_pos.translation.y = options.items[options.selected].0.y - 1.;
    } else if keys.just_pressed(KeyCode::Down) {
        options.selected = (options.selected + 1) % options.items.len();
        dot_pos.translation.x = options.items[options.selected].0.x - 2.;
        dot_pos.translation.y = options.items[options.selected].0.y - 1.;
    }

    // Buy
    if keys.just_pressed(KeyCode::Space) {
        match options.items[options.selected].1 {
            ShopOption::Buy(item, sale) => {
                let price = shop::item_price(&item, sale);
                if !(item == Items::Repair && ship_status.is_max_health()) {
                    // Buy item
                    ship_status.buy(price);
                    ship_status.add(&item);
                }
            }
            ShopOption::Sell(item) => {
                if ship_status.remove(&item) {
                    // Sell item
                    ship_status.add_credits(shop::item_price(&item, true));
                }
            }
            ShopOption::Exit => {
                route.advance();
                transition.set_if_neq(ScreenTransition::to(route.state()));
            }
        }
    }

    // Update item texts
    for (mut item_text, option) in item_texts.iter_mut() {
        item_text.sections[0].value = option.text(&ship_status);
        match option {
            ShopOption::Buy(item, sale) => {
                let price = shop::item_price(item, *sale);
                item_text.sections[0].style = if price > ship_status.get_credits() { TextStyles::Accent.style(&fonts) } else { TextStyles::Basic.style(&fonts) };
                if *item == Items::Repair && ship_status.is_max_health() { item_text.sections[0].style = TextStyles::Accent.style(&fonts); }
            }
            ShopOption::Sell(item) => {
                let amount = ship_status.get(item);
                item_text.sections[0].style = if amount == 0 { TextStyles::Accent.style(&fonts) } else { TextStyles::Basic.style(&fonts) };
            }
            ShopOption::Exit => {}
        }
    }
}

#[derive(Component)]
struct CreditsText;

#[derive(Copy, Clone, Eq, PartialEq, Component)]
enum ShopOption {
    Buy(Items, bool),
    Sell(Items),
    Exit,
}

impl ShopOption {
    fn text(&self, ship_status: &ShipStatus) -> String {
        match self {
            ShopOption::Buy(item, sale) if *item == Items::Repair => format!(
                "[{}]{} - {} ({}/{})",
                shop::item_price(&Items::Repair, *sale), if *sale { "!" } else { "" }, item.name(),
                ship_status.health().0, ship_status.health().1
            ),
            ShopOption::Buy(item, sale) => format!(
                "[{}]{} - {}",
                shop::item_price(item, *sale), if *sale { "!" } else { "" }, item.name()
            ),
            ShopOption::Sell(item) => format!(
                "[{}] - {} ({})",
                shop::item_price(item, true), item.name(), ship_status.get(item)
            ),
            ShopOption::Exit => "EXIT".to_string(),
        }
    }
}

#[derive(Resource)]
pub struct Select<T> {
    pub items: Vec<(Vec2, T)>,
    pub selected: usize,
}

#[derive(Component)]
struct SelectionDot;

fn enter(
    mut commands: Commands,
    textures: Res<Textures>,
    fonts: Res<Fonts>,
    ship_status: Res<ShipStatus>,
    route: Res<CurrentRoute>,
    mut star_field: ResMut<StarsSpeed>,
) {
    star_field.set_by_level(route.level);

    let mut rng = thread_rng();
    let mut is_sale = || rng.next_u32() % 10 == 0;

    // Generate shop options
    let options = vec![
        (vec2(32., 89.), ShopOption::Buy(Items::random_upgrade(), is_sale())),
        (vec2(32., 77.), ShopOption::Buy(Items::Missile, is_sale())),
        (vec2(32., 65.), ShopOption::Buy(Items::Shield, is_sale())),
        (vec2(32., 53.), ShopOption::Buy(Items::Repair, is_sale())),
        (vec2(32., 28.), ShopOption::Sell(Items::random_collectible())),
        (vec2(32., 8.), ShopOption::Exit),
    ];

    // Spawn shop UI
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..default()
            },
            texture: textures.shop_bg.clone(),
            transform: Transform::from_xyz(0., 0., z_pos::SHOP),
            ..default()
        })
        .insert(ShopUI)
    ;

    // Spawn fixed text
    for (text, x, y) in [
        ("---SHOP---", 56., 124.),
        ("BUY", 24., 100.),
        ("SELL", 24., 40.),
    ] {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(text, TextStyles::Basic.style(&fonts)),
                text_anchor: Anchor::BottomLeft,
                transform: Transform::from_xyz(x, y - 4., z_pos::SHOP_TEXT),
                ..default()
            })
            .insert(ShopUI)
        ;
    }

    // Spawn options
    for (pos, option) in options.iter() {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(option.text(&ship_status), TextStyles::Basic.style(&fonts)),
                text_anchor: Anchor::BottomLeft,
                transform: Transform::from_xyz(pos.x, pos.y - 4., z_pos::SHOP_TEXT),
                ..default()
            })
            .insert(option.clone())
            .insert(ShopUI)
        ;
    }

    // Spawn dot
    commands
        .spawn(SpriteBundle {
            texture: textures.dot.clone(),
            sprite: Sprite {
                anchor: Anchor::BottomRight,
                ..default()
            },
            transform: Transform::from_xyz(options[0].0.x - 2., options[0].0.y - 1., z_pos::SHOP_TEXT),
            ..default()
        })
        .insert(SelectionDot)
        .insert(ShopUI)
    ;

    // Spawn credits text
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("", TextStyles::Basic.style(&fonts)),
            text_anchor: Anchor::BottomRight,
            transform: Transform::from_xyz(129., 8. - 4., z_pos::SHOP_TEXT),
            ..default()
        })
        .insert(CreditsText)
        .insert(ShopUI)
    ;

    commands.insert_resource(Select {
        items: options,
        selected: 0,
    });
}

fn exit(
    mut commands: Commands,
    to_clean: Query<Entity, With<ShopUI>>,
) {
    for id in to_clean.iter() {
        commands
            .entity(id)
            .despawn_recursive();
    }
}