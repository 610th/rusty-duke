/// Terminal interface for Rusty Duke game. Only single player vs AI.
///
/// Very basic for manual testing. If you want something more fancy, feel free
/// to contribute.
pub use crossterm::{
    cursor::{self, MoveTo, MoveToNextLine, RestorePosition, SavePosition},
    event::{self, read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{
        self, Attribute, Color, Colors, Print, ResetColor, SetBackgroundColor, SetForegroundColor,
        Stylize,
    },
    terminal::{self, ClearType, SetTitle},
    Command, Result,
};
use rusty_duke::{
    ai::alpha_beta::{self, Agent},
    logic::{self, Action, Coordinate, GameState, Tile, TileColor},
};
use std::{
    io::{self, stdin, Write},
    time::Duration,
};

use flexi_logger::{self, FileSpec, Logger};

/// (X,Y)
const SQUARE_SIZE: (u16, u16) = (16, 6);
const TILE_SIZE: (u16, u16) = (15, 5);
const TERM_WIDTH: u16 = SQUARE_SIZE.0 * logic::WIDTH as u16;
const TERM_HEIGHT: u16 = SQUARE_SIZE.1 * (logic::HEIGHT) as u16 + TILE_SIZE.1 + 5;

const BLACK_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::DarkGrey),
};
const FOCUSED_BLACK_COLORS: Colors = Colors {
    foreground: Some(Color::DarkGrey),
    background: Some(Color::Blue),
};
const SELECTED_BLACK_COLORS: Colors = Colors {
    foreground: Some(Color::DarkYellow),
    background: Some(Color::Green),
};
const ATTACKED_BLACK_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::DarkRed),
};
const STRIKED_BLACK_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::DarkRed),
};
const COMMANDED_BLACK_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::DarkGreen),
};

const WHITE_COLORS: Colors = Colors {
    foreground: Some(Color::DarkGrey),
    background: Some(Color::White),
};
const FOCUSED_WHITE_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::DarkCyan),
};
const SELECTED_WHITE_COLORS: Colors = Colors {
    foreground: Some(Color::DarkYellow),
    background: Some(Color::Green),
};
const ATTACKED_WHITE_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::DarkRed),
};
const STRIKED_WHITE_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::DarkRed),
};
const COMMANDED_WHITE_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::DarkGreen),
};

const BOARD_COLORS: Colors = Colors {
    foreground: Some(Color::Magenta),
    background: Some(Color::DarkYellow),
};

const SELECTED_SQUARE: Colors = Colors {
    foreground: Some(Color::DarkYellow),
    background: Some(Color::Magenta),
};

/*const MOVE_SQUARE_COLORS: Colors = Colors {
    foreground: Some(Color::White),
    background: Some(Color::Green),
};
const JUMP_SQUARE_COLORS: Colors = Colors {
    foreground: Some(Color::Black),
    background: Some(Color::Cyan),
};*/

enum TileState {
    Normal,
    Drawn,
    Focused,
    Selected,
    Attacked,
    Striked,
    Commanded,
}

struct PlayState {
    state: GameState,
    player_color: Option<TileColor>,
    agent: Agent,
    agent2: Option<Agent>,
    focus: Coordinate,
    selected: Option<Coordinate>,
    selected_command: Option<Coordinate>,
}

enum State {
    MainMenu,
    AiMenu(Option<TileColor>),
    Play(PlayState),
    Exit,
}

