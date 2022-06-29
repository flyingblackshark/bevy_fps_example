use bevy::prelude::*;

use heron::prelude::*;

use first_person_camera::*;

mod shooting_system;
mod first_person_camera;
fn main() {
    App::new()
        //.insert_resource(Msaa { samples: 4 })
        .insert_resource(Gravity::from(Vec3::new(0., -9.81, 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(FirstPersonCameraPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_startup_system(setup)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    //HeightField
    commands
        .spawn_bundle((Transform::identity(), GlobalTransform::identity()))
        .insert(RigidBody::Static)
        .insert(CollisionShape::HeightField {
            size: Vec2::new(100., 100.),
            heights: vec![
                vec![0.0,0.0,0.0,0.0,0.0],
                vec![0.0,0.0,0.0,0.0,0.0],
                vec![0.0,0.0,0.0,0.0,0.0],
                vec![0.0,0.0,0.0,0.0,0.0],
                vec![0.0,0.0,0.0,0.0,0.0],
                // vec![1.5, 0.8, 0., 0., 3.0],
                // vec![0.8, 0.2, 0., 0., 3.0],
                // vec![0., 0.5, 0., 0., 3.0],
                // vec![0., 0., 0.6, 0., 3.0],
                // vec![3., 3., 3., 3., 3.0],
            ],
        });
    // Plane
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape:: Plane{size:1000.0})),
    //         material: materials.add(Color::BLUE.into()),
    //         ..Default::default()
    //     })
    //     .insert(Transform {
    //         translation: Vec3::new(0.0,0.0, 0.0),
    //         ..Default::default()
    //     })
    //     .insert(RigidBody::Static)
    //     .insert(CollisionShape::Cuboid {
    //         half_extends: Vec3::new(500.0, 0.01, 500.0),
    //         border_radius: Some(0.0),
    //     });
    //MainCamera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(3.0, 7., -19.0).looking_at(Vec3::new(1., 4., 0.), Vec3::Y),
            ..Default::default()
        })
        // .insert(
        //     Transform {
        //         translation: Vec3::new(3.0, 7., -19.0),
        //         ..Default::default()
        //     }
        //     .looking_at(Vec3::new(1., 4., 0.), Vec3::Y),
        // )
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Capsule {
            radius: 1.0,
            half_segment: 1.0,
        })
        .insert(FirstPersonCamera)
        .insert(RotationConstraints::lock());


    // Cylinder
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 2.0,
                ..Default::default()
            })),
            material: materials.add(Color::GREEN.into()),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(3., 15., -7.),
            ..Default::default()
        })
        .insert(GlobalTransform::identity())
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cylinder {
            half_height: 1.0,
            radius: 0.5,
        });


    // Capsule
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 2.0,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(0., 15., 0.),
            ..Default::default()
        })
        .insert(GlobalTransform::identity())
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Capsule {
            radius: 0.5,
            half_segment: 1.0,
        });

    // light
    // commands.spawn_bundle(PointLightBundle {
    //     transform: Transform::from_xyz(-4.0, 9.0, -4.0),
    //     point_light:PointLight{
    //         range:100.0,
    //         intensity:200.0,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

}