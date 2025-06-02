use bevy_rand::prelude::{Entropy, WyRand};
use rand::prelude::Rng;

use super::dust::dust;
use crate::{demo::GAME_AREA, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, tick_spawner_timer.in_set(AppSystems::TickTimers));
    app.add_systems(Update, (spawn_dust,).in_set(AppSystems::Update));

    app.add_systems(Update, gizmos.run_if(in_state(Screen::Gameplay)));
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
        let pos = Vec2::new(
            entropy.random_range(-GAME_AREA.x / 2.0..GAME_AREA.x / 2.0),
            GAME_AREA.y / 2.0,
        );
        commands.spawn(dust(pos, entropy.random_range(80.0..120.0)));
    }
}

fn gizmos(mut gizmos: Gizmos) {
    gizmos.rect_2d(Isometry2d::default(), GAME_AREA, Color::BLACK);
}
