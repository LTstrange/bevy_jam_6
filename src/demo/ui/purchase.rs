use bevy::ecs::relationship::RelatedSpawner;
use bevy::ecs::spawn::SpawnWith;

use super::widget;

use crate::demo::ChangePlayerStats;
use crate::demo::gameplay::SetDustSpawnStats;
use crate::demo::gameplay::SetPowerStats;
use crate::demo::gameplay::SpawnAttacker;
use crate::prelude::*;

mod types;
use types::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ShopState>();
    app.add_observer(update_purchase_ui);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PurchaseUI;

#[derive(Event, Debug)]
struct PurchaseUIChanged;

fn update_purchase_ui(
    _: Trigger<PurchaseUIChanged>,
    ui: Single<(Entity, &ChildOf), With<PurchaseUI>>,
    mut commands: Commands,
    shop_state: Res<ShopState>,
) {
    let (ui, parent) = *ui;
    commands.entity(ui).despawn();
    commands.entity(parent.0).with_child(shop_state.render());
}

macro_rules! define_upgrade {
    (
        $const_name:ident,       // 常量名
        $item_name:expr,         // item_name 字段值
        $tips:expr,         // tips 字段值
        $effect_type:ty,         // effects 类型
        $effect_value:expr,      // effects 初始化值
        $cost_type:ident::new($cost_init:expr, $cost_ratio:expr), // costs 类型和初始化值
        $event_type:ty,       // 输出event 的类型
        $effect_fn:expr        // 输出effect event 的函数
    ) => {
        #[allow(non_camel_case_types)]
        struct $const_name {
            item_name: &'static str,
            tips: &'static str,
            effects: $effect_type,
            costs: $cost_type,
        }

        const $const_name: $const_name = $const_name {
            item_name: $item_name,
            tips: $tips,
            effects: $effect_value,
            costs: $cost_type::new($cost_init, $cost_ratio),
        };

        impl Upgrades for $const_name {
            type Effect = $event_type;

            fn name(&self) -> &'static str {
                self.item_name
            }
            fn tips(&self) -> &'static str {
                self.tips
            }

            fn get_current_upgrade(&self, level: usize) -> Option<(Self::Effect, (f32, f32), u32)> {
                let mut combine_iter = self.effects.clone().zip(self.costs.clone());
                let (cur_effect, cost) = combine_iter.nth(level)?;
                let (effect, _) = combine_iter.next()?;
                Some(($effect_fn(effect), (cur_effect, effect), cost))
            }
        }
    };
}

macro_rules! shop_state {
    ($($field:ident -> $target:ident)+) => {
        #[derive(Resource, Reflect, Debug, Default, Clone)]
        #[reflect(Resource)]
        #[allow(non_snake_case)]
        pub struct ShopState {
            $(
                $field: usize,
            )+
        }
        #[derive(Clone, Copy)]
        enum UpgradeItems {
            $(
                $field,
            )+
        }

        impl ShopState {
            pub fn render(&self) -> impl Bundle {
                use UpgradeItems::*;
                let levels = self.clone();
                (
                    Name::new("Purchase UI"),
                    PurchaseUI,
                    Node {
                        width: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<_>| {
                        parent.spawn(widget::header("Research Lab"));
                        $(
                            if let Some(row) = $target.row(levels.$field, $field) {
                                parent.spawn(row);
                            }
                        )+
                    })),
                )
            }

            fn update_by_event(&mut self, item: UpgradeItems) {
                use UpgradeItems::*;
                match item {
                    $(
                        $field => self.$field += 1,
                    )+
                    // AttackUpgrade => self.attack_upgrade_level += 1,
                    // BuySpawner => self.buy_spawner_level += 1,
                    // EnhancePowerRegen => self.enhance_power_regen_level += 1,
                    // SpeedUpDustGen => self.speed_up_dust_gen_level += 1,
                }
            }
        }
    };
}

define_upgrade!(
    ATTACK_UPGRADES,
    "Attack Amount",
    "Max release",
    MultiplicativeEffect,
    MultiplicativeEffect::new(5.0, 1.1),
    ExpCosts::new(10.0, 1.2),
    ChangePlayerStats,
    ChangePlayerStats::SetAttackEnergy
);

define_upgrade!(
    NEW_ATTACKER,
    "New Attacker",
    "Number",
    AdditiveEffect,
    AdditiveEffect::new(1.0, 1.0),
    ExpCosts::new(50.0, 1.2),
    SpawnAttacker,
    |_| SpawnAttacker // 因为没有 event 参数
);

define_upgrade!(
    ENHANCE_POWER_REGEN,
    "Charge Power",
    "Amount per sec",
    MultiplicativeEffect,
    MultiplicativeEffect::new(5.0, 1.5),
    ExpCosts::new(20.0, 1.6),
    SetPowerStats,
    SetPowerStats::RegenSpeed
);

define_upgrade!(
    SPEED_UP_DUST_GEN,
    "Dust Generation",
    "Number per sec",
    AdditiveEffect,
    AdditiveEffect::new(2.0, 1.2),
    ExpCosts::new(30.0, 1.3),
    SetDustSpawnStats,
    SetDustSpawnStats::SpawnSpeed
);

define_upgrade!(
    UPGRADE_POWER_MAX,
    "Power Max",
    "Maximum",
    AdditiveEffect,
    AdditiveEffect::new(20.0, 1.2),
    ExpCosts::new(40.0, 1.3),
    SetPowerStats,
    SetPowerStats::PowerMax
);

shop_state!(
    AttackUpgrade -> ATTACK_UPGRADES
    NewAttacker -> NEW_ATTACKER
    EnhancePowerRegen -> ENHANCE_POWER_REGEN
    SpeedUpDustGen -> SPEED_UP_DUST_GEN
    UpgradePowerMax -> UPGRADE_POWER_MAX
);
