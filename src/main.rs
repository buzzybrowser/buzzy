use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

struct Address(String);

pub struct SetupUIPlugin;

impl Plugin for SetupUIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Set up UI data
        app.insert_resource(Address("".to_owned()));
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SetupUIPlugin)
        .add_system(ui_example.system())
        .run();
}

fn ui_example(mut address: ResMut<Address>, egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx();
    // Add top panel
    // TODO: Remove top and side borders like Chrome or Firefox.
    egui::TopPanel::top("top_panel").show(ctx, |ui| {
        // TODO: Make it look more like Chrome or Firefox
        ui.set_min_height(28.0); // 28.0 pixels looks good with default buttons
                                 // Use `with_layout` instead of `horizontal` to center widgets.
        ui.with_layout(egui::Layout::left_to_right(), |ui| {
            // Add buttons
            // TODO: Adjust size of widgets to make them look more like Chrome or Firefox.
            if ui.button("⬅").clicked() {
                println!("Back clicked!");
            }
            if ui.button("➡").clicked() {
                println!("Forward clicked!")
            }
            if ui.button("🔃").clicked() {
                println!("Reload clicked!")
            };
            if ui.button("🏠").clicked() {
                println!("Home clicked!")
            }

            // Add address bar
            // FIXME: Holding down backspace doesn't work?
            // TODO: Select all text when clicking if there is text
            // TODO: Make address part of the tab instead of having a single one.
            let separator_and_menu_button_width = 50.0;
            ui.add(
                egui::TextEdit::singleline(&mut address.0)
                    .hint_text("Search with Google or enter address")
                    .id_source("address_bar")
                    // Expand to take up the rest of the space except for enough to
                    // fit the separator and menu button.
                    // FIXME: Is there a less hacky way to do this?
                    .desired_width(ui.available_width() - separator_and_menu_button_width),
            );

            // Add separator and menu
            ui.separator();
            if ui.button("☰").clicked() {
                println!("Menu clicked!");
            }
        });
    });
    // Add central panel
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hello world!");
    });
}
