use bevy::color::palettes::{
    css::{BLACK, WHITE},
    tailwind::*,
};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Power::new(20.0, 5.0));

    app.add_systems(Update, update_power_ui.run_if(in_state(Screen::Gameplay)));

    app.add_systems(Update, regenerate_power.in_set(AppSystems::Update));

    app.add_observer(update_power_stats);
}

#[derive(Event, Debug, Clone)]
pub enum SetPowerStats {
    RegenSpeed(f32), // Set the amount of power regenerated per second
    PowerMax(f32),   // Set the maximum power
}

fn update_power_stats(event: Trigger<SetPowerStats>, mut power: ResMut<Power>) {
    match event.event() {
        SetPowerStats::RegenSpeed(speed) => {
            power.regen_speed = *speed;
        }
        SetPowerStats::PowerMax(max) => {
            power.max = *max;
        }
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Power {
    current: f32,
    max: f32,
    regen_speed: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PowerUI;

impl Power {
    fn new(max: f32, regen_speed: f32) -> Self {
        Self {
            current: max,
            max,
            regen_speed,
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn consume(&mut self, amount: f32) -> f32 {
        let output = self.current.min(amount);
        self.current = (self.current - output).max(0.0);
        output
    }

    fn regenerate(&mut self, delta: f32) {
        self.current = (self.current + self.regen_speed * delta).min(self.max);
    }
}

fn regenerate_power(mut power: ResMut<Power>, time: Res<Time>) {
    // Example regeneration logic: regenerate 1 power every 1 seconds
    power.regenerate(time.delta().as_secs_f32());
}

pub fn power_ui() -> impl Bundle {
    (
        Name::new("Power UI"),
        Node {
            width: Val::Px(200.0),
            // height: Val::Px(40.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            padding: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(WHITE.into()),
        children![(
            Node {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                width: Val::Percent(100.0),
                overflow: Overflow::visible(),
                ..default()
            },
            BackgroundColor(GREEN_500.into()),
            children![(
                PowerUI,
                Text::new("Power: 333 / 999"),
                TextLayout::new(JustifyText::Left, LineBreak::NoWrap),
                TextFont::default(),
                TextColor(BLACK.into()),
            )]
        )],
        // Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<_>| {
        //     parent.spawn(Sprite::from_color(WHITE, Vec2::new(50.0, 20.0)));
        // })),
    )
}

fn update_power_ui(
    mut bar: Query<&mut Node>,
    text: Single<(&mut Text, &ChildOf), With<PowerUI>>,
    power: Res<Power>,
) {
    // info!("Update power UI: {}/{}", power.current(), power.max);
    let (mut text, parent) = text.into_inner();
    let mut bar = bar.get_mut(parent.0).expect("Power UI bar not found");

    text.0 = format!("Power: {:.0}/{}", power.current(), power.max);
    bar.width = Val::Percent((power.current() / power.max) * 100.0);
}
