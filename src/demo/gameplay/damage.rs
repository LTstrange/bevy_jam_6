use bevy::color::palettes::css::*;

use crate::{
    prelude::*,
    visual_effect::{AttackLine, TempoEffect},
};

use super::{dust::Dust, health::Health};

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
    amount: u32,
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub enum DamageType {
    Lightning,
}

const LIGHTING_RANGE: f32 = 100.0;

pub fn generate_damage(
    pos: Vec2,
    amount: u32,
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
    mut dust: Query<(Entity, &mut Health, &Transform), With<Dust>>,
) -> Result {
    // TODO: allow attack same dust by multiple damage entities
    let mut attacked_dust = vec![];
    for (damage_entity, Damage { amount }, damage_type, damage_transform, mut entropy) in damages {
        match damage_type {
            DamageType::Lightning => {
                // search for dust entities within a certain radius
                // TODO: use a more efficient spatial partitioning method
                // TODO: Find the nearest dust entity instead of any
                let damage_pos = damage_transform.translation.truncate();

                // find dust entities that are within LIGHTING_RANGE of the damage entity
                let (nearest_dust, _, dust_pos) = dust
                    .iter()
                    .filter(|(_, _, transform)| {
                        let distance = transform
                            .translation
                            .truncate()
                            .distance_squared(damage_pos);
                        20.0 < distance && distance < LIGHTING_RANGE * LIGHTING_RANGE // radius squared
                    })
                    .filter(|(e, _, _)| !attacked_dust.contains(e))
                    .fold(
                        (Entity::PLACEHOLDER, f32::INFINITY, Vec2::ZERO),
                        |min, (entity, _, transform)| {
                            let pos = transform.translation.truncate();
                            let distance = pos.distance_squared(damage_pos);
                            if distance < min.1 {
                                (entity, distance, pos)
                            } else {
                                min
                            }
                        },
                    );
                if nearest_dust == Entity::PLACEHOLDER {
                    // no dust found, dissipate
                    commands.entity(damage_entity).despawn();
                    continue;
                }

                let (_, mut health, _) = dust.get_mut(nearest_dust)?;
                // random the amount of damage to apply
                let deal_amount = entropy.random_range((amount.div_ceil(2))..=*amount);
                health.apply_damage(deal_amount);

                attacked_dust.push(nearest_dust);
                commands.send_event(AttackDustEvent {
                    source: damage_transform.translation.truncate(),
                    target: dust_pos,
                    amount: deal_amount,
                    remaining_energy: *amount - deal_amount,
                    damage_type: *damage_type,
                    entropy: entropy.clone(),
                });
            }
        }
        // cleanup damage entity
        commands.entity(damage_entity).despawn();
    }
    Ok(())
}

#[derive(Event, Debug)]
struct AttackDustEvent {
    source: Vec2,
    target: Vec2,
    amount: u32,
    remaining_energy: u32,
    damage_type: DamageType,
    entropy: Entropy<WyRand>,
}

fn deal_attack_event(
    mut commands: Commands,
    mut event_reader: EventReader<AttackDustEvent>,
) -> Result {
    for &AttackDustEvent {
        source,
        target,
        amount,
        remaining_energy,
        damage_type,
        ref entropy,
    } in event_reader.read()
    {
        commands.spawn(lightning_effect(target, source));
        commands.spawn(damage_text(amount, target));
        if remaining_energy > 0 {
            commands.spawn(generate_damage(
                target,
                remaining_energy,
                damage_type,
                entropy.clone(),
            ));
        }
    }

    Ok(())
}

fn lightning_effect(target: Vec2, source: Vec2) -> impl Bundle {
    (
        Name::new("Lightning Effect"),
        StateScoped(Screen::Gameplay),
        TempoEffect::new(0.1),
        Transform::from_translation(target.extend(0.0)),
        Sprite::from_color(ORANGE, Vec2::new(16.0, 16.0)),
        AttackLine {
            start: source,
            end: target,
        },
    )
}

fn damage_text(amount: u32, pos: Vec2) -> impl Bundle {
    (
        Name::new("Damage Text"),
        StateScoped(Screen::Gameplay),
        Transform::from_translation(pos.extend(0.0)),
        Text2d::new(format!("-{}", amount)),
        TempoEffect::new(0.5),
        TextFont::from_font_size(12.0),
        TextColor(RED.into()),
        GlobalZIndex(100),
    )
}
