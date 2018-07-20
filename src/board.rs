pub enum CellType {
    Border,
    Food,
    Open,
    Player,
}

pub struct Cell {
    x: i32,
    y: i32,
    cell_type: CellType,
}

pub struct Board {
    width: i32,
    height: i32,
    grid: Vec<Cell>,
}

/// Initializes a board with provided dimensions and builds out the 
/// physical (written) form
///
/// For a board with 1D representation 5x5
/// [ 
///     Border, Border, Border, Border, Border,
///     Border, Open,   Open,   Open,   Border,
///     Border, Open,   Open,   Open,   Border,
///     Border, Open,   Open,   Open,   Border,
///     Border, Border, Border, Border, Border,
/// ]
///
/// Border indicies [0 1 2 3 4 5 9 10 14 15 19 20 21 22 23 24 ]
/// Playable / Open Indices [ 6 7 8 11 12 13 16 17 18 ]
impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        Board {
            width,
            height,
            grid: create_grid(width, height),
        }
    }

    fn create_grid(width: i32, height: i32) -> Vec<Cell> {
        let dimensions = width * height;
        let mut board_vec: Vec<Cell> = Vec::new();

        for item in 0..dimensions {

        }

        board_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board() {
        let width = 10;
        let height = 10;
        let board = Board::new(width, height);
        assert_eq!(board.width, width);
        assert_eq!(board.height, height);
    }
}
