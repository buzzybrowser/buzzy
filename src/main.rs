use bevy::prelude::*;

mod address;
mod ui;

use address::AddressPlugin;
use ui::SetupUIPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupUIPlugin)
        .add_plugin(AddressPlugin)
        .run();
}
