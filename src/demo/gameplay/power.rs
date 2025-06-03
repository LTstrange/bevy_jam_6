use bevy::{
    color::palettes::css::WHITE,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Power::new(20, 1.0));

    app.add_systems(Update, regenerate_power.in_set(AppSystems::Update));
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Power {
    current: u32,
    max: u32,
    timer: Timer,
}

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

    fn render(&self) -> impl Bundle {
        (
            Name::new("Power UI"),
            StateScoped(Screen::Gameplay),
            Transform::default(),
            Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<_>| {
                parent.spawn(Sprite::from_color(WHITE, Vec2::new(50.0, 20.0)));
            })),
        )
    }
}

fn regenerate_power(mut power: ResMut<Power>, time: Res<Time>) {
    // Example regeneration logic: regenerate 1 power every 1 seconds
    if time.elapsed_secs() % 1.0 == 0.0 {
        power.regenerate(1);
    }
}
