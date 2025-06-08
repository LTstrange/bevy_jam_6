use std::time::Duration;

use super::dust::dust;
use crate::{
    demo::{GAME_AREA, gameplay::dust::Dust},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, tick_spawner_timer.in_set(AppSystems::TickTimers));
    app.add_systems(Update, (spawn_dust,).in_set(AppSystems::Update));

    app.add_systems(Update, gizmos.run_if(in_state(Screen::Gameplay)));

    app.add_observer(
        |t: Trigger<SetDustSpawnStats>, mut dust_spawner: Single<&mut DustSpawner>| {
            let SetDustSpawnStats::SpawnSpeed(speed) = t.event();
            dust_spawner
                .timer
                .set_duration(Duration::from_secs_f32(1.0 / *speed));
        },
    );
}

pub fn dust_spawner() -> impl Bundle {
    (
        Name::new("Dust Spawner"),
        StateScoped(Screen::Gameplay),
        DustSpawner::new(2.0),
        Entropy::<WyRand>::default(),
    )
}

#[derive(Event, Debug, Clone)]
pub enum SetDustSpawnStats {
    SpawnSpeed(f32), // Change the spawn speed of dust particles
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct DustSpawner {
    timer: Timer,
}

impl DustSpawner {
    pub fn new(spawn_speed: f32) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / spawn_speed, TimerMode::Repeating),
        }
    }
}

fn tick_spawner_timer(mut spawner: Single<&mut DustSpawner>, time: Res<Time>) {
    spawner.timer.tick(time.delta());
}

fn spawn_dust(mut commands: Commands, spawners: Single<(&mut DustSpawner, &mut Entropy<WyRand>)>) {
    let (spawner, mut entropy) = spawners.into_inner();
    if spawner.timer.just_finished() {
        let pos = Vec2::new(
            entropy.random_range(GAME_AREA.min.x..GAME_AREA.max.x),
            GAME_AREA.max.y,
        );
        // TODO: random spawn big or small dust
        commands.spawn(dust(pos, entropy.random_range(80.0..120.0), Dust::Big));
    }
}

fn gizmos(mut gizmos: Gizmos) {
    gizmos.rect_2d(Isometry2d::default(), GAME_AREA.size(), Color::BLACK);
}
