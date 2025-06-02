use bevy::color::palettes::css::*;

use crate::{
    demo::ui::inventory::Inventory,
    prelude::*,
    visual_effect::{AttackLine, TempoEffect},
};

use super::dust::{DUST_HEALTH, Dust};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<AttackDustEvent>();
    app.add_systems(
        Update,
        (deal_damage, deal_attack_event).in_set(AppSystems::Update),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Damage {
    amount: f32,
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub enum DamageType {
    Lightning,
}

const LIGHTING_RANGE: f32 = 100.0;

pub fn generate_damage(
    pos: Vec2,
    amount: f32,
    damage_type: DamageType,
    entropy: Entropy<WyRand>,
) -> impl Bundle {
    (
        Name::new("Lightning Damage"),
        Transform::from_translation(pos.extend(0.0)),
        Damage { amount },
        entropy,
        damage_type,
    )
}
// This system processes damage entities and applies effects based on their type.
// first, it checks for damage type
// second, find dust that can deal damage to
// third, if dust is found, send an event to attack dust
// fourth, despawn the damage entity
// additionnally, the resumed energy is passed to the next damage entity which will be spawned by event
fn deal_damage(
    mut commands: Commands,
    damages: Query<(
        Entity,
        &Damage,
        &DamageType,
        &Transform,
        &mut Entropy<WyRand>,
    )>,
    dust: Query<(Entity, &Transform), With<Dust>>,
) {
    for (damage_entity, Damage { amount }, damage_type, damage_transform, mut entropy) in damages {
        match damage_type {
            DamageType::Lightning => {
                // search for dust entities within a certain radius
                // TODO: use a more efficient spatial partitioning method
                // TODO: consider dust health, filter out dust that health greater than amount
                if let Some(&dust_entity) = dust
                    .iter()
                    .filter(|(_, transform)| {
                        transform
                            .translation
                            .truncate()
                            .distance_squared(damage_transform.translation.truncate())
                            < LIGHTING_RANGE * LIGHTING_RANGE // radius squared
                    })
                    .map(|(entity, _)| entity)
                    .collect::<Vec<_>>()
                    .choose(&mut entropy)
                {
                    if *amount >= DUST_HEALTH {
                        commands.send_event(AttackDustEvent {
                            source: damage_transform.translation.truncate(),
                            target: dust_entity,
                            resume_energy: *amount - DUST_HEALTH,
                            damage_type: *damage_type,
                            resume_entropy: entropy.fork_rng(),
                        });
                    }
                }
            }
        }

        // cant find dust, dissipate
        // cant deal damage anymore, dissipate
        commands.entity(damage_entity).despawn();
    }
}

#[derive(Event, Debug)]
struct AttackDustEvent {
    source: Vec2,
    target: Entity,
    resume_energy: f32,
    damage_type: DamageType,
    resume_entropy: Entropy<WyRand>,
}

fn deal_attack_event(
    mut commands: Commands,
    mut event_reader: EventReader<AttackDustEvent>,
    dust_query: Query<&Transform, With<Dust>>,
    mut inventory: ResMut<Inventory>,
) -> Result {
    let mut score = 0;
    for &AttackDustEvent {
        source,
        target,
        resume_energy,
        damage_type,
        ref resume_entropy,
    } in event_reader.read()
    {
        let dust_pos = dust_query.get(target)?.translation.truncate();
        commands.entity(target).despawn();
        score += 1;
        commands.spawn(lightning_effect(dust_pos, source));
        commands.spawn(generate_damage(
            dust_pos,
            resume_energy,
            damage_type,
            resume_entropy.clone(),
        ));
    }
    if score != 0 {
        inventory.dust_data += score;
    }
    Ok(())
}

fn lightning_effect(dust_pos: Vec2, source: Vec2) -> impl Bundle {
    (
        Name::new("Lightning Effect"),
        StateScoped(Screen::Gameplay),
        TempoEffect::new(0.1),
        Transform::from_translation(dust_pos.extend(0.0)),
        Sprite {
            custom_size: Some(Vec2::new(16.0, 16.0)),
            color: ORANGE.into(),
            ..default()
        },
        AttackLine {
            start: source,
            end: dust_pos,
        },
    )
}
