// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::str;
use std::{process::exit, vec};

use app::game::board;
use serde::Deserialize;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![put_piece, opponent_put_piece])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Deserialize)]
struct GameInfo {
    player_board: Vec<bool>,
    opponent_board: Vec<bool>,
    x: u32,
    y: u32,
}

#[derive(serde::Deserialize)]

struct BoardInfo {
    player_board: Vec<bool>,
    opponent_board: Vec<bool>,
}

#[tauri::command]
fn put_piece(game_info: GameInfo) -> Vec<Vec<bool>> {
    let mut board = board::Board::new(game_info.player_board, game_info.opponent_board);
    let put = board.coordinates_to_bit(game_info.x, game_info.y);

    if board.is_end() {
        let result = board.get_result();
        println!("{}", result.0);
        println!("{}", result.1);
    }

    if !board.can_put(put) {
        return vec![
            board::make_normal_board(board.player_board),
            board::make_normal_board(board.opponent_board),
        ];
    }

    if board.is_pass() {
        return vec![
            board::make_normal_board(board.player_board),
            board::make_normal_board(board.opponent_board),
        ];
    }

    board.reverse(put);
    let new_player_board = board::make_normal_board(board.player_board);
    let new_opponent_board = board::make_normal_board(board.opponent_board);

    vec![new_player_board, new_opponent_board]
}

#[tauri::command]
fn opponent_put_piece(board_info: BoardInfo) -> Vec<Vec<bool>> {
    let mut board = board::Board::new(board_info.player_board, board_info.opponent_board);

    board.swap();
    if board.is_pass() {
        board.swap();
        return vec![
            board::make_normal_board(board.player_board),
            board::make_normal_board(board.opponent_board),
            board::make_normal_board(board.get_legal_board()),
        ];
    }

    board.random_put();
    board.swap();
    let legal_board = board::make_normal_board(board.get_legal_board());
    let new_player_board = board::make_normal_board(board.player_board);
    let new_opponent_board = board::make_normal_board(board.opponent_board);

    if board.is_end() {
        let result = board.get_result();
        println!("{}", result.0);
        println!("{}", result.1);
    }
    
    vec![new_player_board, new_opponent_board, legal_board]
}
