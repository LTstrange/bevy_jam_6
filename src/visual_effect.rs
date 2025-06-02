use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, draw_attack_line);
    app.add_systems(Update, remove_tempo_effect.in_set(AppSystems::TickTimers));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct TempoEffect {
    timer: Timer,
}

impl TempoEffect {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct AttackLine {
    pub start: Vec2,
    pub end: Vec2,
}

fn draw_attack_line(query: Query<&AttackLine>, mut gizmos: Gizmos) {
    for attack_line in query.iter() {
        gizmos.line_2d(attack_line.start, attack_line.end, Color::WHITE);
    }
}

fn remove_tempo_effect(
    mut commands: Commands,
    query: Query<(Entity, &mut TempoEffect)>,
    time: Res<Time>,
) {
    for (entity, mut tempo_effect) in query {
        if tempo_effect.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}
