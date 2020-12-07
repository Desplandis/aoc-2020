use std::collections::HashSet;
use nom::eof; // combinator
use nom::{char, complete, take_while_m_n}; // character
use nom::{many1, separated_list1}; // multi
use nom::terminated; // sequence
use nom::{do_parse, map, IResult};

fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic()
}

fn row(input: &str) -> IResult<&str, HashSet<char>> {
    map!(input, complete!(take_while_m_n!(1, 26, is_alphabetic)), |c| {
        c.chars().map(|c| c).collect()
    })
}

fn group(input: &str) -> IResult<&str, Vec<HashSet<char>>> {
    many1!(input, terminated!(row, char!('\n')))
}

fn document(input: &str) -> IResult<&str, Vec<Vec<HashSet<char>>>> {
    do_parse!{input,
        groups: separated_list1!(complete!(char!('\n')), group) >>
        eof!() >>
        (groups)
    }
}

fn main() {
    let input = include_str!("../input");
    let (_, output) = document(input).unwrap();
    let (any, all) = output.iter().fold((0, HashSet::new()), |(acnt, aset), g| {
        let aggr = g.iter().fold(HashSet::new(), |set, row| {
            set.union(&row).map(|c| *c).collect()
        });
        (acnt + aggr.iter().count(), aset.union(&aggr).map(|c| *c).collect())
    });
    println!("[1] Sum of questions which anyone answered \"yes\": {}", any);

    let all = output.iter().fold(0, |acc, g| {
        let aggr = g.iter().fold(all.clone(), |set, row| {
            set.intersection(&row).map(|c| *c).collect()
        });
        acc + aggr.iter().count()
    });
    println!("[2] Sum of questions which everyone answered \"yes\": {}", all);
}