fn print_tile<W>(w: &mut W, cursor: (u16, u16), state: TileState, tile: &Tile) -> Result<()>
where
    W: Write,
{
    let fg_color: Color;
    let bg_color: Color;

    if tile.color == TileColor::Black {
        match state {
            TileState::Normal => {
                fg_color = BLACK_COLORS.foreground.unwrap();
                bg_color = BLACK_COLORS.background.unwrap();
            }
            TileState::Drawn => {
                fg_color = FOCUSED_BLACK_COLORS.foreground.unwrap();
                bg_color = FOCUSED_BLACK_COLORS.background.unwrap();
            }
            TileState::Focused => {
                fg_color = FOCUSED_BLACK_COLORS.foreground.unwrap();
                bg_color = FOCUSED_BLACK_COLORS.background.unwrap();
            }
            TileState::Selected => {
                fg_color = SELECTED_BLACK_COLORS.foreground.unwrap();
                bg_color = SELECTED_BLACK_COLORS.background.unwrap();
            }
            TileState::Attacked => {
                fg_color = ATTACKED_BLACK_COLORS.foreground.unwrap();
                bg_color = ATTACKED_BLACK_COLORS.background.unwrap();
            }
            TileState::Striked => {
                fg_color = STRIKED_BLACK_COLORS.foreground.unwrap();
                bg_color = STRIKED_BLACK_COLORS.background.unwrap();
            }
            TileState::Commanded => {
                fg_color = COMMANDED_BLACK_COLORS.foreground.unwrap();
                bg_color = COMMANDED_BLACK_COLORS.background.unwrap();
            }
        }
    } else {
        match state {
            TileState::Normal => {
                fg_color = WHITE_COLORS.foreground.unwrap();
                bg_color = WHITE_COLORS.background.unwrap();
            }
            TileState::Drawn => {
                fg_color = FOCUSED_WHITE_COLORS.foreground.unwrap();
                bg_color = FOCUSED_WHITE_COLORS.background.unwrap();
            }
            TileState::Focused => {
                fg_color = FOCUSED_WHITE_COLORS.foreground.unwrap();
                bg_color = FOCUSED_WHITE_COLORS.background.unwrap();
            }
            TileState::Selected => {
                fg_color = SELECTED_WHITE_COLORS.foreground.unwrap();
                bg_color = SELECTED_WHITE_COLORS.background.unwrap();
            }
            TileState::Attacked => {
                fg_color = ATTACKED_WHITE_COLORS.foreground.unwrap();
                bg_color = ATTACKED_WHITE_COLORS.background.unwrap();
            }
            TileState::Striked => {
                fg_color = STRIKED_WHITE_COLORS.foreground.unwrap();
                bg_color = STRIKED_WHITE_COLORS.background.unwrap();
            }
            TileState::Commanded => {
                fg_color = COMMANDED_WHITE_COLORS.foreground.unwrap();
                bg_color = COMMANDED_WHITE_COLORS.background.unwrap();
            }
        }
    }

    print_square(
        w,
        cursor,
        (cursor.0 + TILE_SIZE.0, cursor.1 + TILE_SIZE.1),
        fg_color,
        bg_color,
        Some(tile.kind.to_string()),
    )?;

    Ok(())
}

fn print_square<W>(
    w: &mut W,
    start: (u16, u16),
    stop: (u16, u16),
    fg: Color,
    bg: Color,
    text: Option<String>,
) -> Result<()>
where
    W: Write,
{
    queue!(w, SavePosition, MoveTo(start.0, start.1))?;

    for y in 0..(stop.1 - start.1) {
        if text.is_some() && y == SQUARE_SIZE.1 / 2 {
            let s: String = format!(
                "{: ^width$}",
                text.as_ref().unwrap(),
                width = (stop.0 - start.0) as usize
            );
            queue!(
                w,
                style::PrintStyledContent(s.with(fg).on(bg)),
                MoveTo(start.0, start.1 + y + 1)
            )?;
        } else {
            queue!(
                w,
                style::Print(" ".repeat((stop.0 - start.0) as usize).with(fg).on(bg)),
                MoveTo(start.0, start.1 + y + 1)
            )?;
        }
    }

    queue!(w, RestorePosition)?;
    Ok(())
}

