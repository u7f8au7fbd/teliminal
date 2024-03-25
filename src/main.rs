//#![windows_subsystem = "windows"]
//Bevy 0.13.1を使用
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::*};
use provatheus::*;
mod dev;
mod provatheus;
use dev::{natuyade::plugins::NatuyadePlugin, u7f8au7fbd::plugins::U7f8aU7fbdPlugin, *};
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Project Liminal".into(),    //タイトル
                        resolution: (1280.0, 960.0).into(), //ウィンドウサイズ
                        position: WindowPosition::Centered(MonitorSelection::Primary), //ウィンドウの生成座標を中心に設定
                        present_mode: PresentMode::AutoNoVsync,                        //Vsync有効
                        resizable: false, //サイズ変更不可
                        enabled_buttons: bevy::window::EnabledButtons {
                            minimize: false, //最小化無効
                            maximize: false, //最大化無効
                            close: true,     //閉じる有効
                        },
                        visible: false, //非表示
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), //デフォルトの画像処理をピクセルパーフェクトに設定
        )
        .insert_resource(ClearColor(Color::NONE)) //デフォルトの背景色を設定
        .insert_resource(Msaa::Off) //MSAAを無効化
        .add_plugins((
            ProvatheusPlugin,           //プロヴァテウスプラグイン
            FrameTimeDiagnosticsPlugin, //フレームタイムの診断プラグイン
            U7f8aU7fbdPlugin,           //u7f8au7fbdプラグイン
            NatuyadePlugin,             //natuyadeプラグイン
        ))
        .add_systems(Startup, test_st)
        //以上は固定
        .run();
}

fn test_st(
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
        Camera3dBundle {
            transform: Transform::from_xyz(16.0, 6.0, 16.0)
                .looking_at(Vec3::new(0., 2., 0.), Vec3::Y),
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
    ));

    commands.spawn(SceneBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        scene: asset_server.load("models/denwa.glb#Scene0"),
        ..default()
    });
}
