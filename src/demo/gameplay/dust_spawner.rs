use bevy::math::ops::exp;

use super::dust::dust;
use crate::{
    demo::{GAME_AREA, gameplay::dust::Dust},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (spawn_dust,).in_set(AppSystems::Update));

    app.add_systems(Update, gizmos.run_if(in_state(Screen::Gameplay)));

    app.add_observer(
        |t: Trigger<SetDustSpawnStats>, mut dust_spawner: Single<&mut DustSpawner>| {
            let SetDustSpawnStats::SpawnSpeed(speed) = t.event();
            dust_spawner.set_speed(*speed);
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
    speed: f32,
}

impl DustSpawner {
    pub fn new(spawn_speed: f32) -> Self {
        Self { speed: spawn_speed }
    }
    fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }
}

const A: f32 = 1.0;
const B: f32 = 5.0;
fn spawn_dust(
    mut commands: Commands,
    spawners: Single<(&mut DustSpawner, &mut Entropy<WyRand>)>,
    time: Res<Time>,
    mut big_remainder: Local<f32>,
    mut small_remainder: Local<f32>,
) {
    let (spawner, mut entropy) = spawners.into_inner();
    let expected = spawner.speed * time.delta_secs();
    let alpha = 1.0 / (1.0 + exp(-A * (spawner.speed - B)));
    let big_dust_acount = expected * alpha;
    let small_dust_acount = expected * (1.0 - alpha);
    let big_dust_count = big_dust_acount + *big_remainder;
    let small_dust_count = small_dust_acount + *small_remainder;
    *small_remainder = small_dust_count.fract();
    *big_remainder = big_dust_count.fract();

    // gen small dust
    for _ in 0..small_dust_count as usize {
        let pos = Vec2::new(
            entropy.random_range(GAME_AREA.min.x..GAME_AREA.max.x),
            GAME_AREA.max.y,
        );
        commands.spawn(dust(pos, entropy.random_range(80.0..120.0), Dust::Small));
    }

    for _ in 0..big_dust_count as usize {
        let pos = Vec2::new(
            entropy.random_range(GAME_AREA.min.x..GAME_AREA.max.x),
            GAME_AREA.max.y,
        );

        commands.spawn(dust(pos, entropy.random_range(80.0..120.0), Dust::Big));
    }
}

fn gizmos(mut gizmos: Gizmos) {
    gizmos.rect_2d(Isometry2d::default(), GAME_AREA.size(), Color::BLACK);
}
