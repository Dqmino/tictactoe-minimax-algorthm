#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    X,
    O,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

pub type Board = [[Cell; 3]; 3];
pub type Move = (usize,usize);

const MAX_SCORE: i8 = 10;
const MIN_SCORE: i8 = -10;

use std::collections::HashMap;
use recursive::recursive;
#[recursive]
pub fn minimax(board: Board, depth: u8, is_maximizing: bool, current_player: Player) -> i8 {
    if is_game_over(board) {
        return evaluate(board).unwrap();
    }

    return if is_maximizing {
        let mut best_score = i8::MIN;
        for each_move in available_moves(board) {
            let new_board = make_move(board.clone(), each_move, current_player);
            let score = minimax(new_board, depth + 1, false, other_player(current_player));
            if score > best_score {
                best_score = score;
            }
        }
        best_score
    } else {
        let mut best_score = i8::MAX;
        for each_move in available_moves(board) {
            let new_board = make_move(board.clone(), each_move, other_player(current_player));
            let score = minimax(new_board, depth + 1, true, other_player(current_player));
            if score < best_score {
                best_score = score;
            }
        }
        best_score
    };
}

fn best_moves(board: Board, player: Player) -> HashMap<Move, i8> {
    let mut best_score: i8 = if player == Player::X { i8::MIN } else { i8::MAX };
    let mut moves: HashMap<Move, i8> = HashMap::new();

    for each_move in available_moves(board) {
        let new_board = make_move(board.clone(), each_move, player);
        let score = minimax(new_board, 0, player == Player::X, player);

        if (player == Player::X && score > best_score) || (player == Player::O && score < best_score) {
            best_score = score;
            moves.clear();
            moves.insert(each_move, score);
        } else if score == best_score {
            moves.insert(each_move, score);
        }
    }

    moves
}



fn available_moves(board: Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for row in 0..3 {
        for col in 0..3 {
            if board[row][col] == Cell::Empty {
                moves.push((row, col));
            }
        }
    }
    moves
}
fn make_move (mut board: Board, wanted_move: Move, player: Player) -> Board {
    board[wanted_move.0][wanted_move.1] = Cell::Occupied(player);
    board
}

fn evaluate(board: Board) -> Option<i8> {

    let check_line = |line: &[Cell]| -> Option<Player> {
        if let [Cell::Occupied(player), Cell::Occupied(player2), Cell::Occupied(player3)] = line {
            if player == player2 && player2 == player3 {
                return Some(*player);
            }
        }
        None
    };

    for i in 0..3 {
        if let Some(player) = check_line(&board[i]) {
            return if player == Player::X { Some(MAX_SCORE) } else { Some(MIN_SCORE) };
        }

        if let Some(player) = check_line(&[board[0][i], board[1][i], board[2][i]]) {
            return if player == Player::X { Some(MAX_SCORE) } else { Some(MIN_SCORE) };
        }
    }

    if let Some(player) = check_line(&[board[0][0], board[1][1], board[2][2]]) {
        return if player == Player::X { Some(MAX_SCORE) } else { Some(MIN_SCORE) };
    }
    if let Some(player) = check_line(&[board[0][2], board[1][1], board[2][0]]) {
        return if player == Player::X { Some(MAX_SCORE) } else { Some(MIN_SCORE) };
    }

    Some(0)

}

fn is_game_over(board: Board) -> bool {
    board.iter().all(|row| row.iter().all(|cell| *cell != Cell::Empty)) || evaluate(board).unwrap() != 0
}

fn other_player(player: Player) -> Player {
    match player {
        Player::X => Player::O,
        Player::O => Player::X,
    }
}

#[cfg(test)]
mod tests {
    use crate::Cell::{Empty, Occupied};
    use crate::Player::{O, X};
    use super::*;

    #[test]
    fn test_find_best_moves() {
        let board = [
            [Empty, Empty, Empty],
            [Occupied(X), Occupied(O), Empty],
            [Occupied(X), Empty, Empty]
            ];
        assert!(best_moves(board, O).iter().all(|(_, score)| *score == MAX_SCORE));
    }
}