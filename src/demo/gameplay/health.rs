use bevy::{
    color::palettes::css::*,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_health_bar, toggle_health_bar));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

#[allow(dead_code)]
impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
    pub fn apply_damage(&mut self, damage: f32) {
        self.current -= damage.min(self.current);
    }
    pub fn is_alive(&self) -> bool {
        self.current > 0.5
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn is_max_health(&self) -> bool {
        self.current >= self.max
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct HealthBar(Vec2, Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Transform)]
struct HealthBarCompose;

pub fn health_bar_and_ui(health: f32, offset: Vec2, size: Vec2) -> impl Bundle {
    (
        Health::new(health),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<_>| {
            let parent_entity = parent.target_entity();
            parent.spawn((
                HealthBarCompose,
                Visibility::Hidden,
                health_bar_compose(offset, size, parent_entity),
            ));
        })),
    )
}

fn health_bar_compose(offset: Vec2, size: Vec2, parent: Entity) -> impl Bundle {
    let anchor = offset - size / 2.0;
    children![
        (
            Sprite {
                color: WHITE.into(),
                custom_size: Some(size),
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..Default::default()
            },
            Transform::from_translation(anchor.extend(0.0)),
        ),
        (
            Sprite {
                color: RED.into(),
                custom_size: Some(Vec2::new(size.x - 2.0, size.y - 2.0)),
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..Default::default()
            },
            Transform::from_translation((anchor + Vec2::ONE).extend(0.1)),
        ),
        (
            HealthBar(size, parent),
            Sprite {
                color: GREEN.into(),
                custom_size: Some(Vec2::new(size.x * 0.8, size.y - 2.0)), // 80.0 只是示例，实际应根据血量调整
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..Default::default()
            },
            Transform::from_translation((anchor + Vec2::ONE).extend(0.2)), // 让绿色条在红色条上方
        ),
    ]
}

fn update_health_bar(
    health_bar: Query<(&mut Sprite, &HealthBar)>,
    parents: Query<&Health>,
) -> Result {
    for (mut sprite, HealthBar(size, parent)) in health_bar {
        let Health { current, max } = parents.get(*parent)?;
        let ratio = current / max;
        sprite.custom_size = Some(Vec2::new((size.x - 2.0) * ratio, size.y - 2.0));
    }
    Ok(())
}

fn toggle_health_bar(
    health_bar: Query<(&ChildOf, &mut Visibility), With<HealthBarCompose>>,
    health: Query<&Health>,
) -> Result {
    for (ChildOf(parent), mut visibility) in health_bar {
        let parent_health = health.get(*parent)?;
        if parent_health.is_max_health() {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
    Ok(())
}
