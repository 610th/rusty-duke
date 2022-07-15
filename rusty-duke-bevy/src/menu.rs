use std::time::Duration;

use crate::*;
use bevy::app::AppExit;

// Much of the code in this file is derived from the Bevy 0.7 game_menu example.
const MIN_AI_LEVEL: u8 = 2;
const MIN_GAME_TIME: u8 = 0;
const MIN_GAME_TIME_INCREMENT: u8 = 0;

// Components
#[derive(Component)]
pub enum MenuButtonAction {
    MainMenu,
    SingleplayerMenu,
    MultiplayerMenu,
    InGameMenu,
    Play,
    Quit,
    IncreaseAI,
    DecreaseAI,
    IncreaseGameTime,
    DecreaseGameTime,
    IncreaseGameTimeIncrement,
    DecreaseGameTimeIncrement,
}
#[derive(Component)]
struct OnMainMenuScreen;
#[derive(Component)]
struct OnSingleplayerMenuScreen;
#[derive(Component)]
struct OnMultiplayerMenuScreen;
#[derive(Component)]
struct OnInGameMenuScreen;

// Resources
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct AiLevel(pub u8);
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct GameTime(pub Duration);
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct GameTimeIncrement(pub Duration);

// Plugins
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // FIXME: Use menu sub-states
            .insert_resource(AiLevel(6))
            .insert_resource(GameTime(Duration::from_secs(15 * 60)))
            .insert_resource(GameTimeIncrement(Duration::from_secs(0)))

            // Main menu
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::MainMenu)
                    .with_system(despawn_screen::<OnMainMenuScreen>),
            )
            // Singleplayer menu
            .add_system_set(
                SystemSet::on_enter(AppState::SingleplayerMenu).with_system(setup_main_menu),
            )
            .add_system_set(
                SystemSet::on_update(AppState::SingleplayerMenu)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::SingleplayerMenu)
                    .with_system(despawn_screen::<OnSingleplayerMenuScreen>),
            )
            // Multiplayer menu
            /*.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(update_main_menu))*/
            // Ingame menu
            .add_system_set(SystemSet::on_enter(AppState::InGameMenu).with_system(setup_main_menu))
            .add_system_set(
                SystemSet::on_update(AppState::InGameMenu)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGameMenu)
                    .with_system(despawn_screen::<OnInGameMenuScreen>),
            );
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_icon_style = Style {
        size: Size::new(Val::Px(30.0), Val::Auto),
        // This takes the icons out of the flexbox flow, to be positionned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        position: UiRect {
            left: Val::Px(10.0),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Display the game name
            parent.spawn_bundle(TextBundle::from_section(
                "Rusty Duke",
                TextStyle {
                    font: font.clone(),
                    font_size: 80.0,
                    color: TEXT_COLOR,
                },
            ));
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(MenuButtonAction::SingleplayerMenu)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_section("Singleplayer", button_text_style));
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(MenuButtonAction::MultiplayerMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section("Multiplayer", button_text_style));
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section("Quit", button_text_style));
                });
        });
}

