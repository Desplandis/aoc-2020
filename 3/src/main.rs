extern crate nom;

use nom::{do_parse, alt, eof, many1, map, separated_list1, IResult};
use nom::character::complete::{char, line_ending};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile { Empty, Tree }

struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
}

// https://www.youtube.com/watch?v=_sG5YwPtetk
struct Slide<'a> {
    map: &'a Map,
    cur: (usize, usize),
    slope: (usize, usize),
}


impl Map {
    fn slide(&self, right: usize, bottom: usize) -> Slide {
        Slide {
            map: &self,
            cur: (0, 0),
            slope: (right, bottom)
        }
    }
}

impl<'a> Iterator for Slide<'a> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let y = self.cur.1 + self.slope.1;
        if y >= self.map.tiles.len() { None }
        else {
            let x = (self.cur.0 + self.slope.0) % self.map.width;
            self.cur = (x, y);
            Some(self.map.tiles[y][x])
        }
    }
}


fn tile(input: &[u8]) -> IResult<&[u8], Tile> { // complete
    alt!(input,
        map!(char('.'), |_| Tile::Empty) |
        map!(char('#'), |_| Tile::Tree)
    )
}

fn row<'a>(input: &'a[u8]) -> IResult<&'a[u8], Vec<Tile>> {
    many1!(input, tile)
}

fn map<'a>(input: &'a[u8]) -> IResult<&'a[u8], Map> {
    map!(input, separated_list1!(line_ending, row), |tiles| {
        let width = tiles[0].len(); // Lazyness...
        Map { tiles, width }
    })
}


fn main() {
    let bytes = include_bytes!("input");
    let result = do_parse!{&bytes,
        map: map >>
        line_ending >>
        eof!() >>
        (map)
    };

    if let Ok((_, map)) = result {
        let filter = |t: &Tile| *t == Tile::Tree;
        let t11 = map.slide(1, 1).filter(filter).count();
        let t31 = map.slide(3, 1).filter(filter).count();
        let t51 = map.slide(5, 1).filter(filter).count();
        let t71 = map.slide(7, 1).filter(filter).count();
        let t12 = map.slide(1, 2).filter(filter).count();
        println!("[1] OUCH! Encountered {} trees", t31);
        println!("[2] {}", t11 * t31 * t51 * t71 * t12);
    }
}
