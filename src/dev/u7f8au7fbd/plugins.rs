use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder, Startup, Update};

use crate::u7f8au7fbd::*; //モジュールのインポート

pub struct U7f8aU7fbdPlugin;

impl PluginGroup for U7f8aU7fbdPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UpdatePlugin)
            .add(StartupPlugin)
    }
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    //スタートアップ時に実行されるプラグイン
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup::set_camera);
    }
}

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    //アップデート時に実行されるプラグイン
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update::move_camera);
    }
}
