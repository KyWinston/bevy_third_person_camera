use bevy::prelude::*;
use bevy_third_person_camera::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ThirdPersonCameraPlugin /* ADD THIS */))
        .add_systems(Startup, (spawn_player, spawn_world, spawn_camera))
        .run();
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneBundle {
            scene: assets.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget, // ADD THIS
    );

    commands.spawn(player);
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera::default(), // ADD THIS
    );
    commands.spawn(camera);
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0))),
        material: materials.add(Color::srgb(0.11, 0.27, 0.16)),
        ..default()
    };

    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(floor);
    commands.spawn(light);
}
