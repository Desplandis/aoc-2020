use nom::alt; // branch
use nom::eof; // combinator
use nom::{char, complete}; // character
use nom::{fold_many_m_n, many1}; // multi
use nom::terminated; // sequence
use nom::{do_parse, map, IResult};

fn bit_row(input: &str) -> IResult<&str, bool> {
    alt!(input,
        map!(char!('F'), |_| false) |
        map!(char!('B'), |_| true)
    )
}

fn bit_col(input: &str) -> IResult<&str, bool> {
    alt!(input,
        map!(char!('L'), |_| false) |
        map!(char!('R'), |_| true)
    )
}

fn row(input: &str) -> IResult<&str, u16> {
    fold_many_m_n!(input, 7, 7, bit_row, 0, |acc, bit|
        (acc << 1) | bit as u16
    )
}

fn col(input: &str) -> IResult<&str, u16> {
    fold_many_m_n!(input, 3, 3, bit_col, 0, |acc, bit|
        (acc << 1) | bit as u16
    )
}

fn pass(input: &str) -> IResult<&str, u16> {
    do_parse!{input,
        msb: row >>
        lsb: col >>
        (msb << 3 | lsb)
    }
}

fn main() {
    let input = include_str!("../input");
    let output = do_parse!{input,
        passes: many1!(terminated!(complete!(pass), char!('\n'))) >>
        eof!() >>
        (passes)
    };
    let passes = {
        let mut passes = output.unwrap().1;
        passes.sort();
        passes
    };

    println!("[1] Highest seat id: {:?}", passes.iter().max());

    let mut seat = None;
    for i in 1..(passes.len()) {
        let next = passes[i];
        if passes[i - 1] + 2 == next {
            seat = Some(next - 1);
            break;
        }
    }
    println!("[2] My seat: {:?}", seat);
}
