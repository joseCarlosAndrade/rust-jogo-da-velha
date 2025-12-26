use std::fmt::{Display};


pub enum Player {
    X,
    O,
}

impl Player {
    pub fn as_char(&self) -> char { // turns the enum into char
        match self {
            Self::X => 'X',
            Self::O => 'O',
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "{}", 'X'),
            Player::O => write!(f, "{}", 'O'),
        }
    }
}

pub struct Board {
    board : Vec<char>,
    current_player : Player
}


pub struct PlayingError {
    pub message: String,
    pub position : i32
}

pub enum GameState {
    XVICTORY,
    OVICTORY,
    GOING,
    DRAW
}
pub trait Game {
    
    fn print_board(&self);
    fn current_player(&self) -> char;
    fn play(&mut self, pos: i32) -> Result<(), PlayingError>;
    fn switch_players(&mut self);
    fn check_game_state(&self) -> GameState;
    
}

impl Board {
    pub fn new() -> Self { // new function should in general be outside the trait

        Self { 
            board: vec![
                ' ', ' ', ' ', 
                ' ', ' ', ' ', 
                ' ', ' ', ' '
                ], 
            current_player: Player::X 
        }
    }

    fn check_position(&self, pos : usize) -> bool {
        if self.board[pos-1] != ' ' {
            return false;
        }
        true
    }
}


impl Game for Board {
    fn print_board(&self) {
        println!("");
        for i in 0..3 {
            let p = i*3;

            let a1: char = if self.board[p] == ' ' { char::from_digit((p+1) as u32, 10).unwrap() } else {self.board[p]};
            let a2: char = if self.board[p+1] == ' ' { char::from_digit((p+2) as u32, 10).unwrap() } else {self.board[p+1]};
            let a3: char = if self.board[p+2] == ' ' { char::from_digit((p+3) as u32, 10).unwrap() } else {self.board[p+2]};


            println!("{} | {} | {} ", 
                a1, 
                a2 , 
                a3);
        }
    }

    fn current_player(&self) -> char{
        return self.current_player.as_char();
    }

    fn play(&mut self, pos: i32) -> Result<(), PlayingError> { // pos recv here: 1..9

        let pos_u : usize = pos.try_into().unwrap();

        if !check_pos_range(pos_u) {
            return Err(PlayingError { message: "position outside range".to_string(), position: pos });
        }

        if !self.check_position(pos_u) {
            return Err(PlayingError { message: "position already occupied".to_string(), position: pos });
        }

        self.board[pos_u-1] = self.current_player.as_char();

        Ok(())

    }

    fn switch_players(&mut self) {
        match self.current_player {
            Player::O => self.current_player = Player::X,
            Player::X => self.current_player = Player::O,
        }
    }

    fn check_game_state(&self) -> GameState {

        // check for row win
        if  self.check_row_win() ||
            self.check_column_win() ||
            self.check_diagonal_win() {
            
            return self.return_winner_current();
        }

        if self.check_draw() {
            return GameState::DRAW;
        }


        GameState::GOING
    }
}

fn check_pos_range(p : usize) -> bool {
    if p < 1 || p > 9 {
        return false
    }

    true
}

trait Checker {
    fn check_draw(&self) -> bool;
    fn check_row_win(&self) -> bool;
    fn check_column_win(&self) -> bool;
    fn check_diagonal_win(&self) -> bool;
    fn return_winner_current(&self) -> GameState; 
}

impl Checker for Board {
    fn check_draw(&self) -> bool {
        for i in 0..9 { // using reference so it dosent move ? (i think i could also clone it .clone())
            if self.board[i] == ' ' {
                
                return false;
            }
        }

        true
    }

    fn check_row_win(&self) -> bool {
        for i in 0..3 {
            let s = i*3;
            if self.board[s] == self.board[s+1] && self.board[s] == self.board[s+2] && self.board[s] != ' ' {
                return true;
            }
        }
        
        false
    }

    fn check_column_win(&self) -> bool {
        for i in 0..3 {
            if self.board[i] == self.board[i+3] && self.board[i] == self.board[i+6] && self.board[i] != ' ' {
                return true;
            }
        }
        
        false
    }

    fn check_diagonal_win(&self) -> bool {
        return 
            (self.board[0] == self.board[4] && self.board[0] == self.board[8] && self.board[0] != ' ') ||
            (self.board[2] == self.board[4] && self.board[2] == self.board[6] && self.board[2] != ' ');
    }

    fn return_winner_current(&self) -> GameState {
        match self.current_player {
            Player::O => GameState::OVICTORY,
            Player::X => GameState::XVICTORY,
        }
    }
}
