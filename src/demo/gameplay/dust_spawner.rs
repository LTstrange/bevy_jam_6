use bevy_rand::prelude::{Entropy, WyRand};
use rand::prelude::Rng;

use super::dust::dust;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        tick_spawner_timer
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );
    app.add_systems(
        Update,
        (spawn_dust,)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub fn dust_spawner() -> impl Bundle {
    (
        Name::new("Dust Spawner"),
        StateScoped(Screen::Gameplay),
        DustSpawner {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        },
        Entropy::<WyRand>::default(),
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct DustSpawner {
    timer: Timer,
}

fn tick_spawner_timer(mut spawner: Single<&mut DustSpawner>, time: Res<Time>) {
    spawner.timer.tick(time.delta());
}

fn spawn_dust(mut commands: Commands, spawner: Single<(&mut DustSpawner, &mut Entropy<WyRand>)>) {
    let (spawner, mut entropy) = spawner.into_inner();
    if spawner.timer.just_finished() {
        let pos = Vec2::new(entropy.random_range(-200.0..200.0), 200.0);
        commands.spawn(dust(pos));
    }
}
