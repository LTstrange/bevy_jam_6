use crate::prelude::*;

pub mod collect_rate;
pub mod inventory;
pub mod purchase;
mod widget;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((inventory::plugin, purchase::plugin, collect_rate::plugin));
}
