enum RPSChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
use RPSChoice::*;
impl RPSChoice {
    fn from_opponent_choice(choice: &char) -> RPSChoice {
        match choice {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => panic!("Invalid choice by opponent: {:?}", choice),
        }
    }

    fn from_player_choice(choice: &char) -> RPSChoice {
        match choice {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!("Invalid choice by player: {:?}", choice),
        }
    }
}

enum RoundWin {
    DRAW = 6,
    PLAYER = 8,
    OPPONENT = 1,
}
impl RoundWin {
    fn from_choices(player_choice: &RPSChoice, opponent_choice: &RPSChoice) -> RoundWin {
        match opponent_choice {
            Rock => match player_choice {
                Rock => RoundWin::DRAW,
                Paper => RoundWin::PLAYER,
                Scissors => RoundWin::OPPONENT,
            },
            Paper => match player_choice {
                Rock => RoundWin::OPPONENT,
                Paper => RoundWin::DRAW,
                Scissors => RoundWin::PLAYER,
            },
            Scissors => match player_choice {
                Rock => RoundWin::PLAYER,
                Paper => RoundWin::OPPONENT,
                Scissors => RoundWin::DRAW,
            },
        }
    }
}

pub struct Round {
    player_choice: RPSChoice,
    opponent_choice: RPSChoice,
    winner: RoundWin,
    points_for_player: usize,
}
impl Round {
    fn new(player_choice_char: &char, opponent_choice_char: &char) -> Round {
        let player_choice = RPSChoice::from_player_choice(player_choice_char);
        let opponent_choice = RPSChoice::from_opponent_choice(opponent_choice_char);
        let winner = RoundWin::from_choices(&player_choice, &opponent_choice);
        let points_for_player = winner as usize;

        return Round {
            player_choice,
            opponent_choice,
            winner,
            points_for_player,
        };
    }
}

pub fn parse_input(unparsed_input: String) -> Vec<Round> {
    let lines: Vec<&str> = unparsed_input.split("\n").collect();
    let rounds_as_char_vecs = lines
        .iter()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let rounds = rounds_as_char_vecs.iter().map(|round_as_char_vec| {
        Round::new(
            &round_as_char_vec[0].chars().collect::<Vec<char>>()[0],
            &round_as_char_vec[1].chars().collect::<Vec<char>>()[0],
        )
    });
    return rounds.collect();
}

pub fn solve(rounds: Vec<Round>) -> String {
    let mut total_points = 0;
    let _ = rounds
        .iter()
        .for_each(|round| total_points = total_points + round.points_for_player);
    return total_points.to_string();
}
