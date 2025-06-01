//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod attacker;
mod dust;
mod dust_spawner;
mod inventory;
pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        dust::plugin,
        dust_spawner::plugin,
        attacker::plugin,
        inventory::plugin,
    ));
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct GameplayState {
    attack_energy: f32,
}