fn setup_singleplayer_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ai_level: Res<AiLevel>,
    game_time: Res<GameTime>,
    game_time_increment: Res<GameTimeIncrement>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_icon_style = Style {
        size: Size::new(Val::Px(30.0), Val::Auto),

        // This takes the icons out of the flexbox flow, to be positionned exactly
        position_type: PositionType::Absolute,

        // The icon will be close to the left border of the button
        position: UiRect {
            left: Val::Px(10.0),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(OnSingleplayerMenuScreen)
        .with_children(|parent| {
            // Set AI level
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Label
                    parent.spawn_bundle(TextBundle::from_section(
                        "AI Level",
                        TextStyle {
                            font: font.clone(),
                            font_size: 80.0,
                            color: TEXT_COLOR,
                        },
                    ));

                    // Value
                    parent.spawn_bundle(TextBundle::from_section(
                        format!("{:?}", *ai_level),
                        TextStyle {
                            font: font.clone(),
                            font_size: 80.0,
                            color: TEXT_COLOR,
                        },
                    ));

                    // Selector
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                margin: UiRect::all(Val::Auto),
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: Color::CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: button_style,
                                    color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                })
                                .insert(MenuButtonAction::IncreaseAI)
                                .with_children(|parent| {
                                    let icon = asset_server.load("icons/up.png");
                                    parent.spawn_bundle(ImageBundle {
                                        style: button_icon_style.clone(),
                                        image: UiImage(icon),
                                        ..default()
                                    });
                                });

                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: button_style.clone(),
                                    color: NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                })
                                .insert(MenuButtonAction::DecreaseAI)
                                .with_children(|parent| {
                                    let icon = asset_server.load("icons/down.png");
                                    parent.spawn_bundle(ImageBundle {
                                        style: button_icon_style.clone(),
                                        image: UiImage(icon),
                                        ..default()
                                    });
                                });
                        });

                    // Set turn timer
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                margin: UiRect::all(Val::Auto),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: Color::CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Label
                            parent.spawn_bundle(TextBundle::from_section(
                                "Game time [Minutes]",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 80.0,
                                    color: TEXT_COLOR,
                                },
                            ));

                            // Value
                            parent.spawn_bundle(TextBundle::from_section(
                                format!("{:?}", *game_time),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 80.0,
                                    color: TEXT_COLOR,
                                },
                            ));

                            // Selector
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        margin: UiRect::all(Val::Auto),
                                        flex_direction: FlexDirection::ColumnReverse,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    color: Color::CRIMSON.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(ButtonBundle {
                                            style: button_style,
                                            color: NORMAL_BUTTON_COLOR.into(),
                                            ..default()
                                        })
                                        .insert(MenuButtonAction::IncreaseGameTime)
                                        .with_children(|parent| {
                                            let icon = asset_server.load("icons/up.png");
                                            parent.spawn_bundle(ImageBundle {
                                                style: button_icon_style.clone(),
                                                image: UiImage(icon),
                                                ..default()
                                            });
                                        });

                                    parent
                                        .spawn_bundle(ButtonBundle {
                                            style: button_style,
                                            color: NORMAL_BUTTON_COLOR.into(),
                                            ..default()
                                        })
                                        .insert(MenuButtonAction::DecreaseGameTime)
                                        .with_children(|parent| {
                                            let icon = asset_server.load("icons/down.png");
                                            parent.spawn_bundle(ImageBundle {
                                                style: button_icon_style.clone(),
                                                image: UiImage(icon),
                                                ..default()
                                            });
                                        });
                                });

                            // Set increment

                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        margin: UiRect::all(Val::Auto),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    color: Color::CRIMSON.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Label
                                    parent.spawn_bundle(TextBundle::from_section(
                                        "Game time increment [Seconds]",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 80.0,
                                            color: TEXT_COLOR,
                                        },
                                    ));

                                    // Value
                                    parent.spawn_bundle(TextBundle::from_section(
                                        format!("{:?}", *game_time_increment),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 80.0,
                                            color: TEXT_COLOR,
                                        },
                                    ));

                                    // Selector
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                margin: UiRect::all(Val::Auto),
                                                flex_direction: FlexDirection::ColumnReverse,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            color: Color::CRIMSON.into(),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn_bundle(ButtonBundle {
                                                    style: button_style,
                                                    color: NORMAL_BUTTON_COLOR.into(),
                                                    ..default()
                                                })
                                                .insert(MenuButtonAction::IncreaseGameTimeIncrement)
                                                .with_children(|parent| {
                                                    parent.spawn_bundle(TextBundle::from_section(
                                                        "Up",
                                                        button_text_style,
                                                    ));
                                                });

                                            parent
                                                .spawn_bundle(ButtonBundle {
                                                    style: button_style,
                                                    color: NORMAL_BUTTON_COLOR.into(),
                                                    ..default()
                                                })
                                                .insert(MenuButtonAction::DecreaseGameTimeIncrement)
                                                .with_children(|parent| {
                                                    parent.spawn_bundle(TextBundle::from_section(
                                                        "Down",
                                                        button_text_style,
                                                    ));
                                                });
                                        });
                                });
                        });
                });
        });
}

fn setup_mp_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    todo!();
}

fn setup_in_game_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_icon_style = Style {
        size: Size::new(Val::Px(30.0), Val::Auto),
        // This takes the icons out of the flexbox flow, to be positionned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        position: UiRect {
            left: Val::Px(10.0),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(OnInGameMenuScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Play)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section("Resume", button_text_style));
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(MenuButtonAction::MainMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Exit to Main Menu",
                        button_text_style,
                    ));
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section("Exit", button_text_style));
                });
        });
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        *color = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON_COLOR.into(),
            Interaction::Hovered => HOVERED_PRESSED_BUTTON_COLOR.into(),
            Interaction::None => NORMAL_BUTTON_COLOR.into(),
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<AppState>>,
    mut ai_level: ResMut<AiLevel>,
    mut game_time: ResMut<GameTime>,
    mut game_time_increment: ResMut<GameTimeIncrement>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::MainMenu => {
                    game_state.set(AppState::Game).unwrap();
                }
                MenuButtonAction::SingleplayerMenu => {
                    game_state.set(AppState::Game).unwrap();
                }
                MenuButtonAction::MultiplayerMenu => {
                    game_state.set(AppState::Game).unwrap();
                }
                MenuButtonAction::InGameMenu => {
                    game_state.set(AppState::Game).unwrap();
                }
                MenuButtonAction::Play => {
                    if *game_state.current() == AppState::InGameMenu {
                        game_state.pop().unwrap();
                    } else {
                        game_state.set(AppState::Game).unwrap();
                    }
                }
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::IncreaseAI => {
                    ai_level.0 = ai_level.0 + 1;
                }
                MenuButtonAction::DecreaseAI => {
                    if ai_level.0 > 0 {
                        ai_level.0 = ai_level.0 - 1;
                    }
                }
                MenuButtonAction::IncreaseGameTime => {
                    game_time.0 = game_time.0 + Duration::from_secs(60);
                }
                MenuButtonAction::DecreaseGameTime => {
                    if game_time.0.as_secs() > 0 {
                        game_time.0 = game_time.0 - Duration::from_secs(60);
                    }
                }
                MenuButtonAction::IncreaseGameTimeIncrement => {
                    game_time_increment.0 = game_time_increment.0 + Duration::from_secs(1);
                }
                MenuButtonAction::DecreaseGameTimeIncrement => {
                    if game_time_increment.0.as_secs() > 0 {
                        game_time_increment.0 = game_time_increment.0 - Duration::from_secs(1);
                    }
                }
            }
        }
    }
}
