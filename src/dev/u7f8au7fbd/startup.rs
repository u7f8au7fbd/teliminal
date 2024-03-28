use bevy::{
    audio::{PlaybackMode, SpatialScale, Volume},
    prelude::*,
};
#[derive(Component)]
pub struct Player;

pub fn summon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(128., 128.)),
        material: materials.add(Color::rgb(0.6, 0.6, 0.6)),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 200000.0,
            ..default()
        },
        ..default()
    });

    // Player
    let default_fov = 70.53_f32.to_radians(); //デフォルトの垂直視野角70.53度(水平視野角は103度)
    commands
        .spawn((
            InheritedVisibility::default(),
            Player,
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 4.0, 40.0),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: default_fov,
                    ..default()
                }),
                ..default()
            },
            FogSettings {
                color: Color::rgba(0., 0., 0., 1.0),
                falloff: FogFalloff::Linear {
                    start: 20.0,
                    end: 48.0,
                },
                ..default()
            },
            SpatialListener {
                left_ear_offset: Vec3::new(0.1, 0.0, 0.0),
                right_ear_offset: Vec3::new(-0.1, 0.0, 0.0),
            },
        ))
        .with_children(|interacter| {
            interacter.spawn((PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::rgb(1., 1., 1.)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -4.0),
                    ..default()
                },
                ..default()
            },));
        });

    commands.spawn((
        SceneBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            scene: asset_server.load("models/denwa.glb#Scene0"),
            ..default()
        },
        AudioBundle {
            source: asset_server.load("sounds/denwa.ogg"),
            settings: PlaybackSettings {
                volume: Volume::new(0.2),
                spatial: true,
                mode: PlaybackMode::Loop,
                spatial_scale: Some(SpatialScale::new(0.05)),
                ..default()
            },
        },
    ));
}

pub fn struct_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(4.0, 8.0, 2.0)),
        material: materials.add(Color::rgb_u8(128, 128, 128)),
        transform: Transform::from_xyz(8.0, 0.5, 0.0),
        ..default()
    });
}
