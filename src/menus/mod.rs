//! The game's menus and transitions between them.

mod complete;
mod credits;
mod main;
mod pause;
mod settings;

use bevy::prelude::*;

pub use complete::{COMPLETE_COLLECTION_RATE, CompleteTheGame};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Menu>();

    app.add_plugins((
        credits::plugin,
        main::plugin,
        settings::plugin,
        pause::plugin,
        complete::plugin,
    ));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
    Complete,
}
