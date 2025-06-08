use bevy::diagnostic::Diagnostics;

use crate::{
    demo::{
        GAME_AREA,
        ui::{collect_rate::DUST_COLLECT_RATE_DIAGNOSTIC, inventory::Inventory},
    },
    prelude::*,
};

use super::health::{Health, health_bar_and_ui};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (falling_dust,).in_set(AppSystems::Update));
    app.add_systems(
        Update,
        (cleanup_unalived_dust, despawn_dust)
            .chain()
            .in_set(AppSystems::Cleanup),
    );
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub(super) enum Dust {
    #[default]
    Small,
    Big,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Velocity {
    speed: f32,
}

// TODO: impl health component for dust
pub const DUST_HEALTH: f32 = 5.0;

pub fn dust(pos: Vec2, speed: f32, dust: Dust) -> impl Bundle {
    let (health, color, size) = match dust {
        Dust::Small => (DUST_HEALTH, Color::WHITE, Vec2::new(16.0, 16.0)),
        Dust::Big => (DUST_HEALTH * 2.0, Color::BLACK, Vec2::new(20.0, 20.0)),
    };

    (
        Name::new("Dust"),
        dust,
        Transform::from_translation(pos.extend(0.0)),
        StateScoped(Screen::Gameplay),
        Velocity { speed },
        Sprite::from_color(
            color.with_alpha(map_range(speed, 80.0..120.0, 0.4..1.0)),
            size,
        ),
        health_bar_and_ui(health, Vec2::new(0.0, 11.0), Vec2::new(20.0, 4.0)),
    )
}

fn falling_dust(query: Query<(&mut Transform, &Velocity), With<Dust>>, time: Res<Time>) {
    for (mut transform, velocity) in query {
        transform.translation.y -= velocity.speed * time.delta_secs();
    }
}

fn cleanup_unalived_dust(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform, &Dust)>,
    mut inventory: ResMut<Inventory>,
    mut rng: GlobalEntropy<WyRand>,
    mut diagnostic: Diagnostics,
    time: Res<Time>,
) -> Result {
    let mut all_dust_data = 0;
    for (entity, health, transform, dust_ty) in query {
        if !health.is_alive() {
            match dust_ty {
                Dust::Small => {
                    all_dust_data += 1; // Small dust equals 1 dust data
                }
                Dust::Big => {
                    all_dust_data += 2; // Big dust equals 2 dust data
                    let pos = transform.translation.truncate();
                    // despawn big dust, gen small dusts
                    let max_range = 100.0;
                    // 95% confidence interval for normal distribution
                    let distr = rand_distr::Normal::new(0.0, max_range / 2.45)?;
                    // spawn 4 small dusts around the position of big dust
                    for _ in 0..4 {
                        let x = distr.sample(&mut rng);
                        let y = distr.sample(&mut rng);
                        let diff = Vec2::new(x, y).clamp_length_max(max_range);
                        let speed = rng.random_range(80.0..120.0);
                        commands.spawn(dust(
                            (pos + diff).clamp(GAME_AREA.min, GAME_AREA.max),
                            speed,
                            Dust::Small,
                        ));
                    }
                }
            }
            commands.entity(entity).despawn();
        }
    }
    inventory.dust_data += all_dust_data;
    diagnostic.add_measurement(&DUST_COLLECT_RATE_DIAGNOSTIC, || {
        all_dust_data as f64 / time.delta_secs() as f64
    });
    Ok(())
}

fn despawn_dust(mut commands: Commands, query: Query<(Entity, &Transform), With<Dust>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < GAME_AREA.min.y {
            commands.entity(entity).despawn();
        }
    }
}
