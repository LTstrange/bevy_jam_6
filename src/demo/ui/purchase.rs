use crate::prelude::*;
use crate::theme::widget::{button_base, label};
use bevy::ecs::system::IntoObserverSystem;
use bevy::ui::Val::*;

use super::{super::PlayerStats, inventory};

pub(super) fn plugin(app: &mut App) {}

pub fn purchase_ui() -> impl Bundle {
    (
        Name::new("Purchase UI"),
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            row("Upgrade Attack", 10.0, upgrade_attack),
            row("Draggable Attacker", 20.0, enable_dragging),
            // row("Upgrade Speed", 200.0),
            // row("Upgrade Range", 300.0)
        ],
    )
}

fn row<E, B, M, I>(item: impl Into<String>, price: f32, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    (
        Name::new("Row"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            column_gap: Px(10.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        children![
            label(item.into()),
            purchase_button(format!("{}", price), action),
        ],
    )
}

fn purchase_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        TextFont::from_font_size(24.0),
        action,
        Node {
            width: Px(30.0),
            height: Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
}

fn upgrade_attack(
    _t: Trigger<Pointer<Click>>,
    mut inventory: ResMut<inventory::Inventory>,
    mut player_stats: ResMut<PlayerStats>,
) {
    // Check if the player has enough score to upgrade
    if inventory.dust_data >= 10 {
        // Deduct the cost from the inventory score
        inventory.dust_data -= 10;
        // Here you would also increase the attack energy or whatever is needed
        // For demonstration, we just log the upgrade
        player_stats.attack_energy += 5.0; // Example upgrade
        info!(
            "Attack upgraded! New attack energy: {}",
            player_stats.attack_energy
        );
    } else {
        info!("Not enough score to upgrade attack.");
    }
}

fn enable_dragging(_t: Trigger<Pointer<Click>>, mut player_stats: ResMut<PlayerStats>) {
    player_stats.dragable_attacker = true;
    info!("Dragging attacker enabled");
}