fn print_board<W>(w: &mut W, state: &PlayState) -> Result<()>
where
    W: Write,
{
    let player_color;
    let game_state = &state.state;
    let board = &game_state.board;
    let focus = state.focus;
    let selected = state.selected;

    if state.player_color.is_some() {
        player_color = state.player_color.unwrap();
    } else {
        // Not pretyy, but works.
        player_color = state.agent.color;
    }

    let fg = BOARD_COLORS.foreground.unwrap();
    let bg = BOARD_COLORS.background.unwrap();

    queue!(
        w,
        style::SetColors(BOARD_COLORS),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    // Header
    let s: String = format!(
        "{: ^width$}",
        "Rusty Duke",
        width = (TERM_WIDTH + 1) as usize
    );
    queue!(w, style::Print(s), cursor::MoveToNextLine(1))?;

    // Print board
    for y in 0..(SQUARE_SIZE.1 * logic::HEIGHT as u16) {
        if y % SQUARE_SIZE.1 == 0 {
            queue!(
                w,
                style::Print("-".repeat(1 + (SQUARE_SIZE.0 * logic::WIDTH as u16) as usize)),
                cursor::MoveToNextLine(1)
            )?;
        } else {
            let s: String = format!(
                "|{: ^width$}",
                " ".to_string(),
                width = (SQUARE_SIZE.0 - 1) as usize
            );
            queue!(
                w,
                style::Print(s.repeat(logic::WIDTH as usize)),
                style::Print("|"),
                cursor::MoveToNextLine(1)
            )?;
        }
    }

    // Last line
    queue!(
        w,
        style::Print("-".repeat(1 + (SQUARE_SIZE.0 * logic::WIDTH as u16) as usize))
    )?;

    // Find top left terminal cordinate for board square cordinate.
    let square_cursor = |cord: Coordinate, player_color: TileColor| -> (u16, u16) {
        let x: u16;
        let y: u16;

        if player_color == TileColor::Black {
            x = (1 + (cord.x as u16 * SQUARE_SIZE.0)) as u16;
            y = (2 + ((logic::HEIGHT - cord.y - 1) as u16 * SQUARE_SIZE.1)) as u16;
        } else {
            x = (1 + ((logic::WIDTH - cord.x - 1) as u16 * SQUARE_SIZE.0)) as u16;
            y = (2 + ((logic::HEIGHT - cord.y - 1) as u16 * SQUARE_SIZE.1)) as u16;
        }

        (x, y)
    };

    // Get actions
    let actions: Vec<Action>;

    if selected.is_some() {
        let cord = selected.unwrap();
        actions = logic::get_tile_actions(game_state, cord);
    } else if !game_state.drawn().is_empty() {
        actions = logic::get_actions(game_state);
    } else {
        actions = logic::get_tile_actions(game_state, focus);
    }

    // Print them tiles
    for y in 0..logic::HEIGHT {
        for x in 0..logic::WIDTH {
            let mut tile_state = TileState::Normal;
            let tile = &board[y as usize][x as usize].tile;
            let cord = Coordinate { x: x, y: y };
            let cursor = square_cursor(cord, player_color);
            let mut square_text: Option<String> = None;

            for a in actions.iter() {
                match a {
                    Action::PlaceNew(c) if *c == cord => {
                        square_text = Some("Deploy".to_string());
                    }
                    Action::Move(ad) if ad.target_pos == cord => {
                        if tile.is_some() {
                            tile_state = TileState::Attacked;
                        } else {
                            square_text = Some("Move".to_string());
                        }
                    }
                    Action::Jump(ad) if ad.target_pos == cord => {
                        if tile.is_some() {
                            tile_state = TileState::Attacked;
                        } else {
                            square_text = Some("Jump".to_string());
                        }
                    }
                    Action::Slide(ad) if ad.target_pos == cord => {
                        if tile.is_some() {
                            tile_state = TileState::Attacked;
                        } else {
                            square_text = Some("Slide".to_string());
                        }
                    }
                    Action::JumpSlide(ad) if ad.target_pos == cord => {
                        if tile.is_some() {
                            tile_state = TileState::Attacked;
                        } else {
                            square_text = Some("Jumpslide".to_string());
                        }
                    }
                    Action::Command(cd) => {
                        if tile.is_some() {
                            if cd.target_pos == cord {
                                tile_state = TileState::Attacked;
                            } else if cd.command_tile_pos == cord {
                                tile_state = TileState::Commanded;
                            }
                        } else if cd.target_pos == cord {
                            square_text = Some("Command Move".to_string());
                        }
                    }
                    Action::Strike(ad) if ad.target_pos == cord => {
                        if tile.is_some() {
                            tile_state = TileState::Striked;
                        } else {
                            square_text = Some("Strike".to_string());
                        }
                    }
                    _ => {}
                }
            }

            // Selected looks like focused
            if selected.is_some() && selected.unwrap() == cord {
                tile_state = TileState::Selected;
            }

            if tile.is_some() {
                // Focus override any state
                if focus == cord {
                    tile_state = TileState::Focused;
                }
                print_tile(
                    w,
                    square_cursor(cord, player_color),
                    tile_state,
                    &tile.unwrap(),
                )?;
            } else {
                let mut square_fg = fg;
                let mut square_bg = bg;

                if focus == cord {
                    square_fg = SELECTED_SQUARE.foreground.unwrap();
                    square_bg = SELECTED_SQUARE.background.unwrap();
                }

                print_square(
                    w,
                    cursor,
                    (cursor.0 + TILE_SIZE.0, cursor.1 + TILE_SIZE.1),
                    square_fg,
                    square_bg,
                    square_text,
                )?;
            }
        }
    }

    // Print drawn tile if any.
    if !game_state.drawn().is_empty() {
        print_tile(
            w,
            (0, TERM_HEIGHT - SQUARE_SIZE.1),
            TileState::Drawn,
            game_state.drawn().last().unwrap(),
        )?;
    } else {
        // Clear drawn tile
        print_square(
            w,
            (0, TERM_HEIGHT - SQUARE_SIZE.1 - 2),
            (0 + TILE_SIZE.0, TERM_HEIGHT - SQUARE_SIZE.1 + TILE_SIZE.1),
            fg,
            bg,
            None,
        )?;
    }

    // Print ply info
    execute!(
        w,
        ResetColor,
        MoveTo(0, TERM_HEIGHT - 2),
        Print(" ".repeat(TERM_WIDTH as usize)),
        MoveTo(0, TERM_HEIGHT - 1),
        Print(format!("Player to go: {:?}", state.state.ply)),
    )?;

    Ok(())
}

/// See if selected tile can command focused tile
fn can_command_tile(state: &mut PlayState) -> bool {
    // Commander has to be selected
    if state.selected.is_none() {
        return false;
    }

    // Commanded cannot already be selected.
    if state.selected_command.is_some() {
        return false;
    }

    let selected = state.selected.unwrap();
    let actions = logic::get_tile_actions(&state.state, selected);
    for a in actions.iter() {
        match a {
            Action::Command(cd) if cd.command_tile_pos == state.focus => {
                return true;
            }
            _ => {}
        }
    }

    false
}

/// If possible, will perform an action with selected tile on to focused square.
/// Returns true if action is performed.
fn try_tile_action(state: &mut PlayState) -> bool {
    if state.selected.is_none() {
        return false;
    }

    let selected = state.selected.unwrap();

    if state.state.board[selected.y as usize][selected.x as usize]
        .tile
        .is_none()
    {
        return false;
    }

    let actions = logic::get_tile_actions(&state.state, selected);

    for a in actions.iter() {
        match a {
            Action::Move(ad)
            | Action::Jump(ad)
            | Action::Slide(ad)
            | Action::JumpSlide(ad)
            | Action::Strike(ad)
                if ad.target_pos == state.focus && state.selected_command.is_none() =>
            {
                logic::do_unsafe_action(&mut state.state, a);
                state.selected = None;
                return true;
            }
            Action::Command(cd) if cd.target_pos == state.focus => {
                // Command is two stage
                if state.selected_command.is_some() {
                    let selected_command = state.selected_command.unwrap();
                    if selected_command == cd.command_tile_pos {
                        logic::do_unsafe_action(&mut state.state, a);
                        state.selected = None;
                        state.selected_command = None;
                        return true;
                    }
                }
            }
            _ => {}
        }
    }

    false
}

fn draw_new_tile(state: &mut PlayState) -> bool {
    let actions = logic::get_actions(&state.state);

    for a in actions {
        match a {
            Action::NewFromBag => {
                logic::do_unsafe_action(&mut state.state, &a);
                return true;
            }
            _ => {}
        }
    }

    false
}

fn place_new_tile(state: &mut PlayState) -> bool {
    let actions = logic::get_actions(&state.state);

    for a in actions {
        match a {
            Action::PlaceNew(c) if c == state.focus => {
                logic::do_unsafe_action(&mut state.state, &a);
                return true;
            }
            _ => {}
        }
    }

    false
}

fn ai_turn(agent: &Agent, state: &mut GameState) -> Result<()> {
    let a = alpha_beta::get_action(agent, state);

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

    Ok(())
}

fn player_vs_ai<W>(w: &mut W, state: &mut State) -> Result<()>
where
    W: Write,
{
    let play_state: &mut PlayState;

    match state {
        State::Play(s) => {
            play_state = s;
        }
        _ => {
            panic!("Illegal state.");
        }
    }

    let player_color = play_state.player_color.expect("No player color.");

    // Black player goes first.
    if player_color == TileColor::White {
        ai_turn(&play_state.agent, &mut play_state.state)?;
    }

    loop {
        print_board(w, play_state)?;
        w.flush()?;

        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') =>
            // Quit
            {
                *state = State::MainMenu;
                break;
            }
            Event::Key(event) if event.code == KeyCode::Esc =>
            // Cancel
            {
                if play_state.selected_command.is_some() {
                    play_state.selected_command = None;
                    continue;
                }

                play_state.selected = None;
            }
            Event::Key(event)
                if event.code == KeyCode::Enter || event.code == KeyCode::Char(' ') =>
            // Multi function key. Place new tile or select tile or perform action.
            {
                if !play_state.state.drawn().is_empty() {
                    if place_new_tile(play_state) {
                        // If success, let AI player do her turn.
                        ai_turn(&play_state.agent, &mut play_state.state)?;
                    }
                } else {
                    let square = &play_state.state.board[play_state.focus.y as usize]
                        [play_state.focus.x as usize];

                    // If selected, do action stuff
                    if play_state.selected.is_some() {
                        // Try do action. This also works for commanded tile.
                        if try_tile_action(play_state) {
                            // If success, let AI player do her turn.
                            ai_turn(&play_state.agent, &mut play_state.state)?;
                        }
                    } else if square.tile.is_some() {
                        // If not selected, select.
                        let tile = &square.tile.unwrap();
                        if tile.color == player_color {
                            play_state.selected = Some(play_state.focus);
                        }
                    }
                }
            }
            // Command tile
            Event::Key(event) if event.code == KeyCode::Char('c') =>
            {
                if play_state.selected.is_some() {
                    // Maybe just select second tile to command?
                    if can_command_tile(play_state) {
                        play_state.selected_command = Some(play_state.focus);
                    }
                }
            }
            // Grab new tile from bag.
            Event::Key(event) if event.code == KeyCode::Char('n') =>
            {
                if draw_new_tile(play_state) {
                    play_state.selected_command = None;
                    play_state.selected = None;
                }
            }
            Event::Key(event) if event.code == KeyCode::Left => {
                if player_color == TileColor::Black {
                    play_state.focus.x = (play_state.focus.x + logic::WIDTH - 1) % logic::WIDTH;
                } else {
                    play_state.focus.x = (play_state.focus.x + logic::WIDTH + 1) % logic::WIDTH;
                }
            }
            Event::Key(event) if event.code == KeyCode::Right => {
                if player_color == TileColor::Black {
                    play_state.focus.x = (play_state.focus.x + logic::WIDTH + 1) % logic::WIDTH;
                } else {
                    play_state.focus.x = (play_state.focus.x + logic::WIDTH - 1) % logic::WIDTH;
                }
            }
            Event::Key(event) if event.code == KeyCode::Up => {
                play_state.focus.y = (play_state.focus.y + logic::HEIGHT + 1) % logic::HEIGHT;
            }
            Event::Key(event) if event.code == KeyCode::Down => {
                play_state.focus.y = (play_state.focus.y + logic::HEIGHT - 1) % logic::HEIGHT;
            }
            _ => {}
        }
    }

    Ok(())
}

