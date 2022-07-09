use crate::first_person_camera::*;
use crate::object_system::*;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use heron::rapier_plugin::PhysicsWorld;

pub struct ShootingSystemPlugin;

impl Plugin for ShootingSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_shooting);
    }
}
fn player_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<MouseButton>>,
    world: PhysicsWorld,
    windows: Res<Windows>,

    mut main_camera_query: Query<&mut Transform, With<FirstPersonCamera>>,
    mut enemy_query: Query<(Entity, &mut HealthStatus), With<HealthStatus>>,
) {
    let window = windows.get_primary().unwrap();
    //for mut transform in query.iter_mut() {
    for key in keys.get_pressed() {
        if window.cursor_locked() {
            match key {
                MouseButton::Left => {
                    let transform = main_camera_query.single();
                    let result = world.ray_cast(
                        transform.translation + transform.forward(),
                        transform.forward() * Vec3::new(100.0, 100.0, 100.0),
                        true,
                    );
                    if let Some(shoot_result) = result {
                        let target_entity = shoot_result.entity;
                        for (i,mut j) in enemy_query.iter_mut() {
                            if i.id() == target_entity.id() {

                                        if j.hp<=0 {
                                            println!("target Dead!");
                                            commands.entity(i).despawn();
                                        }else{
                                            j.hp-=10;
                                            println!("target Damaged! Left Hp {}",j.hp);
                                        }
                               
                                        
                           
                                }
                            }
                        

                        println!("{:?}", target_entity.id());
                        commands.entity(target_entity);
                    }
                    // //shoot
                    // commands
                    // .spawn_bundle(PbrBundle {
                    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    //     material: materials.add(Color::BLUE.into()),
                    //     ..Default::default()
                    // })
                    // .insert(Transform {
                    //     translation: transform.translation+transform.forward()*0.3,
                    //     ..Default::default()
                    // })
                    // .insert(RigidBody::Dynamic)
                    // .insert(GlobalTransform::identity())
                    // .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
                    // .insert(CollisionShape::Cuboid {
                    //     half_extends: Vec3::new(0.1, 0.1, 0.1),
                    //     border_radius: None,
                    // })
                    // .insert(Velocity::from_linear(transform.forward()*50.0));
                    }
                _ => (),
            
        }
        }
    }
}
