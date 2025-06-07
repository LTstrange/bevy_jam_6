//! Development tools for the game. This plugin is only enabled in dev builds.

#![allow(unused_imports)]

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{demo::Inventory, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // // inspector egui
    // app.add_plugins(EguiPlugin {
    //     enable_multipass_for_primary_context: true,
    // })
    // .add_plugins(WorldInspectorPlugin::new());

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        (
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
            dev_add_dust_data
                .run_if(input_just_pressed(DEV_ADD_DUST).and(in_state(Screen::Gameplay))),
        ),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;
const DEV_ADD_DUST: KeyCode = KeyCode::F1;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn dev_add_dust_data(mut inventory: ResMut<Inventory>) {
    inventory.dust_data += 100;
    info!("Added 100 dust. Current balance: {}", inventory.dust_data);
}
