//! Implements logic for the Rusty Duke game.

use std::collections::HashMap;
use std::fmt;
pub use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Width of game board in squares.
pub const WIDTH: u8 = 6;
/// Height of game board in squares.
pub const HEIGHT: u8 = 6;

/// Board Coordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    // FIXME: Use wrapping and/or ranged integers?
    pub x: u8,
    pub y: u8,
}

impl Coordinate {
    pub fn legal(x: u8, y: u8) -> bool {
        x < WIDTH && y < HEIGHT
    }

    // FIXME: Make sure all coordinates are made with new() to avoid bugs.
    pub fn new(x: u8, y: u8) -> Coordinate {
        assert!(Coordinate::legal(x, y));
        Coordinate { x: x, y: y }
    }
}

/// Direction relative to tile.
struct Direction {
    x: i8,
    y: i8,
}

/// Offset relative to tile.
#[derive(Debug, Clone)]
pub struct Offset {
    pub x: i8,
    pub y: i8,
}

fn invert_offset(offset: &Offset) -> Offset {
    Offset {
        x: offset.x * -1,
        y: offset.y * -1,
    }
}

/// Take two coordinates and calculate direction. Only works for straight lines and diagonals.
fn get_direction(start: Coordinate, end: Coordinate) -> Direction {
    debug_assert!(start != end);

    let mut dir = Direction { x: 0, y: 0 };

    if start.x < end.x {
        dir.x = 1;
    } else if start.x > end.x {
        dir.x = -1;
    }

    if start.y < end.y {
        dir.y = 1;
    } else if start.y > end.y {
        dir.y = -1;
    }

    return dir;
}

/// Effect imposed by tile on square.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    Dread,
    Defence,
}

/// Square on board. Can have a tile and effects.
#[derive(Debug, Clone)]
pub struct Square {
    pub effects: Vec<Effect>,
    pub tile: Option<Tile>,
}

impl Default for Square {
    fn default() -> Self {
        Square {
            effects: Vec::new(),
            tile: None,
        }
    }
}

/// Action type that a tile can perform.
#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    NewFromBag,
    PlaceNew,
    Move,
    Jump,
    JumpSlide,
    Slide,
    Command,
    Strike,
}


/// Data included with standard tile action.
#[derive(Debug, Clone, Copy)]
pub struct ActionData {
    pub tile_pos: Coordinate,
    pub target_pos: Coordinate,
    pub result: ActionResult,
}

/// Data included with command tile action.
#[derive(Debug, Clone, Copy)]
pub struct CommandActionData {
    pub tile_pos: Coordinate,
    pub command_tile_pos: Coordinate,
    pub target_pos: Coordinate,
    pub result: ActionResult,
}

/// Action that a tile can perform.
#[derive(Debug, Clone, Copy)]
pub enum Action {
    NewFromBag,
    PlaceNew(Coordinate),
    Move(ActionData),
    Jump(ActionData),
    JumpSlide(ActionData),
    Slide(ActionData),
    Command(CommandActionData),
    Strike(ActionData),
}

/// Result that action has on game state.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActionResult {
    Move,
    Capture,
}

/// Specifies an action of a tile type.
#[derive(Debug, Clone)]
pub struct AvailableAction {
    pub kind: ActionType,
    offset: Offset,
}

/// Specifies an effect of a tile type.
#[derive(Debug, Clone)]
pub struct AvailableEffect {
    pub kind: Effect,
    offset: Offset,
}

/// Specifies possible tile colors.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TileColor {
    Black,
    White,
}

/// Contains winner of game.
#[derive(Debug, Clone, PartialEq)]
pub enum Winner {
    Color(TileColor),
    //Draw, Draw does not exist in duke?
}

/// Tile type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum TileType {
    // Basic tiles
    Duke,
    Footman,
    Pikeman,
    Knight,
    Bowman,
    LightHorse,
    Wizard,
    Seer,
    Champion,
    Arbalist,
    General,
    Marshall,
    Countess,
    Ranger,
    Sage,
    RoyalAssassin,
    // Arthurian legends tiles
    /*Arthur,
    Guinevere,
    Lancelot,
    Perceval,
    Merlin,
    Camelot,
    Morgana,
    Mordred,*/
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Actions that a tile type can perform.
pub struct AvailableActions {
    pub front: Vec<AvailableAction>,
    pub back: Vec<AvailableAction>,
}

/// Effects that a tile type can perform.
pub struct AvailableEffects {
    pub front: Vec<AvailableEffect>,
    pub back: Vec<AvailableEffect>,
}

/// Tile that can be played. Will be owned by bag, board or graveyard.
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub kind: TileType,
    pub flipped: bool,
    pub color: TileColor,
}

impl Tile {
    fn flip(&mut self) {
        self.flipped = !self.flipped;
    }
}

