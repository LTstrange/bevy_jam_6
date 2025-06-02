use crate::prelude::*;

mod gameplay;
pub mod level;
mod ui;

const GAME_AREA: Rect = Rect {
    min: Vec2::new(-200.0, -300.0),
    max: Vec2::new(200.0, 300.0),
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, ui::plugin, gameplay::plugin));

    app.insert_resource(PlayerStats {
        attack_energy: 2, // Initial attack energy
    });

    app.add_observer(
        |t: Trigger<ChangePlayerStats>, mut player_stats: ResMut<PlayerStats>| match t.event() {
            ChangePlayerStats::AddAttackEnergy(amount) => {
                player_stats.attack_energy += amount;
                info!("Added attack energy: {}", amount);
            } // ChangePlayerStats::SetDraggableAttacker(value) => {
              //     player_stats.draggable_attacker = *value;
              //     info!("Set draggable attacker: {}", value);
              // }
        },
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct PlayerStats {
    pub attack_energy: u32,
}

#[derive(Event, Debug)]
pub enum ChangePlayerStats {
    AddAttackEnergy(u32),
}
