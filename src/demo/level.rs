//! Spawn the main level.

use crate::prelude::*;

use crate::{AssetsState, audio::music, screens::Screen};

use super::gameplay::*;
use super::ui::{inventory::inventory_ui, purchase::purchase_ui};

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
        ],
    ));
    // commands.trigger(SpawnAttacker);

    // ingame ui
    commands.spawn((
        Name::new("Ingame UI"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        StateScoped(Screen::Gameplay),
        GlobalZIndex(1),
        Pickable::IGNORE,
        children![inventory_ui(), purchase_ui()],
    ));
}
