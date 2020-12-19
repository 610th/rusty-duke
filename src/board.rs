

const BOARD_WIDTH: usize = 6;
const BOARD_HEIGHT: usize = 6;

enum SquareState {
    NORMAL,
    DREAD,
    DEFENCE,
    COMMAND(&Piece)
}

struct Square {
    state: SquareState
    piece: Option<Piece>
}

struct Board {
    squares: [[mut Square; BOARD_WIDTH]; BOARD_HEIGHT]
}