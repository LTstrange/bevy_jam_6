use bevy::color::palettes::css::*;
use bevy_rand::{global::GlobalEntropy, prelude::WyRand};
use rand::seq::IndexedRandom;

use crate::{audio::sound_effect, prelude::*, visual_effect::AttackLine};

use super::{super::ui::inventory::Inventory, dust::Dust};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::Loading).load_collection::<AttackerAssets>(),
    );
    app.add_event::<AttackDustEvent>();
    app.add_systems(
        Update,
        tick_attacker_timer
            .in_set(AppSystems::TickTimers)
            .in_set(PausableSystems),
    );
    app.add_systems(
        Update,
        (
            attack_dust,
            deal_attack_event.run_if(resource_exists::<AttackerAssets>),
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Attacker {
    timer: Timer,
}

#[derive(Resource, Reflect, Debug, Default, AssetCollection)]
#[reflect(Resource)]
struct AttackerAssets {
    #[asset(
        paths(
            "audio/sound_effects/step1.ogg",
            "audio/sound_effects/step2.ogg",
            "audio/sound_effects/step3.ogg",
            "audio/sound_effects/step4.ogg"
        ),
        collection(typed)
    )]
    pub steps: Vec<Handle<AudioSource>>,
}

pub fn attacker(pos: Vec2, attack_interval: f32) -> impl Bundle {
    (
        Name::new("Attacker"),
        Attacker {
            timer: Timer::from_seconds(attack_interval, TimerMode::Once),
        },
        Transform::from_translation(pos.extend(0.0)),
        StateScoped(Screen::Gameplay),
        Sprite {
            custom_size: Some(Vec2::new(16.0, 16.0)),
            color: RED.into(),
            ..default()
        },
    )
}

fn tick_attacker_timer(mut query: Query<&mut Attacker>, time: Res<Time>) {
    for mut attacker in query.iter_mut() {
        attacker.timer.tick(time.delta());
    }
}

#[derive(Event, Debug)]
struct AttackDustEvent {
    source: Vec2,
    target: Entity,
}

fn attack_dust(
    mut commands: Commands,
    mut attacker: Query<(&mut Attacker, &Transform)>,
    dust_query: Query<(Entity, &Transform), With<Dust>>,
) {
    for (mut attacker, transform) in attacker.iter_mut() {
        if attacker.timer.finished() {
            for (dust_entity, dust_transform) in dust_query.iter() {
                if dust_transform.translation.distance(transform.translation) < 100.0 {
                    commands.send_event(AttackDustEvent {
                        source: transform.translation.truncate(),
                        target: dust_entity,
                    });
                    attacker.timer.reset();
                    break;
                }
            }
        }
    }
}

fn deal_attack_event(
    mut commands: Commands,
    mut event_reader: EventReader<AttackDustEvent>,
    dust_query: Query<&Transform, With<Dust>>,
    attacker_assets: Res<AttackerAssets>,
    mut global_entropy: GlobalEntropy<WyRand>,
    mut inventory: ResMut<Inventory>,
) -> Result {
    let mut score = 0;
    for &AttackDustEvent { source, target } in event_reader.read() {
        let dust = dust_query.get(target)?;
        commands.entity(target).despawn();
        score += 1;
        commands.spawn((
            Name::new("Dust Attack Effect"),
            Transform::from_translation(dust.translation),
            Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                color: ORANGE.into(),
                ..default()
            },
            sound_effect(
                attacker_assets
                    .steps
                    .choose(&mut global_entropy)
                    .unwrap()
                    .clone(),
            ),
            AttackLine {
                start: source,
                end: dust.translation.truncate(),
            },
        ));
    }
    if score != 0 {
        inventory.dust_data += score;
    }
    Ok(())
}
