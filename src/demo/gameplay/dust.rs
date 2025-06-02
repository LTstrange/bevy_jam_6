use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(
    //     Update,
    //     tick_spawner_timer
    //         .in_set(AppSystems::TickTimers)
    //         .in_set(PausableSystems),
    // );
    app.add_systems(
        Update,
        (falling_dust, despawn_dust).in_set(AppSystems::Update),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(super) struct Dust;

// TODO: impl health component for dust
pub const DUST_HEALTH: f32 = 5.0;

pub fn dust(pos: Vec2) -> impl Bundle {
    (
        Name::new("Dust"),
        Dust,
        Transform::from_translation(pos.extend(0.0)),
        StateScoped(Screen::Gameplay),
        Sprite {
            custom_size: Some(Vec2::new(16.0, 16.0)),
            color: Color::WHITE,
            ..default()
        },
    )
}

fn falling_dust(mut query: Query<&mut Transform, With<Dust>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= 100.0 * time.delta_secs();
    }
}

fn despawn_dust(mut commands: Commands, query: Query<(Entity, &Transform), With<Dust>>) {
    for (entity, transform) in query.iter() {
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
}