fn ai_vs_ai<W>(w: &mut W, state: &mut State) -> Result<()>
where
    W: Write,
{
    let play_state: &mut PlayState;
    let black_ai: &Agent;
    let white_ai: &Agent;

    match state {
        State::Play(s) => {
            play_state = s;
        }
        _ => {
            panic!("Illegal state.");
        }
    }

    if play_state.agent.color == TileColor::Black {
        black_ai = &play_state.agent;
        white_ai = play_state.agent2.as_ref().unwrap();
    } else {
        black_ai = play_state.agent2.as_ref().unwrap();
        white_ai = &play_state.agent;
    }

    let mut current_ai = black_ai;

    loop {
        print_board(w, play_state)?;
        w.flush()?;

        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') =>
            {
                *state = State::MainMenu;
                break;
            }
            Event::Key(event)
                if event.code == KeyCode::Enter || event.code == KeyCode::Char(' ') =>
            {
                ai_turn(current_ai, &mut play_state.state)?;
                if current_ai.color == TileColor::Black {
                    current_ai = white_ai;
                } else {
                    current_ai = black_ai;
                }
            }
            Event::Key(event) if event.code == KeyCode::Left => {
                play_state.focus.x = (play_state.focus.x + logic::WIDTH - 1) % logic::WIDTH;
            }
            Event::Key(event) if event.code == KeyCode::Right => {
                play_state.focus.x = (play_state.focus.x + logic::WIDTH + 1) % logic::WIDTH;
            }
            Event::Key(event) if event.code == KeyCode::Up => {
                play_state.focus.y = (play_state.focus.y + logic::HEIGHT + 1) % logic::HEIGHT;
            }
            Event::Key(event) if event.code == KeyCode::Down => {
                play_state.focus.y = (play_state.focus.y + logic::HEIGHT - 1) % logic::HEIGHT;
            }
            _ => {}
        }
    }

    Ok(())
}

