use crate::prelude::*;

mod attacker;
mod damage;
mod dust;
mod dust_spawner;
mod health;
mod power;

pub use attacker::SpawnAttacker;
pub use dust_spawner::{SetDustSpawnStats, dust_spawner};
pub use power::{SetPowerStats, power_ui};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        attacker::plugin,
        dust::plugin,
        dust_spawner::plugin,
        damage::plugin,
        health::plugin,
        power::plugin,
    ));
}
