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
        no_effects:
        // $struct_name:ident,      // 结构体名
        $const_name:ident,       // 常量名
        $item_name:expr,         // item_name 字段值
        $cost_type:ty,           // costs 类型
        $cost_value:expr,        // costs 初始化值
        $effect_output:ty,       // impl 中的 Effect 类型
        $effect_fn:expr          // get_current_upgrade 返回的 effect 表达式
    ) => {
        #[allow(non_camel_case_types)]
        struct $const_name {
            item_name: &'static str,
            costs: $cost_type,
        }

        const $const_name: $const_name = $const_name {
            item_name: $item_name,
            costs: $cost_value,
        };

        impl Upgrades for $const_name {
            type Effect = $effect_output;

            fn name(&self) -> &str {
                self.item_name
            }

            fn get_current_upgrade(&self, level: usize) -> Option<(Self::Effect, u32)> {
                let mut combine_iter = self.costs.clone();
                let cost = combine_iter.nth(level)?;
                Some(($effect_fn(), cost))
            }
        }
    };
    // has effects
    (
        has_effects:
        $const_name:ident,       // 常量名
        $item_name:expr,         // item_name 字段值
        $effect_type:ty,         // effects 类型
        $effect_value:expr,      // effects 初始化值
        $cost_type:ty,           // costs 类型
        $cost_value:expr,        // costs 初始化值
        $effect_output:ty,       // impl 中的 Effect 类型
        $effect_fn:expr          // get_current_upgrade 返回的 effect 表达式
    ) => {
        #[allow(non_camel_case_types)]
        struct $const_name {
            item_name: &'static str,
            effects: $effect_type,
            costs: $cost_type,
        }

        const $const_name: $const_name = $const_name {
            item_name: $item_name,
            effects: $effect_value,
            costs: $cost_value,
        };

        impl Upgrades for $const_name {
            type Effect = $effect_output;

            fn name(&self) -> &str {
                self.item_name
            }

            fn get_current_upgrade(&self, level: usize) -> Option<(Self::Effect, u32)> {
                let mut combine_iter = self.effects.clone().zip(self.costs.clone());
                let (effect, cost) = combine_iter.nth(level)?;
                Some(($effect_fn(effect), cost))
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
                        flex_direction: FlexDirection::Column,
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
    has_effects:
    ATTACK_UPGRADES,
    "Upgrade Attack",
    MultiplicativeEffect,
    MultiplicativeEffect::new(5.5, 1.1),
    ExpCosts,
    ExpCosts::new(10.0, 1.2),
    ChangePlayerStats,
    ChangePlayerStats::SetAttackEnergy
);

define_upgrade!(
    no_effects:
    BUY_SPAWNERS,
    "Buy Spawner",
    ExpCosts,
    ExpCosts::new(50.0, 1.2),
    SpawnAttacker,
    || SpawnAttacker // 因为没有 effect 参数
);

define_upgrade!(
    has_effects:
    ENHANCE_POWER_REGEN,
    "Increase Power Regen",
    AdditiveEffect,
    AdditiveEffect::new(6.0, 1.2),
    ExpCosts,
    ExpCosts::new(20.0, 1.3),
    SetPowerStats,
    SetPowerStats::RegenSpeed
);

define_upgrade!(
    has_effects:
    SPEED_UP_DUST_GEN,
    "Speed Up Dust Generation",
    AdditiveEffect,
    AdditiveEffect::new(0.5, 1.2),
    ExpCosts,
    ExpCosts::new(30.0, 1.3),
    SetDustSpawnStats,
    SetDustSpawnStats::SpawnSpeed
);

shop_state!(
    AttackUpgrade -> ATTACK_UPGRADES
    BuySpawner -> BUY_SPAWNERS
    EnhancePowerRegen -> ENHANCE_POWER_REGEN
    SpeedUpDustGen -> SPEED_UP_DUST_GEN
);

// #[derive(Resource, Reflect, Debug, Default, Clone)]
// #[reflect(Resource)]
// pub struct ShopState {
//     attack_upgrade_level: usize,
//     buy_spawner_level: usize,
//     enhance_power_regen_level: usize,
//     speed_up_dust_gen_level: usize,
// }

// #[derive(Clone, Copy)]
// enum UpgradeItems {
//     AttackUpgrade,
//     BuySpawner,
//     EnhancePowerRegen,
//     SpeedUpDustGen,
// }

// impl ShopState {
//     pub fn render(&self) -> impl Bundle {
//         use UpgradeItems::*;
//         let levels = self.clone();
//         (
//             Name::new("Purchase UI"),
//             PurchaseUI,
//             Node {
//                 flex_direction: FlexDirection::Column,
//                 ..default()
//             },
//             Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<_>| {
//                 parent.spawn(widget::header("Research Lab"));
//                 if let Some(row) = ATTACK_UPGRADES.row(levels.attack_upgrade_level, AttackUpgrade) {
//                     parent.spawn(row);
//                 }
//                 if let Some(row) = BUY_SPAWNERS.row(levels.buy_spawner_level, BuySpawner) {
//                     parent.spawn(row);
//                 }
//                 if let Some(row) =
//                     ENHANCE_POWER_REGEN.row(levels.enhance_power_regen_level, EnhancePowerRegen)
//                 {
//                     parent.spawn(row);
//                 }
//                 if let Some(row) =
//                     SPEED_UP_DUST_GEN.row(levels.speed_up_dust_gen_level, SpeedUpDustGen)
//                 {
//                     parent.spawn(row);
//                 }
//             })),
//         )
//     }

//     fn update_by_event(&mut self, item: UpgradeItems) {
//         use UpgradeItems::*;
//         match item {
//             AttackUpgrade => self.attack_upgrade_level += 1,
//             BuySpawner => self.buy_spawner_level += 1,
//             EnhancePowerRegen => self.enhance_power_regen_level += 1,
//             SpeedUpDustGen => self.speed_up_dust_gen_level += 1,
//         }
//     }
// }
