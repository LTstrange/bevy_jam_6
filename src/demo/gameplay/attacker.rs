use bevy::color::palettes::css::*;
use rand::seq::IndexedRandom;

use crate::{audio::sound_effect, demo::PlayerStats, prelude::*};

use super::damage::{DamageType, generate_damage};

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::Loading).load_collection::<AttackerAssets>(),
    );

    app.add_systems(Update, tick_attacker_timer.in_set(AppSystems::TickTimers));
    app.add_systems(
        Update,
        (attack_dust.run_if(resource_exists::<AttackerAssets>),).in_set(AppSystems::Update),
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

pub fn attacker(pos: Vec2, attack_interval: f32, entropy: Entropy<WyRand>) -> impl Bundle {
    (
        Name::new("Attacker"),
        Attacker {
            timer: Timer::from_seconds(attack_interval, TimerMode::Once),
        },
        entropy,
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

fn attack_dust(
    mut commands: Commands,
    attacker: Query<(&mut Attacker, &mut Entropy<WyRand>, &Transform)>,
    player_stats: Res<PlayerStats>,
    attacker_assets: Res<AttackerAssets>,
) {
    for (mut attacker, mut entropy, transform) in attacker {
        if attacker.timer.just_finished() {
            commands.spawn(generate_damage(
                transform.translation.truncate(),
                player_stats.attack_energy,
                DamageType::Lightning,
                entropy.fork_rng(),
            ));
            commands.spawn(sound_effect(
                attacker_assets.steps.choose(&mut entropy).unwrap().clone(),
            ));
            // Reset the attack timer
            attacker.timer.reset();
        }
    }
}
