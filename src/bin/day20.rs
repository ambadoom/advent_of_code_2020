use advent2020::input;

use std::fmt;
use std::collections::HashMap;

use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Side {
    Top,
    Bot,
    Left,
    Right,
}
use Side::*;

impl Side {
    fn opp(self) -> Self {
        match self {
            Top => Bot,
            Bot => Top,
            Left => Right,
            Right => Left,
        }
    }

    fn rotate(self) -> Self {
        match self {
            Top => Right,
            Right => Bot,
            Bot => Left,
            Left => Top,
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Edge(u16);

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>10b}", self.0)
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>10b}", self.0)
    }
}

impl Edge {
    fn flip(self) -> Self {
        let mut out = 0;
        for i in 0..10 {
            if self.0 & (1 << i) > 0 {
                out |= 1 << (9-i);
            }
        }
        Edge(out)
    }

    fn primary(self) -> Self {
        let flip = self.flip();
        if flip.0 > self.0 {
            flip
        } else {
            self
        }
    }
}

#[derive(Clone)]
struct Tile {
    id: usize,
    data: [u16; 10],
}

impl Tile {
    fn parse<'a, I: Iterator<Item=&'a str>>(input: &mut I) -> Option<Tile> {
        let re_name = Regex::new(r"Tile (\d+):").unwrap();


        let namestr = input.next()?;
        let id = if let Some(caps) = re_name.captures(namestr) {
            caps[1].parse().unwrap()
        } else {
            panic!("Failed parse")
        };

        let mut data = [0; 10];
        for i in 0..10 {
            let mut row = 0;
            let line = input.next()?;
            for (j,c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1 << (9-j);
                }
            }
            data[i] = row;
        }

        Some(Tile { id, data })
    }

    fn edge(&self, s: Side) -> Edge {
        match s {
            Top => { Edge(self.data[0]) },
            Bot => { Edge(self.data[9]) },
            Left => { 
                let mut out = 0;
                for (i, n) in self.data.iter().enumerate() {
                    if n & (1 << 9) > 0 {
                        out |= 1 << (9-i);
                    }
                }
                Edge(out)
            },
            Right => { 
                let mut out = 0;
                for (i, n) in self.data.iter().enumerate() {
                    if n & 1 > 0 {
                        out |= 1 << (9-i);
                    }
                }
                Edge(out)
            },
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.data[y] & (1 << (9-x)) > 0
    }

    fn rearrange<F: Fn((usize, usize)) -> (usize, usize)>(&mut self, f: F) {
        let mut out = [0;10];
        for i in 0..10 {
            for j in 0..10 {
                let bit = self.data [j] & (1 << i) > 0;
                if bit {
                    let (x,y) = f((i,j));
                    out[y] |= 1 << x; 
                }
            }
        }

        self.data = out;
    }

    fn rotate(&mut self) {
        self.rearrange(|(x,y)| (y, 9-x));
    }

    fn vflip(&mut self) {
        self.rearrange(|(x,y)| (x, 9-y));
    }

    fn hflip(&mut self) {
        self.rearrange(|(x,y)| (9-x, y));
    }

    /// Rotate and flip so that e1 is on s1 and e2 is on s2
    fn rotate_so(&self, e1: Edge, s1: Side, e2: Edge, s2: Side) -> Option<Self> {
        let e1_prim = e1.primary();
        let e2_prim = e2.primary();
        let mut e1_current = None;
        let mut e2_current = None;

        for s in [Left, Right, Top, Bot].iter() {
            let prim = self.edge(*s).primary();
            if e1_prim == prim { e1_current = Some(s); };
            if e2_prim == prim { e2_current = Some(s); };
        }

        let mut e1_current = *e1_current?;
        let mut e2_current = *e2_current?;

        let mut out = self.clone();

        while e1_current != s1 {
            out.rotate();
            e1_current = e1_current.rotate();
            e2_current = e2_current.rotate();
            assert_eq!(out.edge(e1_current).primary(), e1.primary());
        }
        assert_eq!(out.edge(s1).primary(), e1.primary());

        if e2_current != s2 {
            if e2_current == s2.opp() {
                match s2 {
                    Bot | Top => out.vflip(),
                    Left | Right => out.hflip(),
                }
            } else {
                return None;
            }
        }

        assert_eq!(out.edge(s1).primary(), e1.primary());
        assert_eq!(out.edge(s2).primary(), e2.primary());

        Some(out)
    }
}

fn find<'a>(edges: &HashMap<Edge, Vec<usize>>, tiles: &'a HashMap<usize, Tile>, tile: &Tile, side: Side) -> &'a Tile {
    let edge = tile.edge(side).primary();
    let id = edges[&edge]
        .iter()
        .filter(|id| **id != tile.id)
        .next()
        .unwrap();
    &tiles[id]
}

fn find_sea_monsters(image: &Vec<Vec<bool>>, monster: &Vec<Vec<bool>>) -> (usize, usize) {
    let mut monsters_found = 0;
    let mut image_ = image.clone();
    for y in 0..=(image.len() - monster.len()) {
        for x in 0..=(image[0].len() - monster[0].len()) {
            let mut found = true;
            for i in 0..monster.len() {
                for j in 0..monster[0].len() {
                    if monster[i][j] && !image[y+i][x+j] {
                        found = false;
                    }
                }
            }
            if found {
                monsters_found += 1;
                for i in 0..monster.len() {
                    for j in 0..monster[0].len() {
                        if monster[i][j] {
                            image_[y+i][x+j] = false;
                        }
                    }
                }
            }
        }
    }
    let roughness = image_
        .iter()
        .map(|r| r.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>())
        .sum();
    (monsters_found, roughness)
}

