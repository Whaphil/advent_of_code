use phf::phf_map;

enum RoundWin {
    DRAW,
    PLAYER,
    OPPONENT,
}

struct RPSChoice {
    opponent_letter: char,
    player_letter: char,
    points_for_win: usize,
}

const POINTS_FOR_WIN: usize = 8;
const POINTS_FOR_DRAW: usize = 6;
const POINTS_FOR_LOSS: usize = 1;

const Choices: phf::Map<&str, RPSChoice> = phf_map! {
    "ROCK" => RPSChoice{
        opponent_letter:'A',
        player_letter:'X',
        points_for_win:1
    },
    "PAPER" => RPSChoice{
        opponent_letter:'B',
        player_letter:'Y',
        points_for_win:2
    },
    "SCISSORS" => RPSChoice{
        opponent_letter:'C',
        player_letter:'Z',
        points_for_win:3
    }
};

struct Round {
    player_choice: RPSChoice,
    oppenent_choice: RPSChoice,
    winner: RoundWin,
    points_for_player: usize,
}

impl Round {
    fn new(player_choice: &str, opponent_choice: &str) -> Round {
        let parsed_player_choice = "";
        return Round {
            player_choice: Choices.get("ROCK").unwrap(),
            oppenent_choice: Choices.get("ROCK").unwrap(),
        };
    }
}

pub fn parse_input(unparsed_input: String) -> String {
    return unparsed_input;
}

pub fn solve(formatted_input: String) -> String {
    return formatted_input;
}
