#[derive(PartialEq, Debug)]
pub enum CellType {
    Food,
    Open,
    Player,
}

#[derive(PartialEq, Debug)]
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

impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        Board {
            width,
            height,
            grid: Board::create_grid(width, height),
        }
    }

    fn create_grid(width: i32, height: i32) -> Vec<Cell> {
        let dimensions = (width * height)as usize;
        let mut board_vec: Vec<Cell> = Vec::with_capacity(dimensions);

        for item in 0..dimensions {
            let i = item as i32;
            let cell = Cell {
                x: i % height,
                y: i / width,
                cell_type: CellType::Open,
            };

            board_vec.push(cell);
        }

        board_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board() {
        let width = 4;
        let height = 4;
        let grid: Vec<Cell> = Vec::with_capacity((width * height) as usize);
        let board = Board::new(width, height);
        let expected_grid = vec![
                Cell { x: 0, y: 0, cell_type: CellType::Open },
                Cell { x: 1, y: 0, cell_type: CellType::Open },
                Cell { x: 2, y: 0, cell_type: CellType::Open },
                Cell { x: 3, y: 0, cell_type: CellType::Open },
                Cell { x: 0, y: 1, cell_type: CellType::Open },
                Cell { x: 1, y: 1, cell_type: CellType::Open },
                Cell { x: 2, y: 1, cell_type: CellType::Open },
                Cell { x: 3, y: 1, cell_type: CellType::Open },
                Cell { x: 0, y: 2, cell_type: CellType::Open },
                Cell { x: 1, y: 2, cell_type: CellType::Open },
                Cell { x: 2, y: 2, cell_type: CellType::Open },
                Cell { x: 3, y: 2, cell_type: CellType::Open },
                Cell { x: 0, y: 3, cell_type: CellType::Open },
                Cell { x: 1, y: 3, cell_type: CellType::Open },
                Cell { x: 2, y: 3, cell_type: CellType::Open },
                Cell { x: 3, y: 3, cell_type: CellType::Open },
            ];

        assert_eq!(board.width, width);
        assert_eq!(board.height, height);
        assert_eq!(board.grid, expected_grid);
    }
}