fn main() {
    let input = input::read_all(20);

    let mut lines = input.lines();
    let mut tiles = HashMap::new();
    while let Some(tile) = Tile::parse(&mut lines) {
        tiles.insert(tile.id, tile);
        let _ = lines.next(); // Skip empty line between tiles
    }

    let mut edges = HashMap::new();
    for (_id, tile) in tiles.iter() {
        for side in [Left, Right, Top, Bot].iter() {
            let edge = tile.edge(*side).primary();
            let entry = edges.entry(edge).or_insert(Vec::new());
            entry.push(tile.id);
        }
    }

    let mut border_tiles = HashMap::new();
    for (edge, tiles) in edges.iter() {
        if tiles.len() == 1 {
            let c = border_tiles.entry(tiles[0]).or_insert(0);
            *c += 1;
        }
    }

    println!("Part 1");
    let corners: Vec<usize> = border_tiles
        .iter()
        .filter(|(_, count)| **count == 2)
        .map(|(id, _)| *id)
        .collect(); 

    println!("{}", corners.iter().product::<usize>());

    println!("Part 2");
    let mut img: Vec<Vec<Tile>> = Vec::new();
    for i in 0..12 {
        let mut row: Vec<Tile> = Vec::new();
        if i == 0 {
            // Start with a corner
            let tile = &tiles[&corners[0]];

            let mut borders = [Left, Right, Top, Bot]
                .iter()
                .map(|s| tile.edge(*s))
                .filter(|e| edges[&e.primary()].len() == 1);
            let e_left = borders.next().unwrap();
            let e_top = borders.next().unwrap();
            row.push(tile.rotate_so(e_left, Left, e_top, Top).unwrap());
        } else {
            let above = &img[i-1][0];
            let tile = find(&edges, &tiles, above, Bot);
            let e_top = above.edge(Bot);
            row.push([Left, Right, Top, Bot]
                .iter()
                .map(|s| tile.edge(*s))
                .filter(|e| edges[&e.primary()].len() == 1)
                .filter_map(|e| tile.rotate_so(e, Left, e_top, Top))
                .next()
                .unwrap());
        }

        for j in 1..12 {
            let left = &row[j-1];
            let tile = find(&edges, &tiles, left, Right);

            let e_left = left.edge(Right);
            if i == 0 {
                row.push([Left, Right, Top, Bot]
                    .iter()
                    .map(|s| tile.edge(*s))
                    .filter(|e| edges[&e.primary()].len() == 1)
                    .filter_map(|e| tile.rotate_so(e_left, Left, e, Top))
                    .next()
                    .unwrap())
            } else {
                let above = &img[i-1][j];
                let e_top = above.edge(Bot);
                row.push(tile.rotate_so(e_left, Left, e_top, Top).unwrap());
            };

        }

        img.push(row);
    }

    // Check that tile layout is correct
    for i in 1..11 {
        for j in 1..11 {
            assert_eq!(img[i][j].edge(Left).primary(), img[i][j-1].edge(Right).primary()); 
            assert_eq!(img[i][j].edge(Right).primary(), img[i][j+1].edge(Left).primary()); 
            assert_eq!(img[i][j].edge(Top).primary(), img[i-1][j].edge(Bot).primary()); 
            assert_eq!(img[i][j].edge(Bot).primary(), img[i+1][j].edge(Top).primary()); 
        }
    }

    let mut image: Vec<Vec<bool>> = Vec::new();
    for y in 0..(12*8) {
        let mut row = Vec::new();
        for x in 0..(12*8) {
            let b = img[y/8][x/8].get((x%8)+1, (y%8)+1);
            row.push(b);
        }
        image.push(row);
    }

    let sea_monster = 
"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

//let sea_monster = "#";
    let monster: Vec<Vec<bool>> = sea_monster
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    println!("monster is {:?}", monster);

    println!("Part 2");
    for _ in 0..4 {
        let found = find_sea_monsters(&image, &monster);
        if found.0 > 0 {
            println!("found {} monsters. roughness = {}", found.0, found.1);
        }
        image = rearrange_image(&image, |(x,y)| (image.len()-1-y, x));
    }

    image = rearrange_image(&image, |(x,y)| (image.len()-1-x, y));

    for _ in 0..4 {
        let found = find_sea_monsters(&image, &monster);
        if found.0 > 0 {
            println!("found {} monsters. roughness = {}", found.0, found.1);
        }
        image = rearrange_image(&image, |(x,y)| (image.len()-1-y, x));
    }

}

fn rearrange_image<F: Fn((usize, usize)) -> (usize, usize)>(image: &Vec<Vec<bool>>, f: F) -> Vec<Vec<bool>> {
    let mut out = image.clone();
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            let (x,y) = f((j, i));
            out[y][x] = image[i][j];
        }
    }
    out
}