lazy_static! {

    /// This is where tile types are defined.
    pub static ref TILE_ACTIONS: HashMap<TileType, AvailableActions> = {

        // FIXME: Specify tiles and rules in configuration file.
        // FIXME: Use more suitable hashing algorithm.

        // For future reference:
        // for some reason HashMap method `from` does not work here.
        // Maybe related to lazy static.

        let mut m = HashMap::new();
        m.insert(
            TileType::Duke, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: -1, y: 0 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 0, y: -1 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Footman, /* Type */
            AvailableActions {
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 1 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Pikeman, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 2, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -2, y: 2 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 1, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: -1, y: 2 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Knight, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 1, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -1, y: 2 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 2, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -2, y: -2 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Bowman, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 0 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: -1, y: 1 },
                    },
                ],
            }
        );
        m.insert(
            TileType::LightHorse, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 1, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: -1, y: 2 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Wizard, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 1 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 2 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Seer, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 1 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 2 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Champion, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 0 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 0 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Arbalist, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Strike,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 1, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -1, y: -2 },
                    },
                ],
            }
        );
        m.insert(
            TileType::General, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 1, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -1, y: 2 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 1, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -1, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: -1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: -1, y: 0 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Marshall, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 2 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: 1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: -1, y: 1 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Countess, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: -1, y: 0 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Command,
                        offset: Offset { x: -2, y: 0 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Ranger, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 1, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 2, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -2, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -1 , y: 2 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 1, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: 1, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Jump,
                        offset: Offset { x: -1, y: -2 },
                    },
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: -1, y: 1 },
                    },
                ],
            }
        );
        m.insert(
            TileType::Sage, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 1, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -1, y: -1 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: 2 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 2, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: -2, y: 0 },
                    },
                ],
            }
        );
        m.insert(
            TileType::RoyalAssassin, /* Type */
            AvailableActions{
                front: vec![
                    /* Front side */
                    AvailableAction {
                        kind: ActionType::Move,
                        offset: Offset { x: 0, y: -1 },
                    },
                ],
                back: vec![
                    /* Back side */
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 0, y: 1 },
                    },
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 1, y: 0 },
                    },
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: 0, y: -1 },
                    },
                    AvailableAction {
                        kind: ActionType::Slide,
                        offset: Offset { x: -1, y: 0 },
                    },
                ],
        }
        );
        m
    };

    pub static ref NO_EFFECTS: AvailableEffects = AvailableEffects{front: vec![], back: vec![]};

    pub static ref TILE_EFFECTS: HashMap<TileType, AvailableEffects>  = {
        let m = HashMap::new();
            // Effect tiles will be added.
            m
    };

    /// Same as `TILE_ACTIONS` but inverted offsets. (For white player.)
    static ref INVERTED_TILE_ACTIONS: HashMap<TileType, AvailableActions> = {

        let mut inverted_tile_actions = HashMap::new();

        for (key, val) in TILE_ACTIONS.iter() {

            let inverted_available_actions = AvailableActions{
                front:
                    val.front.iter().map(|a|
                        AvailableAction{
                            kind: a.kind.clone(),
                            offset: invert_offset(&a.offset)
                        }
                    ).collect(),
                back:
                    val.back.iter().map(|a|
                        AvailableAction{
                            kind: a.kind.clone(),
                            offset: invert_offset(&a.offset)
                        }
                    ).collect()
                };

            inverted_tile_actions.insert(*key, inverted_available_actions);
        }

        inverted_tile_actions
    };

    /// Same as `TILE_EFFECTS` but inverted offsets. (For white player.)
    static ref INVERTED_TILE_EFFECTS: HashMap<TileType, AvailableEffects> = {

        let mut inverted_tile_effects = HashMap::new();

        for (key, val) in TILE_EFFECTS.iter() {

            let inverted_available_effects = AvailableEffects{
                front:
                    val.front.iter().map(|a|
                        AvailableEffect{
                            kind: a.kind.clone(),
                            offset: invert_offset(&a.offset)
                        }
                    ).collect(),
                back:
                    val.back.iter().map(|a|
                        AvailableEffect{
                            kind: a.kind.clone(),
                            offset: invert_offset(&a.offset)
                        }
                    ).collect()
                };

            inverted_tile_effects.insert(*key, inverted_available_effects);
        }

        inverted_tile_effects
    };
}

impl Tile {
    fn new(kind: TileType, color: TileColor) -> Tile {
        Tile {
            kind: kind,
            flipped: false,
            color: color,
        }
    }

    fn actions(&self) -> &'static AvailableActions {
        if self.color == TileColor::Black {
            return TILE_ACTIONS
                .get(&self.kind)
                .as_ref()
                .expect("Illegal tile type.");
        } else {
            return INVERTED_TILE_ACTIONS
                .get(&self.kind)
                .as_ref()
                .expect("Illegal tile type.");
        }
    }

    fn effects(&self) -> &'static AvailableEffects {
        if self.color == TileColor::Black {
            let effects = TILE_EFFECTS.get(&self.kind);
            if effects.is_some() {
                return effects.unwrap();
            } else {
                return &NO_EFFECTS;
            }
        } else {
            let effects = INVERTED_TILE_EFFECTS.get(&self.kind);
            if effects.is_some() {
                return effects.unwrap();
            } else {
                return &NO_EFFECTS;
            }
        }
    }
}

