use bevy::winit::cursor::{CursorIcon, CustomCursor, CustomCursorImage};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AssetsState::Loading).load_collection::<CursorAssets>(),
    );
    // Spawn the main camera.
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (update_mouse_tracker,).in_set(AppSystems::RecordInput),
    );

    app.add_observer(
        |t: Trigger<CursorEvents>,
         mut commands: Commands,
         window: Single<Entity, With<Window>>,
         mut cursor_assets: ResMut<CursorAssets>| {
            use CursorEvents::*;
            if let Some((handle, hotspot)) = match (t.event(), cursor_assets.is_dragging) {
                (Over, false) => Some((cursor_assets.hand_open.clone(), (16, 16))),
                (Pressed, false) => {
                    cursor_assets.is_dragging = true;
                    Some((cursor_assets.hand_closed.clone(), (16, 16)))
                }
                (Released, true) => {
                    cursor_assets.is_dragging = false;
                    Some((cursor_assets.hand_open.clone(), (16, 16)))
                }
                (Out, false) => Some((cursor_assets.hand_point.clone(), (8, 6))),
                _ => None,
                //  => (cursor_assets.hand_point.clone(), (8, 6)),
                // Over | Released => (cursor_assets.hand_open.clone(), (16, 16)),
                // Dragging => (cursor_assets.hand_closed.clone(), (16, 16)),
            } {
                commands
                    .entity(*window)
                    .insert(CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
                        handle,
                        hotspot,
                        ..Default::default()
                    })));
            }
        },
    );
}

#[derive(Resource, Reflect, Debug, Default, AssetCollection)]
#[reflect(Resource)]
struct CursorAssets {
    is_dragging: bool,
    #[asset(path = "ui/hand_point.png")]
    hand_point: Handle<Image>,
    #[asset(path = "ui/hand_open.png")]
    hand_open: Handle<Image>,
    #[asset(path = "ui/hand_closed.png")]
    hand_closed: Handle<Image>,
}

#[derive(Event, Debug, Default, Reflect, PartialEq, Eq)]
pub enum CursorEvents {
    #[default]
    Out,
    Over,
    Pressed,
    Released,
}

fn setup(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    asset_server: Res<AssetServer>,
) {
    // mouse tracker
    commands.spawn((
        Name::new("Mouse Tracker"),
        MouseTracker,
        Transform::default(),
    ));
    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: asset_server.load("ui/hand_point.png"),
            hotspot: (8, 6),
            ..Default::default()
        })));

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
