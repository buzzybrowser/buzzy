use bevy::prelude::*;

mod address;
mod ui;

use ui::SetupUIPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupUIPlugin)
        .run();
}
