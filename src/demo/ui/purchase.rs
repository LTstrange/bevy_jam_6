use bevy::ecs::relationship::RelatedSpawner;
use bevy::ecs::spawn::SpawnWith;

use super::inventory;
use super::widget;

use crate::demo::ChangePlayerStats;
use crate::demo::gameplay::SpawnAttacker;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ShopState>();
    app.add_observer(update_purchase_ui);
}

macro_rules! row {
    ($item:expr, $price:expr, $event:expr, $upgrade_item:expr) => {
        widget::row(
            $item,
            $price,
            move |_t: Trigger<Pointer<Click>>,
                  mut inventory: ResMut<inventory::Inventory>,
                  mut commands: Commands,
                  mut shop_state: ResMut<ShopState>| {
                if inventory.dust_data >= $price {
                    inventory.dust_data -= $price;
                    shop_state.update_by_event($upgrade_item);
                    commands.trigger($event);
                    commands.trigger(PurchaseUIChanged);
                } else {
                    info!("Not enough data to purchase.");
                }
            },
        )
    };
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

pub trait Upgrades {
    fn row(&self, level: usize) -> Option<impl Bundle>;
}

const ATTACK_UPGRADES: AttackUpgrades = AttackUpgrades {
    item_name: "Upgrade Attack",
    levels: &[(5, 2), (10, 3), (20, 4)],
};

struct AttackUpgrades {
    item_name: &'static str,
    levels: &'static [(u32, u32)],
}

impl Upgrades for AttackUpgrades {
    fn row(&self, level: usize) -> Option<impl Bundle> {
        if let Some(&(price, effect)) = self.levels.get(level) {
            Some(row!(
                self.item_name,
                price,
                ChangePlayerStats::AddAttackEnergy(effect),
                UpgradeItems::AttackUpgrade
            ))
        } else {
            None
        }
    }
}

const BUY_SPAWNERS: BuySpawners = BuySpawners {
    item_name: "Buy Spawner",
    levels: &[50],
};
struct BuySpawners {
    item_name: &'static str,
    levels: &'static [u32],
}

impl Upgrades for BuySpawners {
    fn row(&self, level: usize) -> Option<impl Bundle> {
        if let Some(&price) = self.levels.get(level) {
            Some(row!(
                self.item_name,
                price,
                SpawnAttacker,
                UpgradeItems::BuySpawner
            ))
        } else {
            None
        }
    }
}

#[derive(Resource, Reflect, Debug, Default, Clone)]
#[reflect(Resource)]
pub struct ShopState {
    attack_upgrade_level: usize,
    buy_spawner_level: usize,
}

enum UpgradeItems {
    AttackUpgrade,
    BuySpawner,
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
                if let Some(row) = ATTACK_UPGRADES.row(levels.attack_upgrade_level) {
                    parent.spawn(row);
                }
                if let Some(row) = BUY_SPAWNERS.row(levels.buy_spawner_level) {
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
        }
    }
}
