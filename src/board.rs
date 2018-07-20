/// The upper and lower boundary char.
const HORZ_BOUNDARY: &'static str = "─";
/// The left and right boundary char.
const VERT_BOUNDARY: &'static str = "│";

/// The top-left corner
const TOP_LEFT_CORNER: &'static str = "┌";
/// The top-right corner
const TOP_RIGHT_CORNER: &'static str = "┐";
/// The bottom-left corner
const BOTTOM_LEFT_CORNER: &'static str = "└";
/// The bottom-right corner
const BOTTOM_RIGHT_CORNER: &'static str = "┘";

pub struct Board {
    width: i32,
    height: i32
}

/// Initializes a board with provided dimensions and builds out the 
/// physical (written) form
impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        Board {
            width,
            height,
        }
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
