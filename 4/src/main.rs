extern crate nom;

use std::collections::HashMap;
use nom::alt; // branch
use nom::eof; // combinator
use nom::{char, complete, tag, take, take_while1}; // character
use nom::separated_list1; // multi
use nom::{do_parse, IResult};

fn value(input: &str) -> IResult<&str, &str> {
    take_while1!(input, |c: char| c.is_alphanumeric() || c == '#')
}

fn field(input: &str) -> IResult<&str, (&str, &str)> {
    do_parse!{&input,
        key: complete!(take!(3)) >>
        char!(':') >>
        value: value >>
        ((key, value))
    }
}

fn passport<'a>(input: &'a str) -> IResult<&'a str, HashMap<&str, &str>> {
    let sep = |i: &'a str| alt!(i, char!('\n') | char!(' '));
    let output = separated_list1!(input, sep, field);
    output.map(|(i, fields)| {
        (i, fields.iter().fold(HashMap::new(), |mut acc, p| {
            acc.insert(p.0, p.1);
            acc
        }))
    })
}

fn passports(input: &str) -> IResult<&str, Vec<HashMap<&str, &str>>> {
    let sep = |i| complete!(i, tag!("\n\n"));
    do_parse!{&input,
        passports: separated_list1!(sep, passport) >>
        char!('\n') >>
        eof!() >>
        (passports)
    }
}

fn main() {
    let input = include_str!("../input");
    let (_, output) = passports(input).unwrap();
    let valid = output.iter().filter(|p|
        p.contains_key("byr") && p.contains_key("iyr") &&
        p.contains_key("hgt") && p.contains_key("eyr") &&
        p.contains_key("hcl") && p.contains_key("ecl") &&
        p.contains_key("pid")
    ).count();
    println!("[1] {} valid passports", valid);
}
