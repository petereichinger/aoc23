pub fn part02(input: &str) -> i64 {
    let games: Vec<_> = input.lines().map(line_to_game).collect();

    let sum: u32 = games
        .iter()
        .map(|g| {
            g.rounds.iter().fold(Round::default(), |acc, r| Round {
                red: acc.red.max(r.red),
                green: acc.green.max(r.green),
                blue: acc.blue.max(r.blue),
            })
        })
        .map(|min_round| min_round.red * min_round.green * min_round.blue)
        .sum();

    sum as i64
}

#[derive(Debug, Default, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Default, Clone)]
struct Game {
    id: i64,
    rounds: Vec<Round>,
}

fn line_to_game(line: &str) -> Game {
    let (game, rounds) = line.split_once(": ").unwrap();
    let (_, id) = game.split_once(' ').unwrap();
    let id = id.parse().unwrap();

    let rounds = rounds.split("; ").map(parse_round).collect();

    Game { id, rounds }
}

fn parse_round(round_str: &str) -> Round {
    let mut round: Round = Round::default();

    round_str.split(", ").for_each(|take| {
        let (number, colour) = take.split_once(' ').unwrap();

        match colour {
            "red" => round.red = number.parse().unwrap(),
            "blue" => round.blue = number.parse().unwrap(),
            "green" => round.green = number.parse().unwrap(),
            other => panic!("unknown color {}", other),
        }
    });
    round
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn it_works() {
        let test = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part02(test), 2286)
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]

    fn games_test(#[case] game: &str, #[case] result: i64) {
        assert_eq!(part02(game), result)
    }
}
