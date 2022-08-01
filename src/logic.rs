// use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use log::info;
use std::panic;

use crate::{Battlesnake, Board, Coord, Game};

pub fn get_info() -> Value {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "",
        "color": "#228866",
        "head": "trans-rights-scarf",
        "tail": "pixel",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} END", game.id);
}

fn scores(source: &Coord, target: &Coord) -> (i32, i32) {
    // scores are the distance between the two points
    let x_score = target.x as i32 - source.x as i32;
    let y_score = target.y as i32 - source.y as i32;
    (x_score, y_score)
}

fn determine_score_directions(
    source: &Coord,
    target: &Coord,
    board: &Board,
    moves: &mut HashMap<&str, i32>,
    score_operation: &dyn Fn(&mut i32, i32),
) {
    let (x_score, y_score) = scores(source, target);

    if x_score < 0 {
        //left
        moves
            .entry("left")
            .and_modify(|e| score_operation(e, board.width as i32 - x_score.abs()));
    }
    if x_score > 0 {
        //right
        moves
            .entry("right")
            .and_modify(|e| score_operation(e, board.width as i32 - x_score.abs()));
    }

    if y_score < 0 {
        //down
        moves
            .entry("down")
            .and_modify(|e| score_operation(e, board.height as i32 - y_score.abs()));
    }
    if y_score > 0 {
        //up
        moves
            .entry("up")
            .and_modify(|e| score_operation(e, board.height as i32 - y_score.abs()));
    }
}

pub fn get_move(game: &Game, _turn: &u32, _board: &Board, my: &Battlesnake) -> &'static str {
    let add = |x: &mut i32, y: i32| *x += y + y;
    let sub = |x: &mut i32, y: i32| *x -= y;
    let mut possible_moves: HashMap<_, _> = vec![
        ("up", 0_i32),
        ("down", 0_i32),
        ("left", 0_i32),
        ("right", 0_i32),
    ]
    .into_iter()
    .collect();
    let my_head = &my.head;
    info!("head position: {:?}", my_head);

    // Step 1 - score walls

    determine_score_directions(
        my_head,
        &Coord { x: 0, y: my_head.y },
        _board,
        &mut possible_moves,
        &sub,
    );
    determine_score_directions(
        my_head,
        &Coord {
            x: _board.width,
            y: my_head.y,
        },
        _board,
        &mut possible_moves,
        &sub,
    );
    determine_score_directions(
        my_head,
        &Coord { x: my_head.x, y: 0 },
        _board,
        &mut possible_moves,
        &sub,
    );
    determine_score_directions(
        my_head,
        &Coord {
            x: my_head.x,
            y: _board.height,
        },
        _board,
        &mut possible_moves,
        &sub,
    );
    info!("wall score: {:?}", possible_moves);

    // score snakes.
    for snake in &_board.snakes {
        for point in &snake.body {
            determine_score_directions(my_head, &point, _board, &mut possible_moves, &sub);
        }
    }
    info!("competition score: {:?}", possible_moves);

    // Find food.
    for point in &_board.food {
        determine_score_directions(my_head, &point, _board, &mut possible_moves, &add);
    }
    info!("food score: {:?}", possible_moves);

    // remove dead moves - don't let it hit anything
    if my_head.x == 0 {
        possible_moves.remove("left");
    }
    if my_head.x == _board.width - 1 {
        possible_moves.remove("right");
    }
    if my_head.y == 0 {
        possible_moves.remove("down");
    }
    if my_head.y == _board.height - 1 {
        possible_moves.remove("up");
    }
    for snake in &_board.snakes {
        for point in &snake.body {
            let (x, y) = scores(my_head, point);
            if x == 1 && y == 0 {
                possible_moves.remove("right");
            }
            if x == -1 && y == 0 {
                possible_moves.remove("left");
            }
            if y == 1 && x == 0 {
                possible_moves.remove("up");
            }
            if y == -1 && x == 0 {
                possible_moves.remove("down");
            }
        }
    }

    // Finally, choose a move from the available safe moves.
    info!("removed dead moves: {:?}", possible_moves);
    let chosen = panic::catch_unwind(|| possible_moves.into_iter().max_by_key(|i| i.1).unwrap());

    // match chosen {
    //     Ok(c) => c.0,
    //     Err(e) => println!("{:?}", e),
    // }
    if chosen.is_err() {
        info!("no moves, we will lose!");
        return "AHHHHH";
    }

    let (chosen_direction, chosen_score) = chosen.unwrap();

    info!(
        "{} MOVE {}, SCORE {}",
        game.id, chosen_direction, chosen_score
    );

    chosen_direction
}
