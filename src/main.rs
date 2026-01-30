use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowPlugin};
use bevy_tweening::TweeningPlugin;

use crate::game::setup_orchestrator;

mod assets;
mod events;
mod galaxy;
mod game;
mod ui;

pub fn main() -> Result<(), String> {
    let mut app = App::new();
    app.add_plugins((
        // Full screen
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Index(0)),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(LogPlugin {
                // Show INFO for the game, but only ERROR for bevy and wgpu
                filter: "info,bevy_render=error,bevy_ecs=error,wgpu=error".into(),
                level: Level::INFO,
                ..default()
            }),
    ))
    .add_plugins(TweeningPlugin)
    .add_systems(PreStartup, assets::load_assets)
    .add_systems(
        Startup,
        (
            game::setup_orchestrator,
            galaxy::setup.after(setup_orchestrator),
            ui::draw_entity_info_menu.after(setup_orchestrator),
            ui::draw_game_options_menu,
        ),
    )
    .add_systems(Update,(
            ui::button_hover, 
            ui::menu_action,
            ui::send_scroll_events,
            galaxy::despawn_celestial,
            galaxy::update_selected_planet,
            game::log_text
        ),
    )
    .add_systems(FixedUpdate, (game::game_loop, galaxy::draw_topology))
    .add_observer(galaxy::destroy_link)
    .add_observer(galaxy::move_celestial)
    .add_observer(ui::on_scroll_handler);
    app.run();
    Ok(())
}
