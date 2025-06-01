use bevy::ecs::{relationship::RelatedSpawner, spawn::SpawnWith};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Inventory>()
        .register_type::<Inventory>();

    app.add_systems(
        Update,
        update_inventory_ui.run_if(resource_changed::<Inventory>),
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Inventory {
    pub score: u32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[allow(non_camel_case_types)]
enum InventoryFields {
    score,
}

pub fn inventory_ui() -> impl Bundle {
    (
        Name::new("Inventory UI"),
        Node::default(),
        StateScoped(Screen::Gameplay),
        Text("Score: ".to_string()),
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<_>| {
            parent.spawn((TextSpan::new("0"), InventoryFields::score));
        })),
    )
}

fn update_inventory_ui(
    mut textspans: Query<(&mut TextSpan, &InventoryFields)>,
    inventory: Res<Inventory>,
) {
    for (mut textspan, field) in textspans.iter_mut() {
        match field {
            InventoryFields::score => {
                textspan.0 = inventory.score.to_string();
            }
        }
    }
}
