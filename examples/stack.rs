use bevy::{
    prelude::{shape::Cube, *},
    window::close_on_esc,
};

use bevy_mod_outline::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(OutlinePlugin)
        .add_startup_system(setup)
        .add_system(close_on_esc)
        .add_system(spin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut cube_mesh = Mesh::from(Cube { size: 1.0 });
    cube_mesh.generate_outline_normals().unwrap();
    let cube_mesh = meshes.add(cube_mesh);
    let parent = commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Spins)
        .id();


    for (color, size, z) in [
        (Color::rgb(1.0, 0.0, 0.0), 1.0, 0.45),
        (Color::rgb(1.0, 1.0, 0.0), 2.0, 0.35),
        (Color::rgb(0.0, 1.0, 0.0), 3.0, 0.25),
        (Color::rgb(0.0, 0.0, 1.0), 4.0, 0.15),
        (Color::rgb(1.0, 0.0, 1.0), 5.0, 0.05),
    ] {
        let plate = commands
            .spawn_bundle(PbrBundle {
                mesh: cube_mesh.clone(),
                material: materials.add(color.into()),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::new(size, size, 0.1),
                        Quat::IDENTITY,
                        Vec3::new(2.5-size/2.0, 2.5-size/2.0, z),
                    )),
                ..default()
            })
            .insert_bundle(OutlineBundle {
                outline: Outline {
                    visible: true,
                    colour: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    width: 5.0,
                },
                ..default()
            })
            .id();

        commands.entity(parent).add_child(plate);
    }

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-3.0, -3.0, 2.0),
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-8.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

#[derive(Component)]
struct Spins;

fn spin(mut query: Query<&mut Transform, With<Spins>>, timer: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate(Quat::from_rotation_z(0.4 * timer.delta_seconds()));
    }
}
