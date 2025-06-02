use crate::{demo::GAME_AREA, prelude::*};

use super::health::Health;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (falling_dust, despawn_dust).in_set(AppSystems::Update),
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
        Health::new(DUST_HEALTH),
        Transform::from_translation(pos.extend(0.0)),
        StateScoped(Screen::Gameplay),
        Velocity { speed },
        Sprite::from_color(
            Color::WHITE.with_alpha(map_range(speed, 80.0..120.0, 0.4..1.0)),
            Vec2::new(16.0, 16.0),
        ),
    )
}

fn falling_dust(query: Query<(&mut Transform, &Velocity), With<Dust>>, time: Res<Time>) {
    for (mut transform, velocity) in query {
        transform.translation.y -= velocity.speed * time.delta_secs();
    }
}

fn despawn_dust(mut commands: Commands, query: Query<(Entity, &Transform), With<Dust>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -GAME_AREA.y / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}
