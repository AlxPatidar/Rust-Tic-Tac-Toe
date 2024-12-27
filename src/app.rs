use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

const BOARD_SIZE: usize = 3;
const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
#[derive(Debug, Default)]
pub struct App {
    row: usize,
    col: usize,
    current_player: char,
    exit: bool,
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
    message: String,
}
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    // updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.set_row(self.row.saturating_sub(1)),
            KeyCode::Char('r') => {
                for row in 0..BOARD_SIZE {
                    for col in 0..BOARD_SIZE {
                        self.board[row][col] = ' '
                    }
                }
                self.set_row(0);
                self.set_col(0);
                self.set_current_player(PLAYER_X);
                self.set_message("Let's Start Again.");
            }
            KeyCode::Char('h') => self.set_row(self.row.saturating_sub(1)),
            KeyCode::Down => self.set_row((self.row + 1).min(2)),
            KeyCode::Char('k') => self.set_row((self.row + 1).min(2)),
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.set_col(self.col.saturating_sub(1)),
            KeyCode::Char('j') => self.set_col(self.col.saturating_sub(1)),
            KeyCode::Right => self.set_col((self.col + 1).min(2)),
            KeyCode::Char('l') => self.set_col((self.col + 1).min(2)),
            KeyCode::Enter => self.allow_click(),
            KeyCode::Char('x') => self.allow_click(),
            KeyCode::Char('o') => self.allow_click(),
            _ => {}
        }
    }
    fn allow_click(&mut self) {
        if vec!['X', 'O'].contains(&self.board[self.row][self.col]) {
            self.set_message("Spot already occupied");
        } else {
            if self.current_player == PLAYER_X {
                self.set_message(&format!("{}'s turn", PLAYER_X));
                self.set_current_player(PLAYER_O)
            } else {
                self.set_message(&format!("{}'s turn", PLAYER_O));
                self.set_current_player(PLAYER_X)
            }
            self.set_player_choice()
        }
    }
    fn check_winner(&self, current_player: char) -> bool {
        let board = self.board;
        for row in 0..BOARD_SIZE {
            if vec!['X', 'O'].contains(&board[row][0])
                && board[row][0] == current_player
                && board[row][1] == current_player
                && board[row][2] == current_player
            {
                return true;
            }
        }
        // check all column are same
        for col in 0..BOARD_SIZE {
            if vec!['X', 'O'].contains(&board[0][col])
                && board[0][col] == current_player
                && board[1][col] == current_player
                && board[2][col] == current_player
            {
                return true;
            }
        }

        if (vec!['X', 'O'].contains(&board[0][0])
            && board[0][0] == current_player
            && board[1][1] == current_player
            && board[2][2] == current_player)
            || (vec!['X', 'O'].contains(&board[0][2])
                && board[0][2] == current_player
                && board[1][1] == current_player
                && board[2][0] == current_player)
        {
            return true;
        }
        return false;
    }
    fn set_col(&mut self, col: usize) {
        self.col = col;
    }
    fn set_current_player(&mut self, player: char) {
        self.current_player = player;
    }
    fn set_row(&mut self, row: usize) {
        self.row = row;
    }
    fn exit(&mut self) {
        self.exit = true;
    }
    fn set_player_choice(&mut self) {
        self.board[self.row][self.col] = self.current_player;
        if self.check_winner(self.current_player) {
            self.set_message(&format!("{}'s win the match", self.current_player));
        }
    }
    fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }
}
impl Widget for &App {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Move ".into(),
            "<Arrow/h/j/k/l>".blue().bold(),
            " Select ".into(),
            "<Enter>/x/o".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let mut grid = vec![];
        for row in 0..BOARD_SIZE {
            let mut row_cells = vec![];
            for col in 0..BOARD_SIZE {
                let selected = match self.board[row][col] {
                    'X' => " X ",
                    'O' => " O ",
                    _ => {
                        if col == self.col && row == self.row {
                            "__"
                        } else {
                            " "
                        }
                    }
                };
                let block = Block::default()
                    //.padding(Padding::new(5, 10, 1, 2))
                    .borders(Borders::ALL);
                let paragraph = Paragraph::new(selected.to_string().light_cyan())
                    .centered()
                    .block(block);
                row_cells.push(paragraph);
            }
            grid.push(row_cells);
        }
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let x_pos = (j as u16) * 10; // Adjust the width of each block
                let y_pos = (i as u16) * 4; // Adjust the height of each block

                let cell_area = Rect::new(x_pos, y_pos, 10, 4);
                cell.render(cell_area, buf);
            }
        }
        let block = Block::default()
            .title_bottom(instructions.clone().centered())
            .borders(Borders::ALL);
        Paragraph::new(self.message.clone().light_cyan())
            .centered()
            .block(block)
            .render(Rect::new(0, 12, 50, 4), buf);
    }
}
