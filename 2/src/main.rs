extern crate nom;

use std::fs::File;
use std::io::{BufReader, BufRead};
use nom::{do_parse, map, tag, Err, IResult, Needed};
use nom::character::complete::{digit1, space1, alpha1};

struct Policy {
    lo: usize,
    hi: usize,
    ch: char
}

struct Line {
    policy: Policy,
    passwd: String
}

fn one_char<'a>(input: &'a str) -> IResult<&'a str, char> {
    if !input.is_empty() {
        Ok((&input[1..], input.chars().next().unwrap() as char))
    } else {
        Err(Err::Incomplete(Needed::new(1)))
    }
}

fn policy<'a>(input: &'a str) -> IResult<&'a str, Policy> {
    do_parse!(input,
        lo: map!(digit1, |i| i.parse::<usize>().unwrap()) >>
        tag!("-") >>
        hi: map!(digit1, |i| i.parse::<usize>().unwrap()) >>
        space1 >>
        ch: one_char >>
        (Policy { lo, hi, ch })
    )
}

fn password<'a>(input: &'a str) -> IResult<&'a str, Line> {
    do_parse!(input,
        policy: policy >>
        tag!(":") >>
        space1 >>
        password: alpha1 >>
        (Line { policy, passwd: String::from(password) })
    )
}

fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);
    let passwd = {
        let mut v = Vec::new();
        for l in reader.lines() {
            let input = l.unwrap();
            let (_, pass) = password(&input).unwrap();
            v.push(pass);
        };
        v
    };

    let valid1 = passwd.iter().filter(|l| {
        let nb = l.passwd.chars().filter(|c| c == &l.policy.ch).count();
        nb >= l.policy.lo && nb <= l.policy.hi
    }).count();
    let valid2 = passwd.iter().filter(|l| {
        let ch = l.policy.ch;
        (l.passwd.chars().nth(l.policy.lo - 1) == Some(ch))
            ^ (l.passwd.chars().nth(l.policy.hi - 1) == Some(ch))
    }).count();

    println!("[1] {} valid password(s)", valid1);
    println!("[2] {} valid password(s)", valid2);
}
