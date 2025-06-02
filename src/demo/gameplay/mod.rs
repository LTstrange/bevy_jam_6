use crate::prelude::*;

mod attacker;
mod damage;
mod dust;
mod dust_spawner;

pub use attacker::attacker;
pub use dust_spawner::dust_spawner;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        attacker::plugin,
        dust::plugin,
        dust_spawner::plugin,
        damage::plugin,
    ));
}
