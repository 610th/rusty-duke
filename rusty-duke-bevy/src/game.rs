use crate::{AppState, despawn_screen, NORMAL_BUTTON_COLOR};
use crate::menu::*;
use bevy::{prelude::*, ui::Interaction};
use rusty_duke_logic::{
    ai::alpha_beta::{self, Agent},
    logic::{self, Action, Coordinate, GameState, Tile, TileColor},
};
use std::time::{Duration, Instant};

// Constants
const TEXT_COLOR: Color = Color::BISQUE;
const MENU_BUTTON_COLOR: Color = Color::DARK_GRAY;
const BACKGROUND_COLOR: Color = Color::DARK_GRAY;
const DRAW_BUTTON_COLOR: Color = Color::GRAY;
const DEFAULT_TEXT_FONT: &str = "fonts/FiraSans-Bold.ttf";

// Board
const BOARD_COLOR: Color = Color::BEIGE;
const FOCUSED_SQUARE_COLOR: Color = Color::GRAY;
const SELECTED_SQUARE_COLOR: Color = Color::DARK_GRAY;
const SQUARE_MARGIN_PX: f32 = 5.0;

// Tiles
const BLACK_TILE_COLOR: Color = Color::NONE;
const BLACK_TILE_TEXT_COLOR: Color = Color::BISQUE;
const TILE_EFFECT_TEXT_COLOR: Color = Color::RED;
const TILE_SELECTED_TEXT_COLOR: Color = Color::BLUE;
const WHITE_TILE_COLOR: Color = Color::BISQUE;
const WHITE_TILE_TEXT_COLOR: Color = Color::NONE;
const TILE_TEXT_FONT: &str = "fonts/FiraSans-Bold.ttf";
const TILE_TEXT_FONT_SIZE: f32 = 15.0;
const TILE_MARGIN_PX: f32 = 5.0;

const DOUBLE_CLICK_TIME_NS: u32 = 500 * 1000 * 1000; // 500 ms

// Components
#[derive(Component)]
struct DrawNewTile;
#[derive(Component)]
struct Selected;
#[derive(Component)]
struct Commanded;
#[derive(Component)]
struct DoubleClicked;
#[derive(Component)]
struct DrawnTile;
#[derive(Component)]
struct OnGameScreen;
#[derive(Component)]
struct PlayerTime(Timer);
#[derive(Component)]
struct OpponentTime(Timer);
#[derive(Component)]
struct Cord(Coordinate);
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Opponent;
#[derive(Component)]
struct TColor(TileColor);
#[derive(Component)]
struct GameTile;
#[derive(Component, Debug)]
enum TileAction {
    Deploy,
    Move,
    Jump,
    JumpSlide,
    Slide,
    Command,
    Strike,
}
#[derive(Component)]
struct Effect(logic::Effect);
#[derive(Component)]
struct TileType(logic::TileType);

// Resources
struct Game(GameState);
#[derive(PartialEq)]
enum Turn {
    Player,
    Opponent
}
struct TurnTracker(Turn);
struct ClickTime(Instant);

// Events
struct ClearBoardEvent;
struct UpdateBoardEvent;

