use bevy::{
    diagnostic::DiagnosticsStore,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
};

use crate::{
    demo::gameplay::DUST_COLLECT_RATE_DIAGNOSTIC,
    prelude::*,
    theme::widget::{header, label},
};

pub(super) fn plugin(app: &mut App) {
    // app.init_resource::<Inventory>();
    app.register_type::<Inventory>();
    app.insert_resource(Inventory {
        dust_data: 0,
        collect_rate: 0.0,
    });

    app.add_systems(
        Update,
        update_inventory_ui.run_if(resource_changed::<Inventory>),
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Inventory {
    pub dust_data: u32,
    pub collect_rate: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[allow(non_camel_case_types)]
enum InventoryFields {
    dust_data,
    collect_rate,
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
            row("Collect Rate: ", InventoryFields::collect_rate),
        ],
    )
}

fn update_inventory_ui(
    mut textspans: Query<(&mut TextSpan, &InventoryFields)>,
    inventory: Res<Inventory>,
    diagnostics: Res<DiagnosticsStore>,
) -> Result {
    for (mut textspan, field) in textspans.iter_mut() {
        match field {
            InventoryFields::dust_data => {
                textspan.0 = inventory.dust_data.to_string();
            }
            InventoryFields::collect_rate => {
                if let Some(rate) = diagnostics
                    .get(&DUST_COLLECT_RATE_DIAGNOSTIC)
                    .and_then(|rate| rate.smoothed())
                {
                    textspan.0 = format!("{:.1} dust/s", rate);
                } else {
                    textspan.0 = "Err dust/s".to_string();
                }
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
