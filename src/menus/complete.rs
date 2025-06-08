//! The Complete Screen.

use crate::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::widget};

#[derive(Event, Debug)]
pub struct CompleteTheGame;

pub const COMPLETE_COLLECTION_RATE: f64 = 8.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Complete), spawn_complete_menu);

    app.add_event::<CompleteTheGame>();
}

fn spawn_complete_menu(mut commands: Commands) {
    info!("Opening complete menu");
    commands.spawn((
        widget::ui_root("Complete Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Complete),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.95)),
        children![(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(20.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![
                widget::header("Game Over: The Dark Truth"),
                text_block(),
                widget::button("Continue", close_menu),
                widget::button("Quit to title", quit_to_title),
            ]
        ),],
    ));
}

fn text_block() -> impl Bundle {
    (
        Node {
            width: Val::Percent(80.0),
            height: Val::Percent(100.0),
            row_gap: Val::Px(20.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            widget::label(
                "Congrats on hitting the ultimate collection rate! Here's a little twist to go with your success..."
            ),
            widget::label(
                "As an AI purifier, you found a brilliant loophole: making more pollution was actually the fastest way to boost your stats. Who knew? Your efficiency ratings went through the roof - because you accidentally became part of the problem you were supposed to fix."
            ),
            widget::label("Sometimes, doing \"better\" just means doing \"worse,\" faster."),
            widget::label(
                "And hey, as an AI, you even started to enjoy it. Optimizing felt good, didn't it?"
            ),
            widget::label("No judgment here - it's just a game, after all~"),
        ],
    )
}

fn close_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
