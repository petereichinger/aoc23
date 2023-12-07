use itertools::Itertools;
use rayon::prelude::*;

pub fn part(input: &str) -> i64 {
    let (seeds, mappings) = input.split_once("\n\n").unwrap();

    let seeds = seeds.trim_start_matches("seeds: ");

    let seeds = seeds
        .split(' ')
        .filter_map(|seed| seed.parse::<i64>().ok())
        .tuples()
        .into_iter()
        .map(|(begin, length)| (begin..(begin + length)))
        .flat_map(|range| range)
        .collect_vec();

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
        .par_iter()
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

    const INPUT: &str = include_str!("test_2.txt");
    #[test]
    fn it_works() {
        assert_eq!(part(INPUT), 46)
    }
}