/*{
    state: GameState,
    player_color: Option<TileColor>,
    ai_agent: Agent,
    // For AI vs AI
    ai_agent2: Option<Agent>,
}*/
/*
impl FromWorld for MyFancyResource {
    fn from_world(world: &mut World) -> Self {
        MyFancyResource { /* stuff */ }
    }
}*/

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ClearBoardEvent>()
        .add_event::<UpdateBoardEvent>()
        .insert_resource(Game(
            GameState::new(
                //
            )
        ))
        .insert_resource(TurnTracker)
        .insert_resource(ClickTime(Instant::now()))
        .add_system_set(
            SystemSet::on_enter(AppState::SingleplayerGame).with_system(setup_game)
        )
        .add_system_set(
            SystemSet::on_update(AppState::SingleplayerMenu)
                .with_system(interaction_system)
                .with_system(clear_board)
                .with_system(update_board)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::SingleplayerGame)
                .with_system(despawn_screen::<OnGameScreen>),
        );
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_time: Res<GameTime>,
    state: Res<AppState>) {

    let font = asset_server.load(DEFAULT_TEXT_FONT);

    // Common style for all buttons on the screen
    let button_style = Style {
        min_size: Size::new(Val::Px(32.0), Val::Px(32.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_icon_style = Style {
        min_size: Size::new(Val::Px(32.0), Val::Auto),
        position_type: PositionType::Relative,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    let timer_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    let square_style = Style {
        min_size: Size::new(Val::Px(32.0), Val::Auto),
        margin: UiRect::all(Val::Px(5.0)),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..default()
    };

    // Add game logic as entity, i.e. "the board game" is an entity.
    // If the game logic was implemented in ECS, things would be different.
    // commands.spawn().insert(GameLogic("Elaina Proctor".to_string()));

    // Add player
    //commands.spawn().insert(Player).insert(Name("Elaina Proctor".to_string()));

    // Create game screen
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: BOARD_COLOR.into(),
            ..default()
        })
        .insert(OnGameScreen)
        .with_children(|parent| {

            // Row above board
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {

                    // Opponent time
                    parent
                        .spawn_bundle(TextBundle::from_section("00:00", timer_text_style))
                        .insert(OpponentTime(Timer::new(game_time.0, false)));

                    // Menu hamburger button
                    parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    })
                    .insert(MenuButtonAction::InGameMenu)
                    .with_children(|parent| {
                        let icon = asset_server.load("icons/hamburger.png");
                        parent.spawn_bundle(ImageBundle {
                            style: button_icon_style.clone(),
                            image: UiImage(icon),
                            ..default()
                        });
                    });
                });

            // Board
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for y in 0..logic::HEIGHT {
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                color: BOARD_COLOR.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                for x in 0..logic::WIDTH {
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: square_style,
                                            ..default()
                                        })
                                        .insert(Interaction::None)
                                        .insert(Cord(Coordinate::new(x.into(), y.into())));
                                }
                            });
                    }
                });

            // Row under board (draw new tile)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {

                    // Menu hamburger button
                    parent
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        color: DRAW_BUTTON_COLOR.into(),
                        ..default()
                    })
                    .insert(DrawNewTile)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            "Draw",
                            button_text_style,
                        ));
                    });

                    // Drawn Tile
                    parent
                        .spawn_bundle(TextBundle::from_section("", timer_text_style))
                        .insert(DrawnTile);
                });

            // Player time
            parent
                .spawn_bundle(TextBundle::from_section("00:00", timer_text_style.clone()))
                .insert(PlayerTime(Timer::new(game_time.0, false)));
        });
}


