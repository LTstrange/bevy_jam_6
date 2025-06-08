use bevy::ecs::{relationship::RelatedSpawner, spawn::SpawnWith};

use crate::{
    prelude::*,
    theme::widget::{header, label},
};

pub(super) fn plugin(app: &mut App) {
    // app.init_resource::<Inventory>();
    app.register_type::<Inventory>();
    app.insert_resource(Inventory {
        dust_data: 0,
        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
    });

    app.add_systems(
        Update,
        (
            update_inventory_ui.run_if(resource_changed::<Inventory>),
            add_dust_data_constantly.run_if(in_state(Screen::Gameplay)),
        )
            .in_set(AppSystems::Update),
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Inventory {
    pub dust_data: u32,
    timer: Timer,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[allow(non_camel_case_types)]
enum InventoryFields {
    dust_data,
}

pub fn inventory_ui() -> impl Bundle {
    (
        Name::new("Inventory UI"),
        Node {
            width: Val::Px(400.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        StateScoped(Screen::Gameplay),
        children![
            header("Data Center"),
            row("Dust Data: ", InventoryFields::dust_data),
        ],
    )
}

fn add_dust_data_constantly(mut inventory: ResMut<Inventory>, time: Res<Time>) {
    if inventory.timer.tick(time.delta()).just_finished() {
        inventory.dust_data += 1; // Increment dust data every second
    }
}

fn update_inventory_ui(
    mut textspans: Query<(&mut TextSpan, &InventoryFields)>,
    inventory: Res<Inventory>,
) -> Result {
    for (mut textspan, field) in textspans.iter_mut() {
        match field {
            InventoryFields::dust_data => {
                textspan.0 = inventory.dust_data.to_string();
            }
        }
    }
    Ok(())
}

fn row(label_text: impl Into<String>, ui_marker: InventoryFields) -> impl Bundle {
    (
        label(label_text.into()),
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<_>| {
            parent.spawn((TextSpan::new("0"), ui_marker));
        })),
    )
}