/// Complete state of a duke game. Bag, board and graveyard are owner of tiles.
#[derive(Clone, Debug)]
pub struct GameState {
    /// Game board.
    pub board: [[Square; WIDTH as usize]; HEIGHT as usize],
    /// Tiles go here before they are deployed to board. One bag per player.
    pub bags: [Vec<Tile>; 2],
    /// When one draws a new tile it is placed here in limbo. One queue for each player.
    pub drawn_tiles: [Vec<Tile>; 2],
    /// Dead tiles go here
    pub graveyard: Vec<Tile>,
    /// Specifies color of current player.
    pub ply: TileColor,
    /// Stores winner if any.
    pub game_over: Option<Winner>,
    /// Put duke positions here to avoid extra search
    dukes: [Option<Coordinate>; 2],
}

impl GameState {
    /// Initialize bags
    fn init_tiles(color: TileColor) -> Vec<Tile> {
        let mut tiles = Vec::new();

        // Add footmen

        tiles.push(Tile::new(TileType::Footman, color));

        // Add pikemen
        tiles.push(Tile::new(TileType::Pikeman, color));
        tiles.push(Tile::new(TileType::Pikeman, color));
        tiles.push(Tile::new(TileType::Pikeman, color));

        // Add bowmen
        tiles.push(Tile::new(TileType::Knight, color));

        // Add knights
        tiles.push(Tile::new(TileType::Bowman, color));

        // Add light horses
        tiles.push(Tile::new(TileType::LightHorse, color));

        // Add wizards
        tiles.push(Tile::new(TileType::Wizard, color));

        // Add Seer
        tiles.push(Tile::new(TileType::Seer, color));

        // Add Champion
        tiles.push(Tile::new(TileType::Champion, color));

        // Add Arbalist
        tiles.push(Tile::new(TileType::Arbalist, color));

        // Add General
        tiles.push(Tile::new(TileType::General, color));

        // Add Marshall
        tiles.push(Tile::new(TileType::Marshall, color));

        // Add Countess
        tiles.push(Tile::new(TileType::Countess, color));

        // Add Ranger
        tiles.push(Tile::new(TileType::Ranger, color));

        // Add Sage
        tiles.push(Tile::new(TileType::Sage, color));

        // Add RoyalAssassin
        tiles.push(Tile::new(TileType::RoyalAssassin, color));

        // Arthurian legends tiles

        /*    if arthurian_legends {
            todo!();

            // Add Arthur
            tiles.push(Tile::new(TileType::RoyalAssassin, color));

            // Add Guinevere
            tiles.push(Tile::new(TileType::RoyalAssassin, color));

            // Add Lancelot
            tiles.push(Tile::new(TileType::RoyalAssassin, color));

            // Add Perceval
            tiles.push(Tile::new(TileType::RoyalAssassin, color));

            // Add Merlin
            tiles.push(Tile::new(TileType::RoyalAssassin, color));

            // Add Camelot
            tiles.push(Tile::new(TileType::RoyalAssassin, color));

            // Add Morgana
            tiles.push(Tile::new(TileType::RoyalAssassin, color));

            // Add Mordred
            tiles.push(Tile::new(TileType::RoyalAssassin, color));
        }*/

        tiles
    }

    pub fn new() -> GameState {
        // These are the first three tiles that will be deployed. In the right
        // order.
        let mut new_black_tiles: Vec<Tile> = Vec::new();
        new_black_tiles.push(Tile::new(TileType::Footman, TileColor::Black));
        new_black_tiles.push(Tile::new(TileType::Footman, TileColor::Black));
        new_black_tiles.push(Tile::new(TileType::Duke, TileColor::Black));

        let mut new_white_tiles: Vec<Tile> = Vec::new();
        new_white_tiles.push(Tile::new(TileType::Footman, TileColor::White));
        new_white_tiles.push(Tile::new(TileType::Footman, TileColor::White));
        new_white_tiles.push(Tile::new(TileType::Duke, TileColor::White));

        GameState {
            /*board: [(); HEIGHT as usize]
            .map(|_| [(); WIDTH as usize]
                .map(|_| Square{effects: Vec::new(), tile: None})),*/
            board: Default::default(),
            bags: [
                GameState::init_tiles(TileColor::Black),
                GameState::init_tiles(TileColor::White),
            ],
            drawn_tiles: [new_black_tiles, new_white_tiles],
            graveyard: Vec::new(),
            ply: TileColor::Black, // Black always start
            game_over: None,
            dukes: [None; 2], // Duke board positions, to decrease amount of search.
        }
    }

    /// Borrow of bag for current ply
    pub fn bag(&self) -> &Vec<Tile> {
        &self.bags[self.ply as usize]
    }

    /// Borrow of  drawn tile(s) for current ply
    pub fn drawn(&self) -> &Vec<Tile> {
        &self.drawn_tiles[self.ply as usize]
    }

