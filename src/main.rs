mod setup;

use bevy::prelude::*;
use crate::setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SetupPlugin))
        .add_systems(Startup, add_the_cube)
        .add_systems(Startup, add_child_cube.after(add_the_cube))
        .add_systems(Update, (rotate_the_cube, rotate_child_cube))
        .run();
}

#[derive(Component)]
struct TheCube;

#[derive(Component)]
struct ChildCube;

fn add_the_cube(mut commands: Commands,
                mut materials: ResMut<Assets<StandardMaterial>>,
                mut meshes: ResMut<Assets<Mesh>>) {

    // Add a cube
    commands.spawn((
        Name::new("TheCube"),
        TheCube,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }));
}

fn add_child_cube(
    mut commands: Commands,
    query: Query<Entity, With<TheCube>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if let Ok(parent_entity) = query.get_single() {
        commands.entity(parent_entity).with_children(|parent| {
            parent.spawn((
                Name::new("ChildCube"),
                ChildCube,
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(1.0, 0.0, 0.0), // Different color for distinction
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.75, 0.0),
                    ..default()
                },
            ));
        });
    }
}

fn rotate_the_cube(mut query: Query<&mut Transform, With<TheCube>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation = transform.rotation * Quat::from_rotation_y(2.0 * time.delta_seconds());
    }
}

fn rotate_child_cube(mut query: Query<&mut Transform, With<ChildCube>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation = transform.rotation * Quat::from_rotation_y(0.25 * time.delta_seconds());
    }
}
