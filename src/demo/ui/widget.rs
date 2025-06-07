use bevy::ecs::spawn::SpawnWith;
use bevy::ecs::system::IntoObserverSystem;
use bevy::ui::Val::*;

use crate::prelude::*;

use crate::theme::palette::*;
use crate::theme::prelude::*;
pub use crate::theme::widget::*;

pub fn row<E, B, M, I>(
    item_name: impl Into<String>,
    tip: impl Into<String>,
    price: u32,
    action: I,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    (
        Name::new("PurchaseRow"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            column_gap: Px(10.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor::from(PURCHASE_ROW_BACKGROUND),
        BorderRadius::all(Val::Px(5.0)),
        children![upgrade_text(item_name, tip), upgrade_button(price, action),],
    )
}

fn upgrade_text(item_name: impl Into<String>, tip: impl Into<String>) -> impl Bundle {
    (
        Name::new("UpgradeText"),
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexStart,
            ..default()
        },
        children![
            (
                Name::new("Item Name"),
                Text::new(item_name.into()),
                TextFont::from_font_size(20.0),
            ),
            (
                Name::new("Tip"),
                Text::new(tip.into()),
                TextFont::from_font_size(16.0),
            ),
        ],
    )
}

pub fn upgrade_button<E, B, M, I>(cost: u32, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("UpgradeButton"),
        Node {
            width: Px(100.0),
            height: Px(40.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Node {
                        width: Px(80.0),
                        height: Px(32.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    Button,
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    BorderRadius::all(Px(5.0)),
                    children![(
                        Name::new("Button Text"),
                        Text::new("Upgrade"),
                        TextFont::from_font_size(16.0)
                    )],
                ))
                .observe(action);
            parent.spawn((
                Name::new("Cost Text"),
                Text::new(format!("Cost: {}", cost)),
                TextFont::from_font_size(16.0),
            ));
        })),
    )
}
