use bevy::ecs::relationship::RelatedSpawner;
use bevy::ecs::spawn::SpawnWith;

use super::widget;

use crate::demo::ChangePlayerStats;
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
        $struct_name:ident,      // 结构体名
        $const_name:ident,       // 常量名
        $item_name:expr,         // item_name 字段值
        $cost_type:ty,           // costs 类型
        $cost_value:expr,        // costs 初始化值
        $effect_output:ty,       // impl 中的 Effect 类型
        $effect_fn:expr          // get_current_upgrade 返回的 effect 表达式
    ) => {
        struct $struct_name {
            item_name: &'static str,
            costs: $cost_type,
        }

        const $const_name: $struct_name = $struct_name {
            item_name: $item_name,
            costs: $cost_value,
        };

        impl Upgrades for $struct_name {
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
    (
        $struct_name:ident,      // 结构体名
        $const_name:ident,       // 常量名
        $item_name:expr,         // item_name 字段值
        $effect_type:ty,         // effects 类型
        $effect_value:expr,      // effects 初始化值
        $cost_type:ty,           // costs 类型
        $cost_value:expr,        // costs 初始化值
        $effect_output:ty,       // impl 中的 Effect 类型
        $effect_fn:expr          // get_current_upgrade 返回的 effect 表达式
    ) => {
        struct $struct_name {
            item_name: &'static str,
            effects: $effect_type,
            costs: $cost_type,
        }

        const $const_name: $struct_name = $struct_name {
            item_name: $item_name,
            effects: $effect_value,
            costs: $cost_value,
        };

        impl Upgrades for $struct_name {
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

define_upgrade!(
    AttackUpgrades,
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
    BuySpawners,
    BUY_SPAWNERS,
    "Buy Spawner",
    ExpCosts,
    ExpCosts::new(50.0, 1.2),
    SpawnAttacker,
    || SpawnAttacker // 因为没有 effect 参数
);

define_upgrade!(
    EnhancePowerRegen,
    ENHANCE_POWER_REGEN,
    "Increase Power Regen",
    AdditiveEffect,
    AdditiveEffect::new(6.0, 1.2),
    ExpCosts,
    ExpCosts::new(20.0, 1.3),
    SetPowerStats,
    SetPowerStats::RegenSpeed
);

#[derive(Resource, Reflect, Debug, Default, Clone)]
#[reflect(Resource)]
pub struct ShopState {
    attack_upgrade_level: usize,
    buy_spawner_level: usize,
    enhance_power_regen_level: usize,
}

#[derive(Clone, Copy)]
enum UpgradeItems {
    AttackUpgrade,
    BuySpawner,
    EnhancePowerRegen,
}

impl ShopState {
    pub fn render(&self) -> impl Bundle {
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
                if let Some(row) =
                    ATTACK_UPGRADES.row(levels.attack_upgrade_level, UpgradeItems::AttackUpgrade)
                {
                    parent.spawn(row);
                }
                if let Some(row) =
                    BUY_SPAWNERS.row(levels.buy_spawner_level, UpgradeItems::BuySpawner)
                {
                    parent.spawn(row);
                }
                if let Some(row) = ENHANCE_POWER_REGEN.row(
                    levels.enhance_power_regen_level,
                    UpgradeItems::EnhancePowerRegen,
                ) {
                    parent.spawn(row);
                }
            })),
        )
    }

    fn update_by_event(&mut self, item: UpgradeItems) {
        use UpgradeItems::*;
        match item {
            AttackUpgrade => self.attack_upgrade_level += 1,
            BuySpawner => self.buy_spawner_level += 1,
            EnhancePowerRegen => self.enhance_power_regen_level += 1,
        }
    }
}
