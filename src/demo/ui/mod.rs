use crate::prelude::*;

pub mod inventory;
pub mod purchase;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((inventory::plugin, purchase::plugin));
}