    /// Borrow of players duke board position for current ply
    pub fn own_duke_pos(&self) -> &Option<Coordinate> {
        &self.dukes[self.ply as usize]
    }

    /// Borrow of opponent duke board position for current ply
    pub fn opponent_duke_pos(&self) -> &Option<Coordinate> {
        if self.ply == TileColor::Black {
            &self.dukes[TileColor::White as usize]
        } else {
            &self.dukes[TileColor::Black as usize]
        }
    }

    /// Borrow of square
    pub fn square(&self, cord: Coordinate) -> &Square {
        &self.board[cord.y as usize][cord.x as usize]
    }

    /// Mut borrow of bag for current ply
    pub fn mut_bag(&mut self) -> &mut Vec<Tile> {
        &mut self.bags[self.ply as usize]
    }

    /// Mut borrow of  drawn tile(s) for current ply
    pub fn mut_drawn(&mut self) -> &mut Vec<Tile> {
        &mut self.drawn_tiles[self.ply as usize]
    }

    /// Mut borrow of players duke board position for current ply
    pub fn mut_own_duke_pos(&mut self) -> &mut Option<Coordinate> {
        &mut self.dukes[self.ply as usize]
    }

    /// Mut borrow of opponent duke board position for current ply
    pub fn mut_opponent_duke_pos(&mut self) -> &mut Option<Coordinate> {
        if self.ply == TileColor::Black {
            &mut self.dukes[TileColor::White as usize]
        } else {
            &mut self.dukes[TileColor::Black as usize]
        }
    }

    /// Mut borrow of square
    pub fn mut_square(&mut self, cord: Coordinate) -> &mut Square {
        &mut self.board[cord.y as usize][cord.x as usize]
    }
}

/// Check if square effects prevent tile from doing anything at all.
fn tile_can_act(state: &GameState, tile: (Coordinate, &Tile)) -> bool {
    let square = &state.board[tile.0.y as usize][tile.0.x as usize];

    for effect in &square.effects {
        if *effect == Effect::Dread && tile.1.kind != TileType::Duke {
            return false;
        }
    }

    true
}

/// Check if path between two coordinates is straight
fn straight_path(start: &Coordinate, end: &Coordinate) -> bool {
    debug_assert!(start != end);
    debug_assert!(Coordinate::legal(start.x, start.y));
    debug_assert!(Coordinate::legal(end.x, end.y));

    // Vertical or horizontal
    if start.x == end.x || start.y == end.y {
        return true;
    }

    // Make things more readable.
    let start_x = start.x as i8;
    let start_y = start.y as i8;
    let end_x = end.x as i8;
    let end_y = end.y as i8;

    // Diagonal
    if (end_x - start_x).abs() == (end_y - start_y).abs() {
        return true;
    }

    false
}

/// Check if path between two coordinates is blocked. Check from square
/// adjacent to start to and including end square.
fn path_blocked(
    state: &GameState,
    tile_color: TileColor,
    action_type: ActionType,
    start: Coordinate,
    end: Coordinate,
) -> bool {
    debug_assert!(Coordinate::legal(start.x, start.y));
    debug_assert!(Coordinate::legal(end.x, end.y));

    let board = &state.board;

    // FIXME: Create lookup table? That might be more efficient and more readable.

    let dir = get_direction(start, end);

    // Straight path?
    if straight_path(&start, &end) {
        let mut cord = Coordinate::new(start.x, start.y);
        loop {
            cord.x = (cord.x as i8 + dir.x) as u8;
            cord.y = (cord.y as i8 + dir.y) as u8;

            let square = state.square(cord);

            // Check if path is blocked by defence
            for effect in &square.effects {
                if *effect == Effect::Defence {
                    return true;
                }
            }

            if cord != end {
                // Move is blocked by any tile in way.
                if action_type == ActionType::Move && square.tile.is_some() {
                    return true;
                }
            } else {
                // Any action is blocked by tile of same color on final square.
                if square.tile.is_some() {
                    let t = square.tile.as_ref().unwrap();
                    if tile_color == t.color {
                        return true;
                    }
                }
                break;
            }
        }
    } else {
        // Non-straight path

        // Closure to remove some redundant code. Hope it's not more confusing.
        let non_straight_blocked = |x_first: bool| -> bool {
            let mut cord = start.clone();

            // First axis
            loop {
                if x_first {
                    cord.x = (cord.x as i8 + dir.x) as u8;
                } else {
                    cord.y = (cord.y as i8 + dir.y) as u8;
                }

                let square = state.square(cord);

                // Check if path is blocked by defence
                for effect in &square.effects {
                    if *effect == Effect::Defence {
                        return true;
                    }
                }

                // Move is blocked by any tile in way.
                if action_type == ActionType::Move && square.tile.is_some() {
                    return true;
                }

                if x_first {
                    if cord.x == end.x {
                        break;
                    }
                } else {
                    if cord.y == end.y {
                        break;
                    }
                }
            }

            // Second axis
            loop {
                if x_first {
                    cord.y = (cord.y as i8 + dir.y) as u8;
                } else {
                    cord.x = (cord.x as i8 + dir.x) as u8;
                }

                let square = &board[cord.y as usize][cord.x as usize];

                // Check if path is blocked by defence
                for effect in &square.effects {
                    if *effect == Effect::Defence {
                        return true;
                    }
                }

                if x_first && (cord.y != end.y) || (!x_first) && (cord.x != end.x) {
                    // Move is blocked by any tile in way.
                    if action_type == ActionType::Move && square.tile.is_some() {
                        return true;
                    }
                } else {
                    // Any action is blocked by tile of same color on final square.
                    if square.tile.is_some() {
                        let t = square.tile.as_ref().unwrap();
                        if tile_color == t.color {
                            return true;
                        }
                    }

                    return false;
                }
            }
        };

        if non_straight_blocked(true) && non_straight_blocked(false) {
            return true;
        }
    }

    false
}

