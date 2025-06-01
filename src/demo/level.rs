//! Spawn the main level.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    AssetsState,
    audio::music,
    demo::{attacker::attacker, inventory::inventory_ui},
    screens::Screen,
    theme::widget::{self},
};

use super::dust_spawner::dust_spawner;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::Loading).load_collection::<LevelAssets>(),
    );

    app.register_type::<LevelAssets>();
}

#[derive(Resource, AssetCollection, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    music: Handle<AudioSource>,
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    info!("Spawn level");
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            (
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            ),
            dust_spawner(),
            attacker(Vec2::new(0.0, 0.0), 1.0),
        ],
    ));

    // ingame ui
    commands.spawn((
        Name::new("Ingame UI"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        StateScoped(Screen::Gameplay),
        GlobalZIndex(1),
        children![inventory_ui()],
    ));
}
