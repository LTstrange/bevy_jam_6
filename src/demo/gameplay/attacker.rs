use bevy::color::palettes::css::*;
use rand::seq::IndexedRandom;

use crate::{
    audio::sound_effect,
    demo::{
        PlayerStats,
        gameplay::{damage::LIGHTING_RANGE, dust::Dust},
    },
    prelude::*,
};

use super::{
    damage::{DamageType, generate_damage},
    power::Power,
};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::Loading).load_collection::<AttackerAssets>(),
    );

    app.add_systems(Update, tick_attacker_timer.in_set(AppSystems::TickTimers));
    app.add_systems(
        Update,
        (attack_dust.run_if(resource_exists::<AttackerAssets>),).in_set(AppSystems::Update),
    );

    app.add_observer(
        |t: Trigger<Pointer<Drag>>, mut attackers: Query<&mut Transform, With<Attacker>>| {
            if let Ok(mut transform) = attackers.get_mut(t.target()) {
                transform.translation.x += t.delta.x;
                transform.translation.y -= t.delta.y;
            }
        },
    );

    app.add_observer(
        |_: Trigger<SpawnAttacker>, mut commands: Commands, mut entropy: GlobalEntropy<WyRand>| {
            commands.spawn(attacker(Vec2::ZERO, 1.0, entropy.fork_rng()));
        },
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Attacker {
    timer: Timer,
}

#[derive(Event, Debug)]
pub struct SpawnAttacker;

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

pub fn attacker(pos: Vec2, attack_interval: f32, entropy: Entropy<WyRand>) -> impl Bundle {
    (
        Name::new("Attacker"),
        Attacker {
            timer: Timer::from_seconds(attack_interval, TimerMode::Once),
        },
        entropy,
        Transform::from_translation(pos.extend(1.0)),
        StateScoped(Screen::Gameplay),
        Sprite::from_color(RED, Vec2::new(16.0, 16.0)),
        Pickable::default(),
    )
}

fn tick_attacker_timer(query: Query<&mut Attacker>, time: Res<Time>) {
    for mut attacker in query {
        attacker.timer.tick(time.delta());
    }
}

fn attack_dust(
    mut commands: Commands,
    attacker: Query<(&mut Attacker, &mut Entropy<WyRand>, &Transform)>,
    player_stats: Res<PlayerStats>,
    attacker_assets: Res<AttackerAssets>,
    power: Res<Power>,
    dust: Query<&Transform, With<Dust>>,
) {
    let mut current_energy = power.current();
    for (mut attacker, mut entropy, attacker_trans) in attacker {
        if attacker.timer.finished() {
            if current_energy < player_stats.attack_energy {
                continue; // No energy to attack
            }
            current_energy -= player_stats.attack_energy;

            let has_dust = dust.iter().any(|dust_trans| {
                let distance = dust_trans
                    .translation
                    .truncate()
                    .distance_squared(attacker_trans.translation.truncate());
                distance < LIGHTING_RANGE * LIGHTING_RANGE
            });
            if !has_dust {
                continue; // No dust in range to attack
            }

            commands.spawn(generate_damage(
                attacker_trans.translation.truncate(),
                player_stats.attack_energy,
                DamageType::Lightning,
                entropy.fork_rng(),
                None,
            ));
            commands.spawn(sound_effect(
                attacker_assets.steps.choose(&mut entropy).unwrap().clone(),
            ));

            // Reset the attack timer
            attacker.timer.reset();
        }
    }
}