/// Get legal move action if any. Only valid coordinates.
fn get_move_action(
    state: &GameState,
    tile: (Coordinate, &Tile),
    target: Coordinate,
) -> Option<Action> {
    debug_assert!(Coordinate::legal(target.x, target.y));

    if path_blocked(state, tile.1.color, ActionType::Move, tile.0, target) {
        return None;
    }

    let square = &state.board[target.y as usize][target.x as usize];

    if square.tile.is_some() {
        let blocking_tile = square.tile.as_ref().unwrap();
        if tile.1.color != blocking_tile.color {
            return Some(Action::Move(ActionData {
                tile_pos: tile.0,
                target_pos: target,
                result: ActionResult::Capture,
            }));
        } else {
            return None;
        }
    }

    return Some(Action::Move(ActionData {
        tile_pos: tile.0,
        target_pos: target,
        result: ActionResult::Move,
    }));
}

/// Get slide or jumpslide action(s). Each square in path generate one action.
/// Only valid coordinates.
fn get_slide_actions(
    state: &GameState,
    tile: (Coordinate, &Tile),
    jumpslide: bool,
    start: Coordinate,
) -> Vec<Action> {
    debug_assert!(Coordinate::legal(start.x, start.y));

    let board = &state.board;
    let mut x = start.x;
    let mut y = start.y;
    let dir = get_direction(tile.0, start);
    let mut actions: Vec<Action> = Vec::new();

    // Check if jump is blocked.
    if jumpslide {
        if path_blocked(state, tile.1.color, ActionType::Jump, tile.0, start) {
            return actions;
        }
    }

    while x < WIDTH && y < HEIGHT {
        let square = &board[y as usize][x as usize];

        // Check if path is blocked by defence
        for effect in &square.effects {
            if *effect == Effect::Defence {
                return actions;
            }
        }

        // If tile in path, stop or capture.
        if square.tile.is_some() {
            let blocking_tile = square.tile.as_ref().unwrap();
            if tile.1.color != blocking_tile.color {
                if jumpslide {
                    actions.push(Action::JumpSlide(ActionData {
                        tile_pos: tile.0,
                        target_pos: Coordinate { x: x, y: y },
                        result: ActionResult::Capture,
                    }));
                } else {
                    actions.push(Action::Slide(ActionData {
                        tile_pos: tile.0,
                        target_pos: Coordinate { x: x, y: y },
                        result: ActionResult::Capture,
                    }));
                }
            }
            return actions;
        }

        if jumpslide {
            actions.push(Action::JumpSlide(ActionData {
                tile_pos: tile.0,
                target_pos: Coordinate { x: x, y: y },
                result: ActionResult::Move,
            }));
        } else {
            actions.push(Action::Slide(ActionData {
                tile_pos: tile.0,
                target_pos: Coordinate { x: x, y: y },
                result: ActionResult::Move,
            }));
        }

        x = (x as i8 + dir.x) as u8;
        y = (y as i8 + dir.y) as u8;
    }

    actions
}

/// Get legal jump action, if any. Only valid coordinates.
fn get_jump_action(
    state: &GameState,
    tile: (Coordinate, &Tile),
    target: Coordinate,
) -> Option<Action> {
    debug_assert!(Coordinate::legal(target.x, target.y));

    if path_blocked(state, tile.1.color, ActionType::Jump, tile.0, target) {
        return None;
    }

    let square = &state.board[target.y as usize][target.x as usize];

    if square.tile.is_some() {
        let blocking_tile = square.tile.as_ref().unwrap();
        if tile.1.color != blocking_tile.color {
            return Some(Action::Jump(ActionData {
                tile_pos: tile.0,
                target_pos: target,
                result: ActionResult::Capture,
            }));
        } else {
            return None;
        }
    }

    return Some(Action::Jump(ActionData {
        tile_pos: tile.0,
        target_pos: target,
        result: ActionResult::Move,
    }));
}