fn play<W>(w: &mut W, state: &mut State) -> Result<()>
where
    W: Write,
{
    let play_state: &mut PlayState;

    match state {
        State::Play(data) => {
            play_state = data;
        }
        _ => {
            panic!("Illegal state.");
        }
    }
    execute!(w, terminal::Clear(terminal::ClearType::All))?;

    if play_state.player_color.is_some() {
        player_vs_ai(w, state)?;
    } else {
        ai_vs_ai(w, state)?;
    }

    Ok(())
}

const AI_SCREEN: &str = r#"Configure AI

Controls:
 - Type numerical value and hit enter.
 - 'q' - quit or return to this menu

"#;

fn ai_screen<W>(w: &mut W, state: &mut State) -> Result<()>
where
    W: Write,
{
    let player_color: Option<TileColor>;
    let ai_color: TileColor;

    match state {
        State::AiMenu(Some(c)) => {
            if *c == TileColor::Black {
                player_color = Some(TileColor::Black);
                ai_color = TileColor::White;
            } else {
                player_color = Some(TileColor::White);
                ai_color = TileColor::Black;
            }
        }
        State::AiMenu(None) => {
            player_color = None;
            ai_color = TileColor::White;
        }
        _ => {
            panic!("Illegal state.");
        }
    }

    queue!(
        w,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(1, 1)
    )?;

    for line in AI_SCREEN.split('\n') {
        queue!(w, Print(line), cursor::MoveToNextLine(1))?;
    }

    w.flush()?;

    terminal::disable_raw_mode()?;

    let r = stdin();
    let mut input = String::new();
    let depth: Option<u8>;
    let duration_ms: Option<Duration>;

    loop {
        execute!(w, Print("Alpha Beta AI search depth: ".to_string()))?;

        r.read_line(&mut input)?;

        match input.trim().parse::<u8>() {
            Ok(n) => {
                if n == 0 {
                    depth = None;
                } else {
                    depth = Some(n);
                }
                break;
            }
            _ => {
                continue;
            }
        }
    }

    input = String::new();

    loop {
        execute!(w, Print("Alpha Beta AI search duration [ms]: ".to_string()),)?;

        r.read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(n) => {
                if n == 0 {
                    duration_ms = None;
                } else {
                    duration_ms = Some(Duration::new(0, n * 1000 * 1000));
                }
                break;
            }
            _ => {
                continue;
            }
        }
    }

    if player_color.is_some() {
        *state = State::Play(PlayState {
            state: GameState::new(),
            agent: Agent::new(ai_color, depth, duration_ms),
            agent2: None,
            player_color: player_color,
            focus: Coordinate {
                x: logic::WIDTH / 2,
                y: 0,
            },
            selected: None,
            selected_command: None,
        });
    } else {
        *state = State::Play(PlayState {
            state: GameState::new(),
            agent: Agent::new(TileColor::Black, depth, duration_ms),
            // There is only one kind of AI for now.
            agent2: Some(Agent::new(TileColor::White, depth, duration_ms)),
            player_color: None,
            focus: Coordinate {
                x: logic::WIDTH / 2,
                y: 0,
            },
            selected: None,
            selected_command: None,
        });
    }

    terminal::enable_raw_mode()?;

    Ok(())
}

