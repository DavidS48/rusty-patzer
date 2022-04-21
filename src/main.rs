
#[derive(Copy, Clone, PartialEq)]
enum PieceName {
    K,
    Q,
    R,
    B,
    N,
    P,
}

#[derive(Copy, Clone, PartialEq)]
enum Colour {
    W,
    B,
}

#[derive(Copy, Clone, PartialEq)]
struct Piece {
    colour: Colour,
    name: PieceName,
}

trait Move {
    fn update_square(&self, old_pos: &Board, new_pos: Board, x: usize, y: usize) -> Board;
    fn updated_castling_rights(&self, old_pos: &Board, old_castling_rights: &CastlingRights) -> CastlingRights;
}

struct NormalMove {
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
}

impl Move for NormalMove {
    fn update_square(&self, &old_pos: &Board, mut new_pos: Board, x: usize, y: usize) -> Board {
        if x == self.from_x && y == self.from_y{
            new_pos[y][x] = None
        } else if x == self.to_x && y == self.to_y {
            new_pos[y][x] = old_pos[self.from_y][self.from_x]
        } else {
            new_pos[y][x] = old_pos[y][x]
        }
        new_pos
    }

    fn updated_castling_rights(&self, old_pos: &Board, old_castling_rights: &CastlingRights) -> CastlingRights {
        let piece = old_pos[self.from_y][self.from_x];
        match piece {
            Some(Piece { colour, name }) => {
                if name == PieceName::K {
                    if colour == Colour::W {
                        return CastlingRights { w_castle_ks: false, w_castle_qs: false, ..*old_castling_rights };
                    } else {
                        return CastlingRights { b_castle_ks: false, b_castle_qs: false, ..*old_castling_rights };
                    }
                }
            }
            _ => ()
        }
        return CastlingRights { ..*old_castling_rights }
    }
}

struct EnPassant {
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    captured_x: usize,
    captured_y: usize,
}

impl Move for EnPassant {
    fn update_square(&self, &old_pos: &Board, mut new_pos: Board, x: usize, y: usize) -> Board {
        if x == self.from_x && y == self.from_y{
            new_pos[y][x] = None
        } else if x == self.captured_x && y == self.captured_y {
            new_pos[y][x] = None
        } else if x == self.to_x && y == self.to_y {
            new_pos[y][x] = old_pos[self.from_y][self.from_x]
        } else {
            new_pos[y][x] = old_pos[y][x]
        }
        new_pos
    }

    fn updated_castling_rights(&self, old_pos: &Board, old_castling_rights: &CastlingRights) -> CastlingRights {
        return CastlingRights { ..*old_castling_rights };
    }
}


/*
struct Castle {
    rook_from_x: usize,
    rook_from_y: usize,
    king_from_x: usize,
    king_from_y: usize,
    rook_from_x: usize,
    rook_from_y: usize,
    king_from_x: usize,
    king_from_y: usize,
}

struct Promote {
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    piece: Piece,
}


*/
type Square = Option<Piece>;

type Board = [[Square; 8]; 8];


fn empty_board() -> Board {
    let board = [[ None; 8]; 8];
    board
}



#[derive(Copy, Clone)]
struct CastlingRights {
    w_castle_ks: bool,
    w_castle_qs: bool,
    b_castle_ks: bool,
    b_castle_qs: bool,
}

impl CastlingRights {
    fn initial() -> Self {
        Self { w_castle_ks: true, w_castle_qs: true, b_castle_ks: true, b_castle_qs: true }
    }
}

struct Position {
    board: Board,
    castling_rights: CastlingRights,
}


impl Position {
    fn make_move(&self, move_xy: &impl Move) -> Position {
        let mut new_board = empty_board();
        for y in 0..8 {
            for x in 0..8{
                new_board = move_xy.update_square(&self.board, new_board, x, y)
            }
        }
        Position {
            board: new_board,
            castling_rights: move_xy.updated_castling_rights(&self.board, &self.castling_rights)
        }
    }

    fn print(&self) {
        for rank in self.board.iter().rev() {
            let mut rank_out = String::new();
            for square in rank {
                rank_out.push(get_square_rep(&square))
            }
            println!("{}", rank_out)
        }
    }

    fn from_fen(fen: &str) -> Self {
        let mut board = empty_board();
        let mut x: usize = 0;
        let mut y: usize = 7;
        for c in fen.chars(){
            let c_d = c.to_digit(10);
            match c_d {
                Some(c_d) => x += c_d as usize,
                None => ()
            }
            if c.is_alphabetic() {
                let colour: Colour;
                if c.is_uppercase() {
                    colour = Colour::W;
                } else {
                    colour = Colour::B;
                }
                let c_l = c.to_lowercase().collect::<String>();
                let name: PieceName;
                if c_l == "k" {
                    name = PieceName::K;
                } else if c_l == "q" {
                    name = PieceName::Q;
                } else if c_l == "r" {
                    name = PieceName::R;
                } else if c_l == "b" {
                    name = PieceName::B;
                } else if c_l == "n" {
                    name = PieceName::N;
                } else if c_l == "p" {
                    name = PieceName::P;
                }
                else {
                    println!("Unrecognized piece!");
                    break;
                }
                board[y][x] = Some(Piece { colour: colour, name: name});
                x += 1;
            }
            if c == '/' {
                x = 0;
                y -= 1;
            }
            if x > 8 {
                println!("Went over the end of a line!");
                break;
            }
        }
        Self { board: board, castling_rights: CastlingRights::initial() }
    }

    fn start_pos() -> Self {
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    }

}


fn get_piece_rep(name: &PieceName, colour: &Colour) -> char {
    match colour {
        Colour::W =>
            match name {
                PieceName::K => 'K',
                PieceName::Q => 'Q',
                PieceName::R => 'R',
                PieceName::B => 'B',
                PieceName::N => 'N',
                PieceName::P => 'P',
            },
        Colour::B =>
            match name {
                PieceName::K => 'k',
                PieceName::Q => 'q',
                PieceName::R => 'r',
                PieceName::B => 'b',
                PieceName::N => 'n',
                PieceName::P => 'p',
            }
    }
}

fn get_square_rep(square: &Square) -> char{
    match square {
        None => ' ',
        Some(Piece { name, colour }) => get_piece_rep(&name, &colour)
    }
}

fn main() {
    let pos = Position::start_pos();
    let mut updated = pos.make_move(&NormalMove { from_x: 5, from_y: 1, to_x: 5, to_y: 3 });
    updated = updated.make_move(&NormalMove { from_x: 5, from_y: 6, to_x: 5, to_y: 5 });
    updated = updated.make_move(&NormalMove { from_x: 5, from_y: 3, to_x: 5, to_y: 4 });
    updated = updated.make_move(&NormalMove { from_x: 6, from_y: 6, to_x: 6, to_y: 4 });
    updated = updated.make_move(&EnPassant { from_x: 5, from_y: 4, to_x: 6, to_y: 5, captured_x: 6, captured_y: 4 });

    updated.print();
}