// This is an event reader and not a system. Mainly triggered by the interaction system.
fn update_board(
    mut commands: Commands,
    mut ev_update: EventReader<UpdateBoardEvent>,
    asset_server: Res<AssetServer>,
    state: Res<GameState>,
    mut squares_query: Query<
        (Entity, &Cord, Option<&Selected>, Option<&Commanded>, Option<&DoubleClicked>, &Children)
    >,
    selected_query: Query<&Parent, With<Selected>>
) {
    let state = &state;
    let board = &state.board;
    let font = asset_server.load(TILE_TEXT_FONT);

    let tile_style = Style {
        margin: UiRect::all(Val::Px(5.0)),
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::Center,
        ..default()
    };

    let tile_text_style = TextStyle {
        font: font.clone(),
        font_size: TILE_TEXT_FONT_SIZE,
        ..default()
    };

    let selected_text_style = TextStyle {
        font: font.clone(),
        font_size: TILE_TEXT_FONT_SIZE,
        color: TILE_SELECTED_TEXT_COLOR,
    };

    let effect_text_style = TextStyle {
        font: font.clone(),
        font_size: TILE_TEXT_FONT_SIZE,
        color: TILE_EFFECT_TEXT_COLOR,
    };

    // Get actions before updating the board.
    let mut actions: Vec<Action> = Vec::new();

    if !selected_query.is_empty() {
        // Only get tile actions if a tile is selected
        let parent = selected_query.single();

        let cord = selected_query.single().0;
        actions = logic::get_tile_actions(state, cord);
    } else if !state.drawn().is_empty() {
        // Get all actions (only contains deploy actions) if new tile is drawn.
        actions = logic::get_actions(state);
    }

    for (square, cord, selected, commanded, double_clicked, children) in squares_query.iter() {

        // All UI tiles are entities and children to UI squares, because we want
        // to utilize that children transforms are relative to parents.

        // This is not pretty, but works for now. First, remove all tiles and
        // then reprint them. More elegant, but maybe not more efficient, would
        // be to not remove the entity every iteration. However, performance is
        // not an issue here.
        for child in children {
            commands.entity(*child).despawn_recursive();
        }

        let cord = cord.0;
        let tile = board[cord.y as usize][cord.x as usize].tile;

        let mut ui_tile: Option<Entity> = None;

        // Print tile
        if tile.is_some() {

            let tile = tile.as_ref().unwrap();
            let mut tts = tile_text_style.clone();

            if tile.color == TileColor::Black {
                tts.color = BLACK_TILE_TEXT_COLOR;
            }
            else {
                tts.color = WHITE_TILE_TEXT_COLOR;
            }

            let tile_color = if tile.color == TileColor::Black {BLACK_TILE_COLOR} else {WHITE_TILE_COLOR};

            ui_tile = Some(commands.spawn_bundle(NodeBundle{
                style: tile_style,
                color: tile_color.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    tile.kind.to_string(),
                    tts.clone(),
                ));
            })
            .insert(GameTile)
            .id());

            commands.entity(square).push_children(&[ui_tile.unwrap()]);
        }

        // Add effects
        for a in actions.iter() {
            match a {
                Action::PlaceNew(c) if *c == cord => {
                    commands.entity(square).insert(TileAction::Deploy);
                }
                Action::Move(ad) if ad.target_pos == cord => {
                    if ui_tile.is_some() {
                        commands.entity(ui_tile.unwrap()).insert(TileAction::Move);
                    } else {
                        commands.entity(square).insert(TileAction::Move);
                    }
                }
                Action::Jump(ad) if ad.target_pos == cord => {
                    if ui_tile.is_some() {
                        commands.entity(ui_tile.unwrap()).insert(TileAction::Jump);
                    } else {
                        commands.entity(square).insert(TileAction::Jump);
                    }
                }
                Action::Slide(ad) if ad.target_pos == cord => {
                    if ui_tile.is_some() {
                        commands.entity(ui_tile.unwrap()).insert(TileAction::Slide);
                    } else {
                        commands.entity(square).insert(TileAction::Slide);
                    }
                }
                Action::JumpSlide(ad) if ad.target_pos == cord => {
                    if ui_tile.is_some() {
                        commands.entity(ui_tile.unwrap()).insert(TileAction::JumpSlide);
                    } else {
                        commands.entity(square).insert(TileAction::JumpSlide);
                    }
                }
                Action::Command(cd) if cd.target_pos == cord => {
                    if ui_tile.is_some() {
                        commands.entity(ui_tile.unwrap()).insert(TileAction::Command);
                    } else {
                        commands.entity(square).insert(TileAction::Command);
                    }
                }
                Action::Strike(ad) if ad.target_pos == cord => {
                    if ui_tile.is_some() {
                        commands.entity(ui_tile.unwrap()).insert(TileAction::Strike);
                    } else {
                        commands.entity(square).insert(TileAction::Strike);
                    }
                }
                _ => {}
            }
        }

        // Update colors and text
    }

    // Add effects


    // Add drawn tile


}

fn timers_system(
    time: Res<Time>,
    player_time: ResMut<PlayerTime>,
    opponent_time: ResMut<OpponentTime>,
    turn: Res<TurnTracker>
) {
    if turn.0 == Turn::Player {
        player_time.0.tick(time.delta());
    }
    else {
        opponent_time.0.tick(time.delta());
    }
}

