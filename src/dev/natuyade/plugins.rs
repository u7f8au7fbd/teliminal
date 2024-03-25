use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder, Startup, Update};

use crate::natuyade::*;

pub struct NatuyadePlugin;

impl PluginGroup for NatuyadePlugin {
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
        app.add_systems(Startup, (dummy_startup, test::print_my_id));
    }
}

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    //アップデート時に実行されるプラグイン
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (dummy_update, dummy::dummy));
    }
}

pub fn dummy_startup() {} //ダミーのスタートアップ関数（関数の追加の例で作成したから消してもいいよ）
pub fn dummy_update() {} //ダミーのアップデート関数（関数の追加の例で作成したから消してもいいよ）
