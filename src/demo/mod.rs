use crate::prelude::*;

mod gameplay;
pub mod level;
mod ui;

const GAME_AREA: Vec2 = Vec2::new(400.0, 600.0);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, ui::plugin, gameplay::plugin));

    app.insert_resource(PlayerStats {
        attack_energy: 5.0,        // Initial attack energy
        draggable_attacker: false, // Whether the attacker can be dragged
    });

    app.add_observer(
        |t: Trigger<ChangePlayerStats>, mut player_stats: ResMut<PlayerStats>| match t.event() {
            ChangePlayerStats::AddAttackEnergy(amount) => {
                player_stats.attack_energy += amount;
                info!("Added attack energy: {}", amount);
            }
            ChangePlayerStats::SetDraggableAttacker(value) => {
                player_stats.draggable_attacker = *value;
                info!("Set draggable attacker: {}", value);
            }
        },
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct PlayerStats {
    pub attack_energy: f32,
    pub draggable_attacker: bool,
}

#[derive(Event, Debug)]
pub enum ChangePlayerStats {
    AddAttackEnergy(f32),
    SetDraggableAttacker(bool),
}
