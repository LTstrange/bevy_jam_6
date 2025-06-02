use crate::demo::ChangePlayerStats;
use crate::demo::gameplay::SpawnAttacker;
use crate::prelude::*;
use crate::theme::widget::{button_base, header, label};
use bevy::ecs::system::IntoObserverSystem;
use bevy::ui::Val::*;

use super::inventory;

pub(super) fn plugin(_app: &mut App) {}

macro_rules! row {
    ($item:expr, $price:expr, $event:expr) => {
        row(
            $item,
            $price,
            |_t: Trigger<Pointer<Click>>,
             mut inventory: ResMut<inventory::Inventory>,
             mut commands: Commands| {
                if inventory.dust_data >= $price {
                    inventory.dust_data -= $price;
                    commands.trigger($event);
                } else {
                    info!("Not enough data to purchase.");
                }
            },
        )
    };
}

pub fn purchase_ui() -> impl Bundle {
    (
        Name::new("Purchase UI"),
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            header("Research Lab"),
            row!(
                "Upgrade Attack",
                10,
                ChangePlayerStats::AddAttackEnergy(5.0)
            ),
            row!(
                "Draggable Attacker",
                20,
                ChangePlayerStats::SetDraggableAttacker(true)
            ),
            row!("New Attacker", 50, SpawnAttacker),
            // row("Upgrade Speed", 200.0),
            // row("Upgrade Range", 300.0)
        ],
    )
}

fn row<E, B, M, I>(item: impl Into<String>, price: u32, action: I) -> impl Bundle
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