/// This is where the magic happens. Should probably be splitted into several
/// systems if one wants to be ECS purist.
fn interaction_system(
    mut commands: Commands,
    mut turn: ResMut<TurnTracker>,
    mut game_state: ResMut<Game>,
    mut click_time: ResMut<ClickTime>,
    mut interaction_query: Query<
        (Entity, &Interaction, Option<&Cord>, Option<&TColor>),
        (Changed<Interaction>),
    >,
    mut selected_query: Query<
        (Entity, &Cord, &TColor), With<Selected>
    >,
    mut commanded_query: Query<
        (Entity, &Cord, &TColor), With<Commanded>
    >,
    mut player_query: Query<&TColor, With<Player>>,
    mut double_clicked: Query<Entity, With<DoubleClicked>>,
    mut ev_clear: EventWriter<ClearBoardEvent>,
    mut ev_update: EventWriter<UpdateBoardEvent>,
) {

    let game = &game_state.0;
    let mut selected = None;
    let mut commanded = None;
    let player_color = player_query.single().0;

    if !selected_query.is_empty() {
        // FIXME: This is ugly.
        let c = selected_query.single().1;
        selected = Some((*c).0);
    }
    if !commanded_query.is_empty() {
        let c = commanded_query.single().1;
        commanded = Some((*c).0);
    }

    for (e, i, c, tc) in interaction_query.iter() {

        let cord = c.0;

        match i {

            Interaction::Clicked => {

                // Check if click outside of board
                if c.is_none() {
                    ev_clear.send(ClearBoardEvent);
                    break;
                }

                // Clear any double clicks
                if !double_clicked.is_empty() {
                    commands.entity(double_clicked.single()).remove::<DoubleClicked>();
                }

                let now = Instant::now();

                // Double clicked?
                if (now - click_time.0) < Duration::new(0, DOUBLE_CLICK_TIME_NS) {
                    // Clear square components
                    commands.entity(selected_query.single().0)
                    .remove::<Selected>();
                    commands.entity(selected_query.single().0)
                        .remove::<Commanded>();
                    commands.entity(e).insert(DoubleClicked);
                    return;
                }

                click_time.0 = now;

                // If there is a drawn tile, it has to be deployed.
                if !game.drawn().is_empty() {

                    let actions = logic::get_actions(game);

                    for a in actions {
                        match a {
                            Action::NewFromBag => {
                                logic::do_unsafe_action(&mut game, &a);
                                // Opponent turn
                                turn.0 = Turn::Opponent;
                                // Opponent turn event?
                            }
                            _ => {}
                        }
                    }
                }
                else {

                    // If a tile is selected and of player color, try to perform action.
                    if selected.is_some() && turn.0 == Turn::Player {

                        // If selected, check if current click means an action, if
                        // so, perform the action.

                        let actions = logic::get_tile_actions(game, selected.unwrap());

                        for a in actions.iter() {
                            match a {
                                Action::Move(ad)
                                | Action::Jump(ad)
                                | Action::Slide(ad)
                                | Action::JumpSlide(ad)
                                | Action::Strike(ad)
                                    if ad.target_pos == cord && commanded.is_none() =>
                                {
                                    logic::do_unsafe_action(&mut game_state.0, a);

                                    // Clear components
                                    commands.entity(selected_query.single().0)
                                                .remove::<Selected>();

                                    // Let opponent do her turn.

                                }
                                Action::Command(cd) if cd.target_pos == cord => {
                                    // Command is two stage
                                    if commanded.is_some() {
                                        let sc = commanded.unwrap();
                                        if sc == cd.command_tile_pos {
                                            logic::do_unsafe_action(&mut game_state.0, a);

                                            // Clear square components
                                            commands.entity(selected_query.single().0)
                                                .remove::<Selected>();
                                            commands.entity(commanded_query.single().0)
                                                .remove::<Commanded>();

                                            // Let opponent do her turn.

                                        }
                                    }
                                }
                                _ => {
                                    // No match, clear selected.
                                    ev_clear.send(ClearBoardEvent);
                                }
                            }
                        }
                    }
                    else {
                        // If not selected and tile on square, select.
                        if tc.is_some() {
                            // If not selected, select.
                            commands.entity(e).insert(Selected);
                        }
                    }
                }

                // Always update board after click
                ev_update.send(UpdateBoardEvent);
            }
            Interaction::Hovered => {
                // FIXME: Change square color.
            }
            _ => {}
        }
    }
}

// Clear select, actions etc.
fn clear_board(
    mut commands: Commands,
    mut ev_clear: EventReader<ClearBoardEvent>,
    mut ev_update: EventWriter<UpdateBoardEvent>,
    mut squares: Query<
        Entity,
        With<Cord>,
    >,
) {
    for e in squares.iter() {
        commands.entity(e).remove::<Selected>();
        commands.entity(e).remove::<Commanded>();
        commands.entity(e).remove::<DoubleClicked>();

        // To be extended
    }

    ev_update.send(UpdateBoardEvent);
}
