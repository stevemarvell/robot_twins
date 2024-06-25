use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

#[derive(Component)]
struct TheCube;

fn setup(mut commands: Commands,
         mut materials: ResMut<Assets<StandardMaterial>>,
         mut meshes: ResMut<Assets<Mesh>>) {

    // Add a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ, -std::f32::consts::FRAC_PI_4, -std::f32::consts::FRAC_PI_4, 0.0,
        )),
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

fn rotate_cube(mut query: Query<&mut Transform, With<TheCube>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation = transform.rotation * Quat::from_rotation_y(2.0 * time.delta_seconds());
    }
}
