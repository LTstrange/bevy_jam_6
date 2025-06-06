use crate::{
    demo::ui::{
        inventory,
        purchase::{PurchaseUIChanged, ShopState, UpgradeItems},
        widget,
    },
    prelude::*,
};

pub trait Upgrades {
    type Effect: Event + Clone;
    fn name(&self) -> &'static str;
    fn get_current_upgrade(&self, level: usize) -> Option<(Self::Effect, u32)>;
    fn row(&self, level: usize, item: UpgradeItems) -> Option<impl Bundle> {
        if let Some((effect, cost)) = self.get_current_upgrade(level) {
            Some(widget::row(
                self.name(),
                cost,
                move |_t: Trigger<Pointer<Click>>,
                      mut inventory: ResMut<inventory::Inventory>,
                      mut commands: Commands,
                      mut shop_state: ResMut<ShopState>| {
                    if inventory.dust_data >= cost {
                        inventory.dust_data -= cost;
                        shop_state.update_by_event(item);
                        commands.trigger(effect.clone());
                        commands.trigger(PurchaseUIChanged);
                    } else {
                        info!("Not enough data to purchase.");
                    }
                },
            ))
        } else {
            None
        }
    }
}

/// factor * base^k
#[derive(Debug, Clone)]
pub struct ExpCosts {
    factor: f32,
    base: f32,
    k: usize,
}

impl ExpCosts {
    pub const fn new(factor: f32, base: f32) -> Self {
        Self { factor, base, k: 0 }
    }
}

impl Iterator for ExpCosts {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let cost = self.factor * self.base.powi(self.k as i32);
        self.k += 1;
        Some(cost as u32)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let cost = self.factor * self.base.powi((self.k + n) as i32);
        self.k += n;
        Some(cost as u32)
    }
}

/// v_n = v_0 + k * dv
#[derive(Debug, Clone)]
pub struct AdditiveEffect {
    initial_value: f32,
    increment: f32,
    k: usize,
}

impl AdditiveEffect {
    pub const fn new(initial_value: f32, increment: f32) -> Self {
        Self {
            initial_value,
            increment,
            k: 0,
        }
    }
}

impl Iterator for AdditiveEffect {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.initial_value + self.increment * (self.k as f32);
        self.k += 1;
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let value = self.initial_value + self.increment * (self.k + n) as f32;
        self.k += n;
        Some(value)
    }
}

/// v_n = v_0 * r^k
#[derive(Debug, Clone)]
pub struct MultiplicativeEffect {
    initial_value: f32,
    ratio: f32,
    k: usize,
}
impl MultiplicativeEffect {
    pub const fn new(initial_value: f32, ratio: f32) -> Self {
        Self {
            initial_value,
            ratio,
            k: 0,
        }
    }
}
impl Iterator for MultiplicativeEffect {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.initial_value * self.ratio.powi(self.k as i32);
        self.k += 1;
        Some(value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let value = self.initial_value * self.ratio.powi((self.k + n) as i32);
        self.k += n;
        Some(value)
    }
}