/// Get legal jump action, if any. Only valid coordinates.
fn get_strike_action(
    state: &GameState,
    tile: (Coordinate, &Tile),
    target: Coordinate,
) -> Option<Action> {
    debug_assert!(Coordinate::legal(target.x, target.y));

    if path_blocked(state, tile.1.color, ActionType::Jump, tile.0, target) {
        return None;
    }

    let square = &state.board[target.y as usize][target.x as usize];

    if square.tile.is_some() {
        let blocking_tile = square.tile.as_ref().unwrap();
        if tile.1.color != blocking_tile.color {
            return Some(Action::Strike(ActionData {
                tile_pos: tile.0,
                target_pos: target,
                result: ActionResult::Capture,
            }));
        } else {
            return None;
        }
    }

    None
}

/// Get command actions. Only valid coordinates.
fn get_command_actions(
    state: &GameState,
    tile: (Coordinate, &Tile),
    target: Coordinate,
) -> Vec<Action> {
    debug_assert!(Coordinate::legal(target.x, target.y));

    let mut actions: Vec<Action> = Vec::new();
    let command_square = state.square(target);

    if command_square.tile.is_none() {
        return actions;
    }

    if command_square.tile.as_ref().unwrap().color != tile.1.color {
        return actions;
    }

    // Command actions can't be blocked.

    // Get all command squares
    let mut command_squares: Vec<Coordinate> = Vec::new();
    let mut push_cord = |a: &AvailableAction| {
        if a.kind == ActionType::Command {
            let x = (tile.0.x as i8 + a.offset.x) as u8;
            let y = (tile.0.y as i8 + a.offset.y) as u8;
            if Coordinate::legal(x, y) {
                command_squares.push(Coordinate::new(x, y));
            }
        }
    };

    if tile.1.flipped {
        for a in tile.1.actions().back.iter() {
            push_cord(&a);
        }
    } else {
        for a in tile.1.actions().front.iter() {
            push_cord(&a);
        }
    }

    for cord in command_squares {
        let square = state.square(cord);
        if square.tile.is_some() {
            let t = square.tile.as_ref().unwrap();

            // Will not move to own square or to one occupied by same color.
            // This avoids multiple checks for target cord in push_cord.
            if t.color != tile.1.color {
                actions.push(Action::Command(CommandActionData {
                    tile_pos: tile.0,
                    command_tile_pos: target,
                    target_pos: cord,
                    result: ActionResult::Capture,
                }));
            }
        } else {
            actions.push(Action::Command(CommandActionData {
                tile_pos: tile.0,
                command_tile_pos: target,
                target_pos: cord,
                result: ActionResult::Move,
            }));
        }
    }

    actions
}

pub fn get_spawn_squares(state: &GameState) -> Vec<Coordinate> {
    let mut squares: Vec<Coordinate> = Vec::new();

    if state.game_over.is_some() {
        return squares;
    }

    // If there is no duke, return initial spawn squares. Assume init.
    if state.own_duke_pos().is_none() {
        if !state.drawn().is_empty() && state.drawn().last().unwrap().kind == TileType::Duke {
            if state.ply == TileColor::Black {
                return vec![Coordinate { x: 2, y: 0 }, Coordinate { x: 3, y: 0 }];
            } else {
                return vec![
                    Coordinate {
                        x: 2,
                        y: HEIGHT - 1,
                    },
                    Coordinate {
                        x: 3,
                        y: HEIGHT - 1,
                    },
                ];
            }
        } else {
            panic!("Should be game over.");
        }
    }

    let duke_pos = state.own_duke_pos().unwrap();
    let mut check_n_add = |x: i8, y: i8| {
        if Coordinate::legal(x as u8, y as u8) {
            let cord = Coordinate::new(x as u8, y as u8);
            if state.square(cord).tile.is_none() {
                squares.push(cord);
            }
        }
    };

    // Check tile on right
    check_n_add(duke_pos.x as i8 + 1, duke_pos.y as i8);
    // Check tile on left
    check_n_add(duke_pos.x as i8 - 1, duke_pos.y as i8);
    // Check tile up
    check_n_add(duke_pos.x as i8, duke_pos.y as i8 + 1);
    // Check tile down
    check_n_add(duke_pos.x as i8, duke_pos.y as i8 - 1);

    squares
}

