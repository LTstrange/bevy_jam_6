use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Spawn the main camera.
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (update_mouse_tracker,).in_set(AppSystems::RecordInput),
    );
}

fn setup(mut commands: Commands) {
    // mouse tracker
    commands.spawn((
        Name::new("Mouse Tracker"),
        MouseTracker,
        Transform::default(),
    ));

    commands.spawn((Name::new("Camera"), Camera2d, MainCamera));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Transform)]
pub struct MouseTracker;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct MainCamera;

fn update_mouse_tracker(
    mut mouse: Single<&mut Transform, With<MouseTracker>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Result {
    let window = windows.single()?;
    let (camera, camera_transform) = camera_q.single()?;
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        mouse.translation = world_position.extend(0.0);
    }

    Ok(())
}
