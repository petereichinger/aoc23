pub fn part01(input: &str) -> i64 {
    let games: Vec<_> = input.lines().map(line_to_game).collect();

    let sum: i64 = games
        .iter()
        .filter(|g| {
            g.rounds
                .iter()
                .all(|Round { red, green, blue }| red <= &12 && green <= &13 && blue <= &14)
        })
        .map(|g| g.id)
        .sum();

    sum
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
    use super::*;

    #[test]
    fn it_works() {
        let test = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part01(test), 8)
    }
}
