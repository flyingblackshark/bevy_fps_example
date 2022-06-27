use bevy::prelude::*;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use heron::prelude::*;
pub struct FirstPersonCameraPlugin;
//Modified https://github.com/sburris0/bevy_flycam
#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

/// Mouse sensitivity and movement speed
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}
/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

impl Plugin for FirstPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
        .init_resource::<MovementSettings>()
        .add_startup_system(initial_grab_cursor)
        .add_system(player_move)
        .add_system(player_look)
        .add_system(player_shooting)
        .add_system(cursor_grab);
    }
}
#[derive(Debug, Component)]
pub struct FirstPersonCamera;


fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    mut query: Query<&mut Transform, With<FirstPersonCamera>>,
) {
    let window = windows.get_primary().unwrap();
    for mut transform in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            if window.cursor_locked() {
                match key {
                    KeyCode::W => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::A => velocity -= right,
                    KeyCode::D => velocity += right,
                    KeyCode::Space => velocity.y = 9.8,
                   // KeyCode::LShift => velocity -= Vec3::Y,
                    _ => (),
                }
            }
        }

        velocity = velocity.normalize_or_zero();

        transform.translation += velocity * time.delta_seconds() * settings.speed
    }
}
fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FirstPersonCamera>>,
) {
    let window = windows.get_primary().unwrap();
    let mut delta_state = state.as_mut();
    for mut transform in query.iter_mut() {
        for ev in delta_state.reader_motion.iter(&motion) {
            if window.cursor_locked() {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                delta_state.pitch -=
                    (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                delta_state.yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
            }

            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
        }
    }
}
fn player_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<MouseButton>>,

    windows: Res<Windows>,
    
    mut query: Query<&mut Transform, With<FirstPersonCamera>>,
) {
    let window = windows.get_primary().unwrap();
    for mut transform in query.iter_mut() {
        for key in keys.get_pressed() {
            if window.cursor_locked() {
                match key {
                   MouseButton::Left=>{
                    //shoot
                    commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                        material: materials.add(Color::BLUE.into()),
                        ..Default::default()
                    })
                    .insert(Transform {
                        translation: transform.translation+transform.rotation.mul_vec3(transform.translation).normalize()*0.5,
                        ..Default::default()
                    })
                    .insert(RigidBody::Dynamic)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new(0.1, 0.1, 0.1),
                        border_radius: Some(0.3),
                    })
                    .insert(Velocity::from_linear(transform.rotation.mul_vec3(transform.translation).normalize()*50.0));
                   },
                    _ => (),
                }
            }
        }
    }
}
// fn shoot_bullet(  commands: &Commands,
//    meshes: &ResMut<Assets<Mesh>>,
//     materials: &ResMut<Assets<StandardMaterial>>,
//     ){
    
// }
fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}