//#![windows_subsystem = "windows"]
//Bevy 0.13.1を使用
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::*};
use bevy_egui::EguiPlugin;
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
                        title: "Telminal".into(),                                      //タイトル
                        resolution: (1280.0, 720.0).into(), //ウィンドウサイズ
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
            EguiPlugin,
            U7f8aU7fbdPlugin, //u7f8au7fbdプラグイン
            NatuyadePlugin,   //natuyadeプラグイン
        ))
        //以上は固定
        .run();
}
