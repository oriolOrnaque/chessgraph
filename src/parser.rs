use nom::{
    IResult,
    Parser,
    combinator::{
        recognize,
        map_res,
        opt,
    },
    sequence::{
        tuple,
        terminated,
    },
    branch::{
        alt,
    },
    multi::{
        many1,
    },
    character::complete::{
        digit1,
        char,
        multispace0,
        u8,
    },
};

use crate::model::{PlayerColor, PlayerMove};

fn parse_turn_index(input: &str) -> IResult<&str, usize> {
    map_res(
        terminated(
            digit1,
            tuple((
                char('.'),
                multispace0,
            ))
        )
        , |s: &str| s.parse::<usize>()
    ).parse(input)
}

fn parse_column(input: &str) -> IResult<&str, char> {
    alt((
        char('a'),
        char('b'),
        char('c'),
        char('d'),
        char('e'),
        char('f'),
        char('g'),
        char('h'),
    )).parse(input)
}

fn parse_row(input: &str) -> IResult<&str, u8> {
    u8.parse(input)
}

fn parse_piece(input: &str) -> IResult<&str, char> {
    alt((
        char('N'),
        char('K'),
        char('Q'),
        char('B'),
        char('R'),
    )).parse(input)
}

fn parse_position(input: &str) -> IResult<&str, &str> {
    terminated(
        recognize(
            tuple((
                opt(parse_piece),
                parse_column,
                opt(parse_row),
            )),
        ),
        multispace0
    ).parse(input)
}

fn parse_player_move(input:&str) -> IResult<&str, &str> {
    terminated(
        recognize(
            tuple((
                parse_position,
                opt(
                    tuple((
                        char('x'),
                        parse_position,
                    ))
                )
            ))
        ),
        multispace0
    ).parse(input)
}

fn parse_turn(input: &str) -> IResult<&str, Vec<PlayerMove>> {
    match terminated(
        tuple((
            parse_turn_index,
            parse_player_move,
            opt(parse_player_move),
        )),
        multispace0
    ).parse(input) {
        Ok((remaining_input, (
            turn_index,
            player1_move,
            player2_move,
        ))) => {
            let mut vec = Vec::<PlayerMove>::with_capacity(2);
            vec.push(PlayerMove::new(turn_index, PlayerColor::White, player1_move));
            if player2_move.is_some() {
                vec.push(PlayerMove::new(turn_index, PlayerColor::Black, player2_move.unwrap()));
            }
            Ok((
                remaining_input,
                vec,
            ))
        },
        Err(e) => Err(e),
    }
}

pub fn parse_turns(input: &str) -> IResult<&str, Vec<PlayerMove>> {
    match many1(parse_turn).parse(input) {
        Ok((remaining_input, nested_moves)) =>{
            Ok((
                remaining_input,
                nested_moves.into_iter().flatten().collect(),
            ))
        },
        Err(e) => Err(e),
    }
}