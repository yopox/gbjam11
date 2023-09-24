use bevy::app::App;
use bevy::prelude::*;

use crate::screens::Sounds;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayBGMEvent>()
            .add_systems(Update, update)
            .add_systems(Startup, setup)
        ;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BGM {
    Title,
    Hangar,
    Space,
    Elite,
    Boss,
    Shop,
    Repair,
}

impl BGM {
    fn source(&self, sounds: &Sounds) -> Handle<AudioSource> {
        match self {
            BGM::Title => sounds.title.clone(),
            BGM::Hangar => sounds.hangar.clone(),
            BGM::Space => sounds.space.clone(),
            BGM::Elite => sounds.elite.clone(),
            BGM::Boss => sounds.boss.clone(),
            BGM::Shop => sounds.shop.clone(),
            BGM::Repair => sounds.repair.clone(),
        }
    }
}

#[derive(Event)]
pub struct PlayBGMEvent(pub BGM);

#[derive(Component)]
struct BGMSource;

#[derive(Resource)]
struct FadeOut(f32, f32, BGM);

impl FadeOut {
    fn to(bgm: BGM) -> Self { FadeOut(0.7, 0.0, bgm) }
}

fn setup(
    mut commands: Commands,
) {
    commands
        .spawn(AudioBundle::default())
        .insert(BGMSource)
    ;
}

fn update(
    mut commands: Commands,
    mut bgm_event: EventReader<PlayBGMEvent>,
    sounds: Option<Res<Sounds>>,
    time: Res<Time>,
    fade_out: Option<ResMut<FadeOut>>,
    mut bgm: Query<(Entity, Option<&mut AudioSink>, &mut Handle<AudioSource>), With<BGMSource>>,
) {
    let Some(sounds) = sounds else { return; };
    let Ok((e, mut sink, mut source)) = bgm.get_single_mut() else { return; };

    for PlayBGMEvent(bgm) in bgm_event.iter() {
        if let Some(ref mut s) = sink {
            if !s.is_paused() { commands.insert_resource(FadeOut::to(*bgm)); }
        } else {
            *source = bgm.source(&sounds);
        }
    }

    if let Some(mut f) = fade_out {
        f.1 += time.delta_seconds();
        let ratio = (1.0 - f.1 / f.0).powi(2);
        if f.0 <= f.1 {
            commands.entity(e).despawn_recursive();
            commands
                .spawn(AudioBundle {
                    source: f.2.source(&sounds).clone(),
                    settings: PlaybackSettings::LOOP,
                })
                .insert(BGMSource)
            ;
            commands.remove_resource::<FadeOut>();
        }
        else { if let Some(s) = sink { s.set_volume(ratio); } }
    }
}