/// Get tile actions. Tile has to be in play. Also shows actions for who can not
/// play this ply.
pub fn get_tile_actions(state: &GameState, tile_pos: Coordinate) -> Vec<Action> {
    let mut actions = Vec::new();

    if state.game_over.is_some() {
        return actions;
    }

    if state.board[tile_pos.y as usize][tile_pos.x as usize]
        .tile
        .is_none()
    {
        return actions;
    }
    let tile = state.board[tile_pos.y as usize][tile_pos.x as usize]
        .tile
        .as_ref()
        .unwrap();

    // Check if tile is inhibited by some effect.
    if !tile_can_act(state, (tile_pos, tile)) {
        return actions;
    }

    // Let's get available actions for this tile.
    let avail_actions;

    // Each tile has a front and a back.
    if tile.flipped {
        avail_actions = &tile.actions().back;
    } else {
        avail_actions = &tile.actions().front;
    }

    // Check actual actions for evry available action. Some available actions,
    // like slide, can result in many actual actions. And some available actions,
    // does not produce any actual actions.
    for action in avail_actions {
        let x = (tile_pos.x as i8 + action.offset.x) as u8;
        let y = (tile_pos.y as i8 + action.offset.y) as u8;

        // Skip if cordinate is illegal.
        if !Coordinate::legal(x, y) {
            continue;
        }

        let target = Coordinate::new(x, y);

        match action.kind {
            ActionType::Move => {
                let action = get_move_action(state, (tile_pos, tile), target);
                if action.is_some() {
                    actions.push(action.unwrap());
                }
            }
            ActionType::Jump => {
                let action = get_jump_action(state, (tile_pos, tile), target);
                if action.is_some() {
                    actions.push(action.unwrap());
                }
            }
            ActionType::JumpSlide => {
                actions.append(&mut get_slide_actions(
                    state,
                    (tile_pos, tile),
                    true,
                    target,
                ));
            }
            ActionType::Slide => {
                actions.append(&mut get_slide_actions(
                    state,
                    (tile_pos, tile),
                    false,
                    target,
                ));
            }
            ActionType::Command => {
                actions.append(&mut get_command_actions(state, (tile_pos, tile), target));
            }
            ActionType::Strike => {
                let action = get_strike_action(state, (tile_pos, tile), target);
                if action.is_some() {
                    actions.push(action.unwrap());
                }
            }
            _ => {
                panic! {"Illegal action type: {:?}", action.kind};
            }
        }
    }

    actions
}

/// Get possible actions for a given game state.
pub fn get_actions(state: &GameState) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();

    if state.game_over.is_some() {
        return actions;
    }

    // Get squares where tiles can be spawned.
    let spawn_squares = get_spawn_squares(state);

    // Place drawn tile if any
    if !state.drawn().is_empty() {
        // If there is a drawn tile and no spawn squares, we have a bug.
        debug_assert!(
            !spawn_squares.is_empty(),
            "Drawn tile but no spawn squares."
        );

        for square in spawn_squares {
            actions.push(Action::PlaceNew(square));
        }

        return actions;
    }

    // Add any potential spawn actions first.
    if !spawn_squares.is_empty() && !state.bag().is_empty() {
        actions.push(Action::NewFromBag);
    }

    // Check each cordinate for available actions.
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cord = Coordinate::new(x, y);
            if state.square(cord).tile.is_some() {
                let tile = state.square(cord).tile.as_ref().unwrap();
                if tile.color == state.ply {
                    actions.append(&mut get_tile_actions(
                        state,
                        Coordinate {
                            x: x as u8,
                            y: y as u8,
                        },
                    ));
                }
            }
        }
    }

    actions
}

fn add_tile_effects(state: &mut GameState, tile_pos: Coordinate) {
    let tile = &state.board[tile_pos.y as usize][tile_pos.x as usize]
        .tile
        .expect("Add effects, but no tile.");
    let effects: &Vec<AvailableEffect>;

    if !tile.flipped {
        effects = &tile.effects().front;
    } else {
        effects = &tile.effects().back;
    }

    // Each effect position has to be calculated.
    // FIXME: Use effect references, again?
    for effect in effects {
        let effect_x = (tile_pos.x as i8 + effect.offset.x) as u8;
        let effect_y = (tile_pos.y as i8 + effect.offset.y) as u8;
        if Coordinate::legal(effect_x, effect_y) {
            state.board[effect_y as usize][effect_x as usize]
                .effects
                .push(effect.kind.clone());
        }
    }
}

fn clear_tile_effects(state: &mut GameState, tile_pos: Coordinate) {
    let tile = &state.board[tile_pos.y as usize][tile_pos.x as usize]
        .tile
        .expect("Clear effects, but no tile.");
    let effects: &Vec<AvailableEffect>;

    if !tile.flipped {
        effects = &tile.effects().front;
    } else {
        effects = &tile.effects().back;
    }

    // Each effect position has to be calculated.
    // FIXME: Use effect references, again?
    for effect in effects {
        let effect_x = (tile_pos.x as i8 + effect.offset.x) as u8;
        let effect_y = (tile_pos.y as i8 + effect.offset.y) as u8;
        if Coordinate::legal(effect_x, effect_y) {
            let square_effects = &mut state.board[effect_y as usize][effect_x as usize].effects;
            let mut effect_idx: Option<usize> = None;
            for i in 0..square_effects.len() {
                if square_effects[i] == effect.kind {
                    effect_idx = Some(i);
                    break;
                }
            }

            if effect_idx.is_some() {
                square_effects.swap_remove(effect_idx.unwrap());
            }
        }
    }
}

