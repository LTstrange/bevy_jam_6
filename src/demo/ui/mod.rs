use crate::prelude::*;

pub mod inventory;
pub mod purchase;
mod widget;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((inventory::plugin, purchase::plugin));
}