const MAIN_MENU: &str = r#"Rusty Duke

Main Menu:
- Press number to choose menu item.
- 'q' - quit or return to this menu

Select color:

1. Black
2. White
3. AI vs AI

"#;

fn main_menu<W>(w: &mut W, state: &mut State) -> Result<()>
where
    W: Write,
{
    match state {
        State::MainMenu => {}
        _ => {
            panic!("Illegal state.");
        }
    }

    queue!(
        w,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    for line in MAIN_MENU.split('\n') {
        queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
    }

    w.flush()?;

    loop {
        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') => {
                *state = State::Exit;
                break;
            }
            Event::Key(event) if event.code == KeyCode::Char('1') => {
                *state = State::AiMenu(Some(TileColor::Black));
                break;
            }
            Event::Key(event) if event.code == KeyCode::Char('2') => {
                *state = State::AiMenu(Some(TileColor::White));
                break;
            }
            Event::Key(event) if event.code == KeyCode::Char('3') => {
                *state = State::AiMenu(None);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    let old_size = terminal::size()?;
    terminal::enable_raw_mode()?;

    println!("{} {}", TERM_WIDTH, TERM_HEIGHT);
    execute!(
        w,
        terminal::SetSize(TERM_WIDTH, TERM_HEIGHT),
        SetTitle("Rusty Duke")
    )?;

    let mut state = &mut State::MainMenu;

    loop {
        match state {
            State::MainMenu => {
                main_menu(w, &mut state)?;
            }
            State::AiMenu(_) => {
                ai_screen(w, &mut state)?;
            }
            State::Play(_) => {
                play(w, &mut state)?;
            }
            State::Exit => {
                break;
            }
        }
    }

    execute!(
        w,
        ResetColor,
        SetTitle(""),
        terminal::Clear(terminal::ClearType::All),
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;
    terminal::SetSize(old_size.0, old_size.1);

    Ok(())
}

// FIXME: Terminal cleanup on SIGTERM.

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Init logger
    Logger::try_with_str("debug")?
        .log_to_file(FileSpec::default())
        .start()?;

    let mut stdout = io::stdout();
    run(&mut stdout)?;
    Ok(())
}
