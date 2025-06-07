use crate::prelude::*;

mod gameplay;
pub mod level;
mod ui;

#[cfg(feature = "dev")]
pub use ui::inventory::Inventory;

const GAME_AREA: Rect = Rect {
    min: Vec2::new(-200.0, -300.0),
    max: Vec2::new(200.0, 300.0),
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, ui::plugin, gameplay::plugin));

    app.insert_resource(PlayerStats {
        attack_energy: 5.0, // Initial attack energy
    });

    app.add_observer(
        |t: Trigger<ChangePlayerStats>, mut player_stats: ResMut<PlayerStats>| match t.event() {
            ChangePlayerStats::SetAttackEnergy(amount) => {
                player_stats.attack_energy = *amount;
            }
        },
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct PlayerStats {
    pub attack_energy: f32,
}

#[derive(Event, Debug, Clone)]
pub enum ChangePlayerStats {
    SetAttackEnergy(f32),
}
