use crate::chess::models::{Color, File, PieceType, Rank, Square};

pub struct SlideIter {
    current: Square,
    dest: Square,
    step: (i8, i8),
}

impl SlideIter {
    // iterator includes all positions between src and dest, excluding src and dest
    pub fn new(src: Square, dest: Square) -> SlideIter {
        let step = (
            ((dest.0 as i8) - (src.0 as i8)).signum(),
            ((dest.1 as i8) - (src.1 as i8)).signum(),
        );
        if !is_rook_move(src, dest) && !is_bishop_move(src, dest) {
            panic!("SlideIter::new called with non-sliding move");
        }
        let current = pos_plus(src, step).unwrap();
        SlideIter {
            current,
            dest,
            step,
        }
    }
}

impl Iterator for SlideIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current != self.dest {
            let curr = self.current;
            self.current =
                pos_plus(self.current, self.step).expect("SlideIter::next: step out of bounds");
            Some(curr)
        } else {
            None
        }
    }
}

pub fn pos_plus(pos: Square, step: (i8, i8)) -> Option<Square> {
    let new_file = match File::from_i8(pos.0 as i8 + step.0) {
        Some(file) => file,
        None => return None,
    };
    let new_rank = match Rank::from_i8(pos.1 as i8 + step.1) {
        Some(rank) => rank,
        None => return None,
    };
    Some(Square(new_file, new_rank))
}

fn pos_minus(dest: Square, src: Square) -> (i8, i8) {
    (
        (dest.0 as i8) - (src.0 as i8),
        (dest.1 as i8) - (src.1 as i8),
    )
}

const ROOK_DIRS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

const ALL_DIRS: [(i8, i8); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

const KNIGHT_HOPS: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

pub struct KnightHopIter {
    current: usize,
    positions: [Option<Square>; 8],
}

impl KnightHopIter {
    pub fn new(origin: Square) -> KnightHopIter {
        let mut p_idx = 0;
        let mut positions = [None; 8];
        for hop in KNIGHT_HOPS.iter() {
            if let Some(pos) = pos_plus(origin, *hop) {
                positions[p_idx] = Some(pos);
                p_idx += 1;
            }
        }
        KnightHopIter {
            current: 0,
            positions,
        }
    }
}

impl Iterator for KnightHopIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 8 {
            let pos = self.positions[self.current];
            self.current += 1;
            pos
        } else {
            None
        }
    }
}

pub struct DirIter {
    current: usize,
    dirs: &'static [(i8, i8)],
}

impl DirIter {
    fn new(dirs: &'static [(i8, i8)]) -> DirIter {
        DirIter { current: 0, dirs }
    }

    pub fn rook() -> DirIter {
        DirIter::new(&ROOK_DIRS)
    }

    pub fn bishop() -> DirIter {
        DirIter::new(&BISHOP_DIRS)
    }

    pub fn all() -> DirIter {
        DirIter::new(&ALL_DIRS)
    }
}

impl Iterator for DirIter {
    type Item = (i8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.dirs.len() {
            let dir = self.dirs[self.current];
            self.current += 1;
            Some(dir)
        } else {
            None
        }
    }
}

pub struct RayIter {
    dir: (i8, i8),
    current: Square,
}

impl RayIter {
    pub fn new(base: Square, dir: (i8, i8)) -> RayIter {
        RayIter { dir, current: base }
    }
}

impl Iterator for RayIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        match pos_plus(self.current, self.dir) {
            Some(pos) => {
                self.current = pos;
                Some(pos)
            }
            None => None,
        }
    }
}

fn is_rook_move(src: Square, dest: Square) -> bool {
    let (x, y) = pos_minus(dest, src);
    x == 0 || y == 0
}

fn is_bishop_move(src: Square, dest: Square) -> bool {
    let (x, y) = pos_minus(dest, src);
    x.abs() == y.abs()
}

fn is_knight_move(src: Square, dest: Square) -> bool {
    let (x, y) = pos_minus(dest, src);
    (x.abs() == 2 && y.abs() == 1) || (x.abs() == 1 && y.abs() == 2)
}

fn is_king_move(src: Square, dest: Square) -> bool {
    let (x, y) = pos_minus(dest, src);
    x.abs() <= 1 && y.abs() <= 1
}

fn is_pawn_move(src: Square, dest: Square, player: Color) -> bool {
    let (x, y) = pos_minus(dest, src);
    match player {
        Color::White => (x.abs() <= 1 && y == 1) || (x == 0 && y == 2 && src.1 == Rank::_2),
        Color::Black => (x.abs() <= 1 && y == -1) || (x == 0 && y == -2 && src.1 == Rank::_7),
    }
}

pub fn is_move_pseudo_legal(src: Square, dest: Square, piece: PieceType, player: Color) -> bool {
    match piece {
        PieceType::Queen => is_rook_move(src, dest) || is_bishop_move(src, dest),
        PieceType::Rook => is_rook_move(src, dest),
        PieceType::Bishop => is_bishop_move(src, dest),
        PieceType::Knight => is_knight_move(src, dest),
        PieceType::King => is_king_move(src, dest),
        PieceType::Pawn => is_pawn_move(src, dest, player),
    }
}
