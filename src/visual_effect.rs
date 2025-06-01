use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, draw_attack_line);
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
