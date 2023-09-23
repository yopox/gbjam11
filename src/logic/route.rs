use bevy::log::error;
use bevy::prelude::Resource;
use rand::{Rng, RngCore, thread_rng};

use crate::GameState;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Level {
    /// Regular fight, see [crate::logic::wave]
    Space,
    /// Choice between 2 upgrades
    Upgrade,
    /// Boss fight
    Boss,

    /// Shop with random options (shield / missile / upgrade) + option to repair ship
    Shop,
    /// Free repair (50% HP)
    Repair,
    /// Fight with a strong enemy, loots a good amount of credits and some bombs / shields
    Elite,
    /// Can be anything except [Level::Boss]
    Unknown,

    Win,
}

impl Level {
    pub fn name(&self) -> &str {
        match self {
            Level::Space => "Fight",
            Level::Upgrade => "Upgrade station",
            Level::Boss => "Boss",
            Level::Shop => "Shop",
            Level::Repair => "Repair station",
            Level::Elite => "Elite",
            Level::Unknown => "???",
            Level::Win => "You won!",
        }
    }

    pub fn state(&self) -> GameState {
        match self {
            Level::Space
            | Level::Elite
            | Level::Boss => GameState::Space,
            Level::Shop => GameState::Shop,
            Level::Upgrade => GameState::Upgrade,
            Level::Repair => GameState::Repair,
            Level::Unknown => Level::unknown().state(),
            Level::Win => GameState::GameOver,
        }
    }

    fn random() -> Self {
        let mut rng = thread_rng();
        let options = [
            Level::Unknown,
            Level::Unknown,
            Level::Unknown,
            Level::Unknown,
            Level::Unknown,
            Level::Shop,
            Level::Shop,
            Level::Shop,
            Level::Shop,
            Level::Repair,
            Level::Repair,
            Level::Elite,
            Level::Elite,
            Level::Upgrade,
        ];
        return options[rng.gen_range(0..options.len())]
    }

    fn unknown() -> Self {
        let mut rng = thread_rng();
        let options = [
            Level::Space,
            Level::Space,
            Level::Space,
            Level::Elite,
            Level::Shop,
            Level::Shop,
            Level::Repair,
            Level::Repair,
            Level::Upgrade,
        ];
        return options[rng.gen_range(0..options.len())]
    }
}

#[derive(Debug)]
pub enum RouteElement {
    Level(Level),
    Choice(Level, Level),
}

impl RouteElement {
    pub fn state(&self) -> GameState {
        match self {
            RouteElement::Level(l) => l.state(),
            RouteElement::Choice(_, _) => {
                error!("Next RouteElement shouldn't be a choice.");
                GameState::Space
            },
        }
    }

    fn choice() -> Self {
        let l1 = Level::random();
        let mut l2 = l1;
        while l2 == l1 { l2 = Level::random(); }
        RouteElement::Choice(l1, l2)
    }

    fn choice_with(l1: Level) -> Self {
        let mut l2 = l1;
        while l2 == l1 { l2 = Level::random(); }
        if thread_rng().next_u32() % 2 == 1 { RouteElement::Choice(l1, l2) }
        else { RouteElement::Choice(l2, l1) }
    }
}

#[derive(Debug)]
pub struct Route(pub [RouteElement; 28]);

impl Route {
    fn new() -> Self {
        let mut route = vec![];
        for i in 0..27 {
            let element = match i % 9 {
                3 => RouteElement::Level(Level::Upgrade),
                7 => RouteElement::choice_with(Level::Repair),
                8 => RouteElement::Level(Level::Boss),
                i if i % 2 == 0 => RouteElement::Level(Level::Space),
                _ => RouteElement::choice(),
            };
            route.push(element);
        }
        route.push(RouteElement::Level(Level::Win));
        return Route(route.try_into().expect(""))
    }
}

#[derive(Resource, Debug)]
pub struct CurrentRoute {
    pub route: Route,
    pub level: usize,
    angry_shopkeepers: bool,
}

impl CurrentRoute {
    pub fn new() -> Self {
        CurrentRoute { route: Route::new(), level: 0, angry_shopkeepers: false }
    }

    pub fn advance(&mut self) { self.level += 1; }

    pub fn state(&self) -> GameState {
        if self.level >= self.route.0.len() { return GameState::Hangar; }

        let s = self.route.0[self.level].state();
        if self.angry_shopkeepers && s == GameState::Shop { GameState::Elite } else { s }
    }

    pub fn win(&self) -> bool { self.level == self.route.0.len() - 1 }

    pub fn set_angry_shopkeepers(&mut self, angry: bool) { self.angry_shopkeepers = angry; }
    pub fn are_shopkeepers_angry(&mut self) -> bool { self.angry_shopkeepers }
}

#[test]
fn show_route() {
    let route = CurrentRoute::new();
    for (i, element) in route.route.0.iter().enumerate() {
        if i % 9 == 0 { println!(); }
        println!("{} – {:?}", i, element);
    }
}