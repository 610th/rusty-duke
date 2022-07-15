use bevy::{prelude::*, winit::WinitSettings};

mod game;
mod menu;

// Colors
const BACKGROUND_COLOR: Color = Color::DARK_GRAY;
const TEXT_COLOR: Color = Color::ANTIQUE_WHITE;
const NORMAL_BUTTON_COLOR: Color = Color::GRAY;
const HOVERED_BUTTON_COLOR: Color = Color::OLIVE;
const HOVERED_PRESSED_BUTTON_COLOR: Color = Color::DARK_GRAY;
const PRESSED_BUTTON_COLOR: Color = Color::DARK_GRAY;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu, // FIXME: Use one menu state.
    SingleplayerMenu,
    MultiplayerMenu,
    InGameMenu,
    SingleplayerGame,
    MultiplayerGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        // Set init state to main menu.
        .add_state(AppState::MainMenu)
        // Add common startup system
        .add_startup_system(setup)
        // Add common systems
        // Add state specific systems
        // Main menu
        .add_plugin(menu::MenuPlugin)
        // Game (Game screen, board, tiles etc.)
        .add_plugin(game::GamePlugin)
        // Go go go!
        .run();
}

/// Setup common functionality.
fn setup(mut commands: Commands) {
    // 2d camera
    commands.spawn_bundle(Camera2dBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
