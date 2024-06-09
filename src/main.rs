use bevy::prelude::*;

mod rdl;

use std::fs;
use rdl::parse_rdl;
use crate::rdl::models::Robot;

#[derive(Resource, Debug)]
struct RobotTwin(Robot);

impl From<Robot> for RobotTwin {
    fn from(robot: Robot) -> Self {
        RobotTwin(robot)
    }
}

fn main() {
    let rdl_content = fs::read_to_string("configs/hello_robot.rdl")
        .expect("Unable to read rdl file");

    let robot = match parse_rdl(&rdl_content) {
        Ok(robot) => robot,
        Err(e) => {
            println!("Error parsing RDL: {}", e);
            return;
        }
    };

    let robot_twin: RobotTwin = robot.into();

    App::new()
        .insert_resource(robot_twin)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands,
         mut materials: ResMut<Assets<StandardMaterial>>,
         mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let grass = Color::Rgba {
        red: 34.0 / 255.0,
        green: 139.0 / 255.0,
        blue: 34.0 / 255.0,
        alpha: 1.0,
    };

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(StandardMaterial {
            base_color: grass,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // Spawn the torso part
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}