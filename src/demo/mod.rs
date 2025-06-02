//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod gameplay;
pub mod level;
mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, ui::plugin, gameplay::plugin));

    app.insert_resource(PlayerStats {
        attack_energy: 5.0, // Initial attack energy
    });
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct PlayerStats {
    attack_energy: f32,
}