/// This function assumes that the action is legal. Only provide an action
/// returned by `get_actions` or `get_tile_actions` on the same state or bad
/// things will happen.
pub fn do_unsafe_action(state: &mut GameState, action: &Action) {
    let mut standard_action = |data: &ActionData| {
        let mut tile = state.square(data.tile_pos).tile.unwrap(); // Copy

        assert!(tile.color == state.ply);

        // Clear effects
        clear_tile_effects(state, data.tile_pos);

        // Flip
        tile.flip();

        // Clear source square
        state.mut_square(data.tile_pos).tile = None;

        // And do transfer of tile ownership. This is a bit messy due to ownership.
        if data.result == ActionResult::Capture {
            clear_tile_effects(state, data.target_pos);

            let captured = state.square(data.target_pos).tile.unwrap();
            if captured.kind == TileType::Duke {
                *state.mut_opponent_duke_pos() = None;
            }

            // Put captured in graveyard
            state.graveyard.push(captured);

            // Put action tile on square
            state.mut_square(data.target_pos).tile = Some(tile);
        } else {
            // Put action tile on square
            state.mut_square(data.target_pos).tile = Some(tile);
        }

        // Add effects
        add_tile_effects(state, data.target_pos);

        // If Duke, save Duke pos
        if tile.kind == TileType::Duke {
            *state.mut_own_duke_pos() = Some(data.target_pos.clone());
        }
    };

    // Do action on new state
    match action {
        Action::NewFromBag => {
            let index = (rand::random::<f32>() * state.bag().len() as f32).floor() as usize;
            let tile = state.mut_bag().swap_remove(index);
            state.mut_drawn().push(tile);

            // Don't update ply or game over. Just return. This is a special case.
            return;
        }
        Action::PlaceNew(cord) => {
            let tile = state.mut_drawn().pop().unwrap();

            assert!(tile.color == state.ply);

            if tile.kind == TileType::Duke {
                *state.mut_own_duke_pos() = Some(cord.clone());
            }
            state.mut_square(*cord).tile = Some(tile);

            // Add effects
            add_tile_effects(state, *cord);
        }
        Action::Move(data) | Action::Jump(data) | Action::JumpSlide(data) | Action::Slide(data) => {
            standard_action(data);
        }
        Action::Command(data) => {
            assert!(state.square(data.tile_pos).tile.as_ref().unwrap().color == state.ply);

            let tile = state.square(data.command_tile_pos).tile.unwrap(); // Copy

            // Clear commander effects
            clear_tile_effects(state, data.tile_pos);

            // Clear commanded effects
            clear_tile_effects(state, data.tile_pos);

            state.mut_square(data.command_tile_pos).tile = None;

            if data.result == ActionResult::Capture {
                clear_tile_effects(state, data.target_pos);
                let captured = state.square(data.target_pos).tile.unwrap();
                if captured.kind == TileType::Duke {
                    *state.mut_opponent_duke_pos() = None;
                }
                state.graveyard.push(captured);
                state.mut_square(data.target_pos).tile = Some(tile);
            } else {
                state.mut_square(data.target_pos).tile = Some(tile);
            }

            // Flip
            let commander = state.mut_square(data.tile_pos).tile.as_mut().unwrap();
            commander.flip();

            // Add effects
            add_tile_effects(state, data.tile_pos);
            add_tile_effects(state, data.target_pos);
        }
        Action::Strike(data) => {
            assert!(state.square(data.tile_pos).tile.as_ref().unwrap().color == state.ply);

            clear_tile_effects(state, data.target_pos);
            let captured = state.square(data.target_pos).tile.unwrap();
            if captured.kind == TileType::Duke {
                *state.mut_opponent_duke_pos() = None;
            }
            state.graveyard.push(captured);
            state.mut_square(data.target_pos).tile = None;

            // Flip
            clear_tile_effects(state, data.tile_pos);
            let tile = state.mut_square(data.tile_pos).tile.as_mut().unwrap();
            tile.flip();

            // Add effects
            add_tile_effects(state, data.tile_pos);
        }
    }

    // Update ply
    if state.ply == TileColor::Black {
        state.ply = TileColor::White;
    } else {
        state.ply = TileColor::Black;
    }

    /* let set_win = || {
        if state.ply == TileColor::Black {
            state.game_over = Some(Winner::Color(TileColor::White));
        } else {
            state.game_over = Some(Winner::Color(TileColor::Black));
        }
    };*/

    // Check if game over for new ply.
    if state.own_duke_pos().is_none() {
        let new_tile = state.drawn().last();
        if !(new_tile.is_some() && new_tile.unwrap().kind == TileType::Duke) {
            if state.ply == TileColor::Black {
                state.game_over = Some(Winner::Color(TileColor::White));
            } else {
                state.game_over = Some(Winner::Color(TileColor::Black));
            }
        }
    } else if get_actions(state).is_empty() {
        // Cant implement this with closure because of mut borrow rules.
        if state.ply == TileColor::Black {
            state.game_over = Some(Winner::Color(TileColor::White));
        } else {
            state.game_over = Some(Winner::Color(TileColor::Black));
        }
    }
}

/// Same as `do_unsafe_action` but returns copy of new state. For recursive AI search.
pub fn do_unsafe_action_copy(state: &GameState, action: &Action) -> GameState {
    let mut new_state = state.clone();
    do_unsafe_action(&mut new_state, action);
    new_state
}
