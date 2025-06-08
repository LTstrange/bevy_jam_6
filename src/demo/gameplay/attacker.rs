use bevy::color::palettes::{css::*, tailwind::*};
use rand::seq::IndexedRandom;

use crate::{
    CursorEvents,
    audio::sound_effect,
    demo::{
        GAME_AREA, PlayerStats,
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
        (
            charge_attacker,
            update_attacker_color,
            attack_dust.run_if(resource_exists::<AttackerAssets>),
        )
            .chain()
            .in_set(AppSystems::Update),
    );

    app.add_observer(
        |_: Trigger<SpawnAttacker>, mut commands: Commands, mut entropy: GlobalEntropy<WyRand>| {
            let attacker = commands.spawn(attacker(Vec2::ZERO, 1.0, entropy.fork_rng()));
            setup_cursor_icon(attacker);
        },
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Attacker {
    timer: Timer,
    fully_charged: bool,
}

#[derive(Event, Debug, Clone)]
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
            fully_charged: false,
        },
        entropy,
        Transform::from_translation(pos.extend(1.0)),
        StateScoped(Screen::Gameplay),
        Sprite::from_color(RED, Vec2::new(16.0, 16.0)),
        Pickable::default(),
    )
}

fn update_attacker_color(attacker: Query<(&Transform, &Attacker, &mut Sprite)>) {
    for (transform, attacker, mut sprite) in attacker {
        // if attacker out of bounds, set color to BLACK
        if !GAME_AREA.contains(transform.translation.truncate()) {
            sprite.color = BLACK.into();
            continue;
        }
        // mix color based on timer progress, from dark red to red
        let progress = attacker.timer.fraction();
        let progress = map_range(progress, 0.0..1.0, 0.5..1.0);
        sprite.color = RED.mix(&BLACK, 1.0 - progress).into();
        // if fully charged, set color to BLUE
        if attacker.fully_charged {
            sprite.color = YELLOW_300.into();
        }
    }
}

fn tick_attacker_timer(query: Query<&mut Attacker>, time: Res<Time>) {
    for mut attacker in query {
        attacker.timer.tick(time.delta());
    }
}

fn charge_attacker(
    mut attackers: Query<&mut Attacker>,
    mut power: ResMut<Power>,
    player_stats: Res<PlayerStats>,
    mut rng: GlobalEntropy<WyRand>,
) {
    if power.current() < player_stats.attack_energy {
        return; // Not enough power to charge any attacker
    }
    let mut attackers = attackers
        .iter_mut()
        .filter(|attacker| !attacker.fully_charged)
        .filter(|attacker| attacker.timer.finished())
        .collect::<Vec<_>>();

    // charge in random order
    attackers.shuffle(&mut rng);
    for mut attacker in attackers {
        if attacker.timer.finished() && power.current() >= player_stats.attack_energy {
            power.consume(player_stats.attack_energy);
            attacker.fully_charged = true;
        }
    }
}

fn attack_dust(
    mut commands: Commands,
    attacker: Query<(&mut Attacker, &mut Entropy<WyRand>, &Transform)>,
    player_stats: Res<PlayerStats>,
    attacker_assets: Res<AttackerAssets>,
    dust: Query<&Transform, With<Dust>>,
) {
    for (mut attacker, mut entropy, attacker_trans) in attacker {
        if !GAME_AREA.contains(attacker_trans.translation.truncate()) {
            continue; // Attacker is out of bounds
        }
        if !attacker.fully_charged {
            continue; // Attacker is not fully charged
        }

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

        // Reset the attack timer and fully charged state
        attacker.timer.reset();
        attacker.fully_charged = false;
    }
}

fn setup_cursor_icon(mut attacker: EntityCommands) {
    attacker.observe(|_: Trigger<Pointer<Over>>, mut commands: Commands| {
        commands.trigger(CursorEvents::Over);
    });
    attacker.observe(|_: Trigger<Pointer<Out>>, mut commands: Commands| {
        commands.trigger(CursorEvents::Out);
    });

    attacker.observe(|_: Trigger<Pointer<Pressed>>, mut commands: Commands| {
        commands.trigger(CursorEvents::Pressed);
    });
    attacker.observe(|_: Trigger<Pointer<Released>>, mut commands: Commands| {
        commands.trigger(CursorEvents::Released);
    });

    attacker.observe(
        |t: Trigger<Pointer<Drag>>, mut attackers: Query<&mut Transform, With<Attacker>>| {
            if let Ok(mut transform) = attackers.get_mut(t.target()) {
                transform.translation.x += t.delta.x;
                transform.translation.y -= t.delta.y;
            }
        },
    );
}
