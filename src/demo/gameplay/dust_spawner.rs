use super::dust::dust;
use crate::{demo::GAME_AREA, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(DustSpawnerStats {
        dust_spawn_speed: 2, // Default spawn speed
    });
    app.add_systems(Update, tick_spawner_timer.in_set(AppSystems::TickTimers));
    app.add_systems(Update, (spawn_dust,).in_set(AppSystems::Update));

    app.add_systems(Update, gizmos.run_if(in_state(Screen::Gameplay)));
}

// TODO: make use of it
#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct DustSpawnerStats {
    dust_spawn_speed: u32, // Number of dust particles spawned per second
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

fn tick_spawner_timer(mut spawner: Query<&mut DustSpawner>, time: Res<Time>) {
    for mut spawner in spawner.iter_mut() {
        spawner.timer.tick(time.delta());
    }
}

fn spawn_dust(mut commands: Commands, spawners: Query<(&mut DustSpawner, &mut Entropy<WyRand>)>) {
    for (spawner, mut entropy) in spawners {
        if spawner.timer.just_finished() {
            let pos = Vec2::new(
                entropy.random_range(GAME_AREA.min.x..GAME_AREA.max.x),
                GAME_AREA.max.y,
            );
            commands.spawn(dust(pos, entropy.random_range(80.0..120.0)));
        }
    }
}

fn gizmos(mut gizmos: Gizmos) {
    gizmos.rect_2d(Isometry2d::default(), GAME_AREA.size(), Color::BLACK);
}
