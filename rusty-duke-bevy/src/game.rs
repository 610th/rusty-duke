use crate::{AppState, despawn_screen, NORMAL_BUTTON_COLOR};
use crate::menu::*;
use bevy::{prelude::*, ui::Interaction};
use rusty_duke_logic::logic::{get_actions, do_unsafe_action};
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
const AI_TIMEOUT_MS: u32 = 5 * 1000; // 5 seconds

// Board
const BOARD_COLOR: Color = Color::BEIGE;
const COMMANDED_SQUARE_COLOR: Color = Color::TEAL;
const ATTACKED_SQUARE_COLOR: Color = Color::TOMATO;
const STRIKED_SQUARE_COLOR: Color = Color::SALMON;
const MOVE_SQUARE_COLOR: Color = Color::OLIVE;
const DEPLOYABLE_SQUARE_COLOR: Color = Color::ORANGE;
const SQUARE_EFFECT_TEXT_COLOR: Color = Color::RED;
const SQUARE_MARGIN_PX: f32 = 5.0;

// Tiles
pub const BLACK_TILE_COLOR: Color = Color::NONE;
pub const BLACK_TILE_TEXT_COLOR: Color = Color::BISQUE;
const SELECTED_TILE_COLOR: Color = Color::TEAL;
const TILE_SELECTED_TEXT_COLOR: Color = Color::BLUE;
pub const WHITE_TILE_COLOR: Color = Color::BISQUE;
pub const WHITE_TILE_TEXT_COLOR: Color = Color::NONE;
const TILE_TEXT_FONT: &str = "fonts/FiraSans-Bold.ttf";
const TILE_TEXT_FONT_SIZE: f32 = 15.0;
const TILE_MARGIN_PX: f32 = 5.0;

/*const SELECTED_TILE_COLOR: Color = Color::TEAL;
const ATTACKED_TILE_COLOR: Color = Color::TOMATO;
const COMMANDED_TILE_COLOR: Color = Color::NONE;*/

const DOUBLE_CLICK_TIME_NS: u32 = 500 * 1000 * 1000; // 500 ms

// Components
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
#[derive(Component)]
struct DrawNewTile;
#[derive(Component)]
struct TilePlaceholder;
#[derive(Component)]
struct Ai(Agent);

// Resources
struct Game(GameState);
#[derive(PartialEq)]
enum Turn {
    Player,
    Opponent
}
struct TurnTracker(Turn);
struct ClickTime(Instant);
enum TileState {
    Normal,
    Drawn,
    Selected,
    Attacked,
    Striked,
    Commanded,
}

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
                .with_system(clear_board_effects)
                .with_system(update_board_system)
                .with_system(draw_button_system)
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
    app_state: Res<State<AppState>>,
    game_time: Res<GameTime>,
    player_color: Res<PlayerColor>) {

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
    commands.spawn().insert(Player).insert(TColor(player_color.0));

    // Add opponent
    if let AppState::SingleplayerGame = app_state.0 {

        if player_color.0 == TileColor::Black {
            commands.spawn().insert(Opponent).insert(Ai(logic::ai::alpha_beta::new(
                TileColor::White,
                Some(ai_depth),
                Some(AI_TIMEOUT_MS),

            ));
        }
    }
    else {
        todo!();
    }

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
                        .spawn_bundle(TextBundle::from_section("00:00", timer_text_style.clone()))
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
                                            style: square_style.clone(),
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

                    // Draw new tile button
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
                        .spawn_bundle(TextBundle::from_section("", timer_text_style.clone()))
                        .insert(TilePlaceholder);
                });

            // Player time
            parent
                .spawn_bundle(TextBundle::from_section("00:00", timer_text_style.clone()))
                .insert(PlayerTime(Timer::new(game_time.0, false)));
        });
}

