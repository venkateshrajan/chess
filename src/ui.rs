trait UI {
    fn display_board(&self, board: &Board);
    fn show_winner(&self, winner: bool);
    fn get_move(&self) -> Move;
}

struct consoleUI {}

impl UI for consoleUI {
    fn display_board(&self, board: &Board) {
    }

    fn show_winner(&self, winner: bool) {
    }

    fn get_move(&self) -> Move {
    }
}
