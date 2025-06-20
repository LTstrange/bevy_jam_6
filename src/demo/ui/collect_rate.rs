use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{Diagnostic, DiagnosticPath, DiagnosticsStore, RegisterDiagnostic},
};

use crate::{
    menus::{COMPLETE_COLLECTION_RATE, CompleteTheGame},
    prelude::*,
    theme::palette::PURCHASE_ROW_BACKGROUND,
};

pub(super) fn plugin(app: &mut App) {
    app.register_diagnostic(
        Diagnostic::new(DUST_COLLECT_RATE_DIAGNOSTIC).with_smoothing_factor(5.0),
    );

    app.add_systems(Update, update_collect_rate);

    app.add_systems(
        Update,
        send_complete_event.run_if(in_state(Screen::Gameplay)),
    );
}

pub const DUST_COLLECT_RATE_DIAGNOSTIC: DiagnosticPath =
    DiagnosticPath::const_new("dust_collect_rate");

pub fn send_complete_event(
    mut commands: Commands,
    mut marker: Local<bool>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if *marker {
        return; // Already sent the event
    }
    if let Some(rate) = diagnostics
        .get(&DUST_COLLECT_RATE_DIAGNOSTIC)
        .and_then(|rate| rate.smoothed())
    {
        if rate >= COMPLETE_COLLECTION_RATE {
            *marker = true; // Set marker to true to prevent multiple sends
            commands.send_event(CompleteTheGame);
        }
    }
}

pub fn goal_ui() -> impl Bundle {
    (
        Name::new("Collect Rate UI"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(Val::Px(10.0)),
            ..default()
        },
        BorderRadius::all(Val::Px(5.0)),
        BackgroundColor(PURCHASE_ROW_BACKGROUND),
        StateScoped(Screen::Gameplay),
        children![
            (
                Text::new("Goal: Increase Dust Collection Rate"),
                TextColor::from(Color::BLACK),
                TextFont::from_font_size(20.0)
            ),
            row(),
        ],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct CollectRateTextSpan;

fn row() -> impl Bundle {
    (
        Name::new("Collect Rate Row"),
        Node {
            // flex_direction: FlexDirection::Row,
            // justify_content: JustifyContent::Center,
            // align_items: AlignItems::Center,
            ..default()
        },
        Text::new("Collect Rate: "),
        children![
            (
                TextSpan::new("0.0"),
                CollectRateTextSpan,
                TextFont::from_font_size(20.0),
                TextColor::from(GOLD),
            ),
            (
                TextSpan::new(" dust/s"),
                TextFont::from_font_size(20.0),
                TextColor::from(GOLD),
            )
        ],
    )
}

fn update_collect_rate(
    diagnostics: Res<DiagnosticsStore>,
    mut textspan: Single<&mut TextSpan, With<CollectRateTextSpan>>,
) {
    if let Some(rate) = diagnostics
        .get(&DUST_COLLECT_RATE_DIAGNOSTIC)
        .and_then(|rate| rate.smoothed())
    {
        textspan.0 = format!("{:.1}", rate);
    }
}
