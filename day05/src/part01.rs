use itertools::Itertools;

pub fn part(input: &str) -> i64 {
    let (seeds, mappings) = input.split_once("\n\n").unwrap();

    let seeds = seeds.trim_start_matches("seeds: ");

    let seeds: Vec<_> = seeds
        .split(' ')
        .filter_map(|seed| seed.parse::<i64>().ok())
        .collect();

    part_with_seeds(&seeds, mappings)
}

pub fn part_with_seeds(seeds: &[i64], mappings: &str) -> i64 {
    let stages = mappings
        .split("\n\n")
        .map(|split| {
            let mappings = split
                .split('\n')
                .skip(1)
                .filter_map(|mapping| {
                    let (dest_range, source_range, length) = mapping
                        .split(' ')
                        .filter_map(|val| val.parse::<i64>().ok())
                        .collect_tuple()?;

                    Some(Mapping {
                        dest_range,
                        source_range,
                        length,
                    })
                })
                .collect_vec();

            mappings
        })
        .collect_vec();

    seeds
        .iter()
        .map(|seed| map_seed(*seed, &stages))
        .min()
        .unwrap_or(-1)
}

fn map_seed(seed: i64, stages: &Vec<Vec<Mapping>>) -> i64 {
    stages.iter().fold(seed, |acc, stage| {
        let mapping = stage.iter().find(
            |Mapping {
                 source_range,
                 length,
                 ..
             }| { acc >= *source_range && acc < (source_range + length) },
        );

        if let Some(Mapping {
            source_range,
            dest_range,
            ..
        }) = mapping
        {
            dest_range + (acc - source_range)
        } else {
            acc
        }
    })
}

struct Mapping {
    source_range: i64,
    dest_range: i64,
    length: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = include_str!("test.txt");
    #[test]
    fn it_works() {
        assert_eq!(part_with_seeds(&[79, 14, 55, 13], INPUT), 35)
    }

    #[rstest]
    #[case(79, 82)]
    #[case(14, 43)]
    #[case(55, 86)]
    #[case(13, 35)]
    fn test_cases(#[case] seed: i64, #[case] location: i64) {
        assert_eq!(part_with_seeds(&[seed], INPUT), location)
    }
}
