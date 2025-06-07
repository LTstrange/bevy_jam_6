#![allow(dead_code)]
use bevy::ecs::spawn::SpawnWith;
use bevy::ecs::system::IntoObserverSystem;
use bevy::ui::Val::*;

use crate::prelude::*;

use crate::theme::palette::*;
use crate::theme::prelude::*;
pub use crate::theme::widget::*;

pub fn purchase_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
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

pub fn row<E, B, M, I>(item: impl Into<String>, price: u32, action: I) -> impl Bundle
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
        BorderRadius::all(Val::Px(5.0)),
        children![label(item.into()), buy_button(price, action),],
    )
}

// pub fn row_done(item: impl Into<String>) -> impl Bundle {
//     (
//         Name::new("Row Done"),
//         Node {
//             width: Val::Percent(100.0),
//             height: Val::Px(50.0),
//             column_gap: Px(10.0),
//             flex_direction: FlexDirection::Row,
//             justify_content: JustifyContent::SpaceBetween,
//             align_items: AlignItems::Center,
//             ..default()
//         },
//         BorderRadius::all(Val::Px(5.0)),
//         children![label(item.into()),],
//     )
// }

pub fn buy_button<E, B, M, I>(cost: u32, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("BuyButton"),
        Node {
            width: Px(120.0),
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
                        Text::new("Buy"),
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