// Looks at game state and interactions and updates the board accordingly.
fn update_board_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_update: EventReader<UpdateBoardEvent>,
    state: Res<GameState>,
    mut squares_query: Query<
        (
            Entity,
            &Cord, // Square specific.
            Option<&Selected>,
            Option<&Commanded>,
            Option<&DoubleClicked>,
            Option<&Children>,
            &mut UiColor
        )
    >,
    selected_query: Query<&Cord, With<Selected>>,
    commanded_query: Query<&Cord, With<Selected>>,
    tile_placeholder: Query<(Entity, Option<&Children>), With<TilePlaceholder>>
) {
    let font: Handle<Font> = asset_server.load(TILE_TEXT_FONT);

    let state = &state;
    let board = &state.board;

    // Get actions before updating the board.
    let mut actions: Vec<Action> = Vec::new();

    if !selected_query.is_empty() {
        // Only get tile actions if a tile is selected
        let cord = selected_query.single().0;
        actions = logic::get_tile_actions(state, cord);
    } else if !state.drawn().is_empty() {
        // Or get all actions (only contains deploy actions) if new tile is drawn.
        actions = logic::get_actions(state);
    }

    if !state.drawn().is_empty() {
        let tile = state.drawn().last().unwrap();
        let ui_tile = create_ui_tile(
            &mut commands,
            &asset_server,
            tile,
            TileState::Drawn);
            commands.entity(tile_placeholder.single().0).push_children(&[ui_tile]);
    }
    else {
        // Remove any drawn tile.
        if tile_placeholder.single().1 .is_some() {
            for child in tile_placeholder.single().1.unwrap() {
                commands.entity(*child).despawn_recursive();
            }
        }
    }

    for (square, cord, selected, commanded, double_clicked, children, mut color) in squares_query.iter_mut() {

        // This is not pretty, but works for now. First, remove all tiles and
        // then re-add them. Performance is not really an issue here. I think.
        if children.is_some() {
            for child in children.unwrap() {
                commands.entity(*child).despawn_recursive();
            }
        }

        let cord = cord.0;
        let tile = board[cord.y as usize][cord.x as usize].tile;

        let mut ui_tile: Option<Entity> = None;

        // Add tiles and effects.
        for a in actions.iter() {
            match a {
                Action::PlaceNew(c) if *c == cord => {
                    *color = DEPLOYABLE_SQUARE_COLOR.into();
                }
                Action::Move(ad)
                | Action::Jump(ad)
                | Action::Slide(ad)
                | Action::JumpSlide(ad)
                    if ad.target_pos == cord => {
                        if tile.is_some() {
                            ui_tile = Some(create_ui_tile(
                                                    &mut commands,
                                                    &asset_server,
                                                    tile.as_ref().unwrap(),
                                                    TileState::Attacked));
                            commands.entity(square).push_children(&[ui_tile.unwrap()]);
                        } else {
                            *color = MOVE_SQUARE_COLOR.into();
                        }
                }
                Action::Command(cd) if cd.target_pos == cord => {
                    if tile.is_some() {
                        ui_tile = Some(create_ui_tile(
                                                &mut commands,
                                                &asset_server,
                                                tile.as_ref().unwrap(),
                                                TileState::Commanded));
                        commands.entity(square).push_children(&[ui_tile.unwrap()]);
                    } else {
                        *color = COMMANDED_SQUARE_COLOR.into();
                    }
                }
                Action::Strike(ad) if ad.target_pos == cord => {
                    if tile.is_some() {
                        ui_tile = Some(create_ui_tile(
                                                    &mut commands,
                                                    &asset_server,
                                                    tile.as_ref().unwrap(),
                                                    TileState::Striked));
                        commands.entity(square).push_children(&[ui_tile.unwrap()]);
                    } else {
                        *color = STRIKED_SQUARE_COLOR.into();
                    }
                }
                _ => {}
            }
        }

        if tile.is_some() && ui_tile.is_none() {

            if selected.is_some() {
                    ui_tile = Some(create_ui_tile(
                                                &mut commands,
                                                &asset_server,
                                                tile.as_ref().unwrap(),
                                                TileState::Selected));
                    commands.entity(square).push_children(&[ui_tile.unwrap()]);
            }

            if commanded.is_some() {
                ui_tile = Some(create_ui_tile(
                    &mut commands,
                    &asset_server,
                    tile.as_ref().unwrap(),
                    TileState::Commanded));
                    commands.entity(square).push_children(&[ui_tile.unwrap()]);
            }
        }
    }
}

fn timers_system(
    time: Res<Time>,
    mut player_time: ResMut<PlayerTime>,
    mut opponent_time: ResMut<OpponentTime>,
    turn: Res<TurnTracker>
) {
    if turn.0 == Turn::Player {
        player_time.0.tick(time.delta());
    }
    else {
        opponent_time.0.tick(time.delta());
    }
}

