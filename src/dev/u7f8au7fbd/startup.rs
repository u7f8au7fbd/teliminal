use bevy::{
    audio::{PlaybackMode, SpatialScale, Volume},
    prelude::*,
};
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct DebugText;

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

    // camera
    commands.spawn((
        Player,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 4.0, 40.0),
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
    ));

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

pub fn debug_text(mut commands: Commands) {
    commands.spawn((
        DebugText,
        TextBundle {
            text: Text::from_section(
                "Null",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            ..default()
        },
    ));
}
