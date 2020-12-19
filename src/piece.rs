#[derive(PartialEq, Clone, Copy, Debug)]
enum ActionType {
    Spawn,
    Move,
    Jump,
    JumpSlide,
    Slide,
    Command,
    Strike,
    Dread,
    Defense,
}

// FIXME: Figure out how capture can point at the correct piece using explicit
// lifetime.
#[derive(PartialEq, Debug)]
enum ActionResult {
    Move,
    Capture
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Cordinate {
    x: u32,
    y: u32,
}

impl Cordinate {
    pub fn legal(&self) -> bool {
        self.x < BOARD_WIDTH && self.y < BOARD_HEIGHT
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Offset {
    x: i32,
    y: i32,
}

// TODO: Probably need to implement an available action validator.
#[derive(PartialEq, Debug)]
struct AvailableAction {
    kind: ActionType,
    offset: Offset,
    dir: Option<Direction>,
}

#[derive(PartialEq, Debug)]
struct Action {
    kind: ActionType,
    pos: Cordinate,
    result: ActionResult,
}

// NOTE: Consider using state objects instead
#[derive(PartialEq, Debug)]
enum PieceState {
    Init,
    Play,
    Dead,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum PieceColor {
    Black,
    White,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum PieceType {
    Duke,
    Footman,
    Knight,
    Bowman,
}

#[derive(PartialEq, Debug)]
struct Piece {
    kind: PieceType,
    flipped: bool,
    color: PieceColor,
    pos: Cordinate,
    actions_front: Vec<AvailableAction>,
    actions_back: Vec<AvailableAction>,
    state: PieceState,
    board: Weak<Board>
}

/*struct Board<'a> {
    tiles: Vec<Option<&a'Piece<'a>>>,
    width: u32,
    height: u32,
}*/

/// The game state is completely stored in the pieces.This might turn out super
/// inefficient.
impl Piece {

    pub fn new(&mut self, kind: PieceType, color: PieceColor) -> Piece {
        let mut new_piece = Piece {
            kind: kind,
            flipped: false,
            color: color,
            pos: Cordinate{x: 0, y: 0},
            actions_front: Vec::new(),
            actions_back: Vec::new(),
            state: PieceState::Init,
        };

        // TODO: Put piece definitions in structs or at least somewhere else.
        match kind {
            PieceType::Duke => {
                new_piece.actions_front.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_front.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_back.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_back.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
            }
            PieceType::Footman => {
                new_piece.actions_front.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_front.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_back.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_back.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
            }
            PieceType::Knight => {
                new_piece.actions_front.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_front.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_back.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_back.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
            }
            PieceType::Bowman => {
                new_piece.actions_front.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_front.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_back.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
                new_piece.actions_back.push(AvailableAction {
                    kind: ActionType::Slide,
                    offset: Offset { x: 1, y: 0 },
                    dir: None,
                });
            }
        }

        new_piece
    }

    pub fn set_pos(&mut self, pos: Cordinate) {
        self.pos = pos;
    }

    /// Only straight lines and diagonals are allowed. Assume legal cordinates.
    fn piece_in_way(
        &self,
        pcs: Vec<Piece>,
        start: Cordinate,
        end: Cordinate)
    -> Option<&Piece> {

        let piece: Option<&Piece> = None;

        let mut delta_x: i32;
        if start.x < end.x {
            delta_x = 1;
        }
        else if start.x > end.x {
            delta_x = -1;
        }
        else { // equal
            delta_x = 0;
        }

        let mut delta_y: i32;
        if start.y < end.y {
            delta_y = 1;
        }
        else if start.y > end.y {
            delta_y = -1;
        }
        else { // equal
            delta_y = 0;
        }

        let mut x = start.x as i32;
        let mut y = start.y as i32;
        let end_x = end.x as i32;
        let end_y = end.y as i32;

        while y != end_y {
            while x != end_x {
                piece = pcs.iter().find(|&&p|
                    p.pos.x == x && p.pos.y == y
                );
            }
        }

        let closest_piece: Option<&Piece> = pcs.iter().filter(|p|
            p.state == PieceState::Play &&
            (p.pos.x == start.x && p.pos.y >= start.y)
        ).min_by_key(|p| p.pos.x);

        piece
    }

    fn slide_actions(
        &self,
        pcs: Vec<Piece>,
        offset: Offset,
        dir: Direction,
        jump: bool,
    ) -> Vec<Action>
    {
        assert!(self.state == PieceState::Play);

        let mut actions = Vec::new();

        let x = self.pos.x;
        let y = self.pos.y;

        let mut start = Cordinate{x: 0, y: 0};

        if self.color == PieceColor::White {
            start.x = (x as i32 + offset.x * -1) as u32;
            start.y = (y as i32 + offset.y * -1) as u32;
        }
        else {
            start.x = (x as i32 + offset.x) as u32;
            start.y = (y as i32 + offset.y) as u32;
        }

        if start.x > BOARD_WIDTH || start.y > BOARD_HEIGHT {
            return actions;
        }

        let action = if jump {ActionType::JumpSlide} else { ActionType::Slide };

        // FIXME: This code might be too complex and redundant..
        // FIXME: Searching tiles instead of pieces would save much computation.
        match (self.color, dir) {
            (PieceColor::Black, Direction::Right) |
            (PieceColor::White, Direction::Left) => {
                let closest_piece: Option<&Piece> = pcs.iter().filter(|p|
                        p.state == PieceState::Play &&
                        (p.pos.x >= start.x && p.pos.y == start.y)
                    ).min_by_key(|p| p.pos.x);

                if closest_piece.is_some() {
                    let piece = closest_piece.unwrap();
                    for cord_x in start.x..piece.pos.x {
                         actions.push(Action {
                            kind: action,
                            pos: Cordinate { x: cord_x, y: y },
                            result: ActionResult::Move
                        });
                    }
                    if piece.color != self.color {
                        actions.push(Action {
                            kind: action,
                            pos: piece.pos,
                            result: ActionResult::Capture
                        });
                    }
                }
                else {
                    for cord_x in start.x..BOARD_WIDTH {
                        actions.push(Action {
                            kind: action,
                            pos: Cordinate { x: cord_x, y: y },
                            result: ActionResult::Move
                        });
                    }
                }
            },
            (PieceColor::Black, Direction::Left) |
            (PieceColor::White, Direction::Right) => {

                let closest_piece: Option<&Piece> = pcs.iter().filter(|p|
                    p.state == PieceState::Play &&
                    (p.pos.x <= start.x && p.pos.y == start.y)
                ).max_by_key(|p| p.pos.x);

                if closest_piece.is_some() {
                    let piece = closest_piece.unwrap();
                    for cord_x in (piece.pos.x..start.x).rev() {
                         actions.push(Action {
                            kind: action,
                            pos: Cordinate { x: cord_x, y: y },
                            result: ActionResult::Move
                        });
                    }
                    if piece.color != self.color {
                        actions.push(Action {
                            kind: action,
                            pos: piece.pos,
                            result: ActionResult::Capture
                        });
                    }
                }
                else {
                    for cord_x in (0..start.x).rev() {
                        actions.push(Action {
                            kind: action,
                            pos: Cordinate { x: cord_x, y: y },
                            result: ActionResult::Move
                        });
                    }
                }
            }
            (PieceColor::Black, Direction::Up) |
            (PieceColor::White, Direction::Down) => {

                let closest_piece: Option<&Piece> = pcs.iter().filter(|p|
                        p.state == PieceState::Play &&
                        (p.pos.x == start.x && p.pos.y >= start.y)
                    ).min_by_key(|p| p.pos.x);

                if closest_piece.is_some() {
                    let piece = closest_piece.unwrap();
                    for cord_y in start.y..piece.pos.y {
                         actions.push(Action {
                            kind: action,
                            pos: Cordinate { x: x, y: cord_y },
                            result: ActionResult::Move
                        });
                    }
                    if piece.color != self.color {
                        actions.push(Action {
                            kind: action,
                            pos: piece.pos,
                            result: ActionResult::Capture
                        });
                    }
                }
                else {
                    for cord_y in start.y..BOARD_HEIGHT {
                        actions.push(Action {
                            kind: action,
                            pos: Cordinate { x: x, y: cord_y },
                            result: ActionResult::Move
                        });
                    }
                }
            },
            (PieceColor::Black, Direction::Down) |
            (PieceColor::White, Direction::Up) => {

                let closest_piece: Option<&Piece> = pcs.iter().filter(|p|
                        p.state == PieceState::Play &&
                        (p.pos.x == start.x && p.pos.y >= start.y)
                    ).max_by_key(|p| p.pos.x);

                if closest_piece.is_some() {
                    let piece = closest_piece.unwrap();
                    for cord_y in (start.y..piece.pos.y).rev() {
                         actions.push(Action {
                            kind: action,
                            pos: Cordinate { x: x, y: cord_y },
                            result: ActionResult::Move
                        });
                    }
                    if piece.color != self.color {
                        actions.push(Action {
                            kind: action,
                            pos: piece.pos,
                            result: ActionResult::Capture
                        });
                    }
                }
                else {
                    for cord_y in (0..start.y).rev() {
                        actions.push(Action {
                            kind: action,
                            pos: Cordinate { x: x, y: cord_y },
                            result: ActionResult::Move
                        });
                    }
                }
            },
        }

        actions
    }

    fn get_spawn_tiles(&self, pcs: Vec<Piece>) -> Vec<Cordinate> {
        let duke: Option<&Piece> = pcs.iter().find(|&&p|
            p.state == PieceState::Play &&
            p.kind == PieceType::Duke &&
            p.color == self.color);

        let duke = duke.expect("There is no duke in play.");
        let mut tiles: Vec<Cordinate>;

        // Note: Color does not matter here. We use absolute cordinates.

        let check_n_add = |candidate: Cordinate| {
            if candidate.legal() &&
                pcs.iter().find(|&&p|
                    p.state == PieceState::Play &&
                    p.pos == candidate).is_none()
            {
                tiles.push(candidate.clone());
            }
        };

        // Check tile on right
        check_n_add(Cordinate{x: duke.pos.x + 1, y: duke.pos.y});
        // Check tile on left
        check_n_add(Cordinate{x: duke.pos.x - 1, y: duke.pos.y});
        // Check tile up
        check_n_add(Cordinate{x: duke.pos.x, y: duke.pos.y + 1});
        // Check tile down
        check_n_add(Cordinate{x: duke.pos.x, y: duke.pos.y - 1});

        tiles
    }

    pub fn get_actions(&self, pcs: Vec<Piece>) -> Vec<Action> {

        assert!(self.state != PieceState::Dead);

        let mut actions: Vec<Action>;

        // Special case for init state
        if self.state == PieceState::Init {
            let tiles = self.get_spawn_tiles(pcs);

            for tile in tiles {
                actions.push(Action {
                   kind: ActionType::Spawn,
                   pos: tile,
                   result: ActionResult::Move
                });
            }

            return actions
        }

        let avail_actions = if self.flipped {self.actions_back} else {self.actions_front};

        for action in avail_actions {
            match action.kind {
                ActionType::Spawn => { panic!("Illegal state"); },
                ActionType::Move => {
                    let cord = Cordinate{
                            x: selfpos.x + action.offset.x,
                            y: elfpos.x + action.offset.x};

                    if !cord.legal() {
                        continue;
                    }



                    let mut result: ActionResult;
                    let piece_in_way = pcs.find(|&&p| p.pos == cord);

                    if piece_in_way.is_some() {
                        if piece_in_way.unwrap().color == self.color {
                            continue;
                        }
                        result = ActionResult::Capture;
                    }
                    else {
                        result = ActionResult::Move;
                    }

                    actions.push(Action{
                        kind: ActionType::Move,
                        pos: tile,
                        result: ActionResult::Move
                    });
                },
                ActionType::Jump,
                ActionType::JumpSlide,
                ActionType::Slide,
                ActionType::Command,
                ActionType::Strike,
                ActionType::Dread,
                ActionType::Defense,
            }
        }
        actions
    }
}