/// Takes input and changes game and UI state. No UI updates are done here.
/// Should probably be splitted into several systems if one wants to be ECS
/// purist.
fn interaction_system(
    mut commands: Commands,
    mut turn: ResMut<TurnTracker>,
    mut game_state: ResMut<Game>,
    mut click_time: ResMut<ClickTime>,
    mut interaction_query: Query<
        (Entity, &Interaction, Option<&Cord>, Option<&GameTile>),
        Changed<Interaction>,
    >,
    mut selected_query: Query<
        (Entity, &Cord), With<Selected>
    >,
    mut commanded_query: Query<
        (Entity, &Cord), With<Commanded>
    >,
    mut player_query: Query<&TColor, With<Player>>,
    mut double_clicked: Query<Entity, With<DoubleClicked>>,
    mut ev_clear: EventWriter<ClearBoardEvent>,
    mut ev_update: EventWriter<UpdateBoardEvent>,
) {

    let mut game = &mut game_state.0;
    let mut selected = None;
    let mut commanded = None;

    if !selected_query.is_empty() {
        // FIXME: This is ugly.
        let c = selected_query.single().1;
        selected = Some((*c).0);
    }
    if !commanded_query.is_empty() {
        let c = commanded_query.single().1;
        commanded = Some((*c).0);
    }

    for (e, i, c, gt) in interaction_query.iter() {

        match i {

            Interaction::Clicked => {

                // Check if click outside of board
                if c.is_none() {
                    ev_clear.send(ClearBoardEvent);
                    break;
                }

                let cord = c.unwrap().0;

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
                            Action::PlaceNew(c) if c == cord => {
                                logic::do_unsafe_action(game, &a);

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
                                if ad.target_pos == cord =>
                                {

                                    logic::do_unsafe_action(game, a);

                                    // Clear components
                                    ev_clear.send(ClearBoardEvent);

                                    // Let opponent do her turn.
                                    turn.0 = Turn::Opponent;

                                }
                                Action::Command(cd) if cd.target_pos == cord => {
                                    // Command is two stage
                                    if commanded.is_some() {
                                        let sc = commanded.unwrap();
                                        if sc == cd.command_tile_pos {
                                            logic::do_unsafe_action(game, a);

                                            // Clear square components
                                            ev_clear.send(ClearBoardEvent);

                                            // Let opponent do her turn.
                                            turn.0 = Turn::Opponent;

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
                        if gt.is_some() {
                            // If not selected, select.
                            commands.entity(e).insert(Selected);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                // FIXME: Change square color.
            }
            _ => {}
        }
    }
}

// Menu button is handled in generic menu handler.
fn draw_button_system(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<DrawNewTile>),
    >,
    mut state: ResMut<GameState>
) {
    if let Interaction::Clicked = interaction_query.single() {
        for action in get_actions(&state) {
            if let Action::NewFromBag = action {
                do_unsafe_action(&mut state, &action);
            }
        }
    }
}

// Clear select, actions etc.
fn clear_board_effects(
    mut commands: Commands,
    _ev_clear: EventReader<ClearBoardEvent>,
    things: Query<
        Entity,
        With<Cord>,
    >,
) {
    for e in things.iter() {
        commands.entity(e).remove::<Selected>();
        commands.entity(e).remove::<Commanded>();
        commands.entity(e).remove::<DoubleClicked>();

        // To be extended
    }
}

fn create_ui_tile(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    tile: &Tile,
    state: TileState
) -> Entity {

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

    let mut tts = tile_text_style.clone();

    if tile.color == TileColor::Black {
        tts.color = BLACK_TILE_TEXT_COLOR;
    }
    else {
        tts.color = WHITE_TILE_TEXT_COLOR;
    }

    let mut tile_color = if tile.color == TileColor::Black {BLACK_TILE_COLOR} else {WHITE_TILE_COLOR};

    match state {
        TileState::Selected => {
            tile_color = SELECTED_TILE_COLOR;
        }
        TileState::Attacked => {
            tile_color = ATTACKED_SQUARE_COLOR;
        }
        TileState::Striked => {
            tile_color = STRIKED_SQUARE_COLOR;
        }
        TileState::Commanded => {
            tile_color = COMMANDED_SQUARE_COLOR;
        }
        _ => {}
    }

    // FIXME: Add tile icon

    let ui_tile = commands.spawn_bundle(NodeBundle{
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
    .id();

    ui_tile
}

fn opponent_turn(
    mut _ev_opponent_turn: EventReader<OpponentTurn>,
    state: Res<State<AppState>>,
    mut game_state: ResMut<Game>,
    opponent: Query<Option<&Ai>, With<Opponent>>
) {

    let mut state = &mut game_state.0;

    match state {
        AppState::SingleplayerGame => {

            let a = alpha_beta::get_action(opponent.single().unwrap(), state);

            if a.is_none() {
                // This means game over. But don't do anything now.
                return Ok(());
            }

            let mut a = a.unwrap();

            logic::do_unsafe_action(state, &a);

            // New from bag action is 2 stage
            match a {
                Action::NewFromBag => {
                    a = alpha_beta::get_action(agent, state).expect("AI is unable to deploy drawn tile.");
                    logic::do_unsafe_action(state, &a);
                }
                _ => {}
            }
        }
        AppState::MultiplayerGame => {
            todo!();
        }
        _ => {
            panic!("Illegal state.")
        }
    }
}