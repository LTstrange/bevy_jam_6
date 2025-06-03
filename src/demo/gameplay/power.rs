use bevy::color::palettes::css::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Power::new(20, 1.0 / 5.0));

    app.add_systems(Update, update_power_ui.run_if(in_state(Screen::Gameplay)));

    app.add_systems(Update, regenerate_power.in_set(AppSystems::Update));
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Power {
    current: u32,
    max: u32,
    timer: Timer,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PowerUI;

impl Power {
    fn new(max: u32, regen_interval: f32) -> Self {
        Self {
            current: max,
            max,
            timer: Timer::from_seconds(regen_interval, TimerMode::Repeating),
        }
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn consume(&mut self, amount: u32) -> u32 {
        let output = self.current.min(amount);
        self.current = self.current.saturating_sub(output);
        output
    }

    fn regenerate(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.max);
    }
}

fn regenerate_power(mut power: ResMut<Power>, time: Res<Time>) {
    // Example regeneration logic: regenerate 1 power every 1 seconds
    if power.timer.tick(time.delta()).just_finished() {
        power.regenerate(1);
    }
}

pub fn power_ui() -> impl Bundle {
    (
        Name::new("Power UI"),
        Node {
            width: Val::Px(200.0),
            height: Val::Px(40.0),
            padding: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(WHITE.into()),
        children![(
            PowerUI,
            Node {
                // width: Val::Percent(100.0),
                // height: Val::Percent(100.0),
                ..default()
            },
            Text::new("Power: 333 / 999"),
            TextFont::default(),
            BackgroundColor(GREEN.into()),
        )],
        // Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<_>| {
        //     parent.spawn(Sprite::from_color(WHITE, Vec2::new(50.0, 20.0)));
        // })),
    )
}

fn update_power_ui(ui: Single<(&mut Text, &mut Node), With<PowerUI>>, power: Res<Power>) {
    let (mut text, mut node) = ui.into_inner();
    text.0 = format!("Power: {}/{}", power.current(), power.max);
    // node.width = Val::Percent(power.current() as f32 / power.max as f32);
}
