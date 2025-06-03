#![allow(dead_code)]
use bevy::ecs::system::IntoObserverSystem;
use bevy::ui::Val::*;

use crate::prelude::*;

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

pub fn row_done(item: impl Into<String>) -> impl Bundle {
    (
        Name::new("Row Done"),
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
        children![label(item.into()),],
    )
}
