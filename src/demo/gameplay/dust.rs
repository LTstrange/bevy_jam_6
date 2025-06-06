use crate::{
    demo::{GAME_AREA, ui::inventory::Inventory},
    prelude::*,
};

use super::health::{Health, health_bar_ui};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (falling_dust,).in_set(AppSystems::Update));
    app.add_systems(
        Update,
        (cleanup_unalived_dust, despawn_dust)
            .chain()
            .in_set(AppSystems::Cleanup),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(super) struct Dust;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Velocity {
    speed: f32,
}

// TODO: impl health component for dust
pub const DUST_HEALTH: f32 = 5.0;

pub fn dust(pos: Vec2, speed: f32) -> impl Bundle {
    (
        Name::new("Dust"),
        Dust,
        Transform::from_translation(pos.extend(0.0)),
        StateScoped(Screen::Gameplay),
        Velocity { speed },
        Sprite::from_color(
            Color::WHITE.with_alpha(map_range(speed, 80.0..120.0, 0.4..1.0)),
            Vec2::new(16.0, 16.0),
        ),
        health_bar_ui(DUST_HEALTH, Vec2::new(0.0, 11.0), Vec2::new(20.0, 4.0)),
    )
}

fn falling_dust(query: Query<(&mut Transform, &Velocity), With<Dust>>, time: Res<Time>) {
    for (mut transform, velocity) in query {
        transform.translation.y -= velocity.speed * time.delta_secs();
    }
}

fn cleanup_unalived_dust(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<Dust>>,
    mut inventory: ResMut<Inventory>,
) {
    for (entity, health) in query.iter() {
        if !health.is_alive() {
            inventory.dust_data += 1;
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_dust(mut commands: Commands, query: Query<(Entity, &Transform), With<Dust>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < GAME_AREA.min.y {
            commands.entity(entity).despawn();
        }
    }
}
