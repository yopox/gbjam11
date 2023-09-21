use bevy::prelude::Resource;
use rand::{Rng, RngCore, thread_rng};

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
}

impl Level {
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
pub struct Route(pub [RouteElement; 27]);

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
        return Route(route.try_into().expect(""))
    }
}

#[derive(Resource, Debug)]
pub struct CurrentRoute {
    pub route: Route,
    pub level: usize,
}

impl CurrentRoute {
    pub fn new() -> Self {
        CurrentRoute { route: Route::new(), level: 0 }
    }
}

#[test]
fn show_route() {
    let route = CurrentRoute::new();
    for (i, element) in route.route.0.iter().enumerate() {
        if i % 9 == 0 { println!(); }
        println!("{} – {:?}", i, element);
    }
}