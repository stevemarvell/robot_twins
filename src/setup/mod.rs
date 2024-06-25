use bevy::prelude::*;

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

    // Add some grass
    let grass = Color::Rgba {
        red: 34.0 / 255.0,
        green: 139.0 / 255.0,
        blue: 34.0 / 255.0,
        alpha: 1.0,
    };

    #[derive(Component)]
    struct Terrain;

    commands.spawn((
        Name::new("Terrain"),
        Terrain,
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(StandardMaterial {
                base_color: grass,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        }));
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
