extern crate regex;

use regex::Regex;

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn parse_claim(claim: &str) -> Claim {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let capture = re.captures(claim).unwrap();

    // There must be a better way to do this..
    let id: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
    let x: u32 = capture.get(2).unwrap().as_str().parse().unwrap();
    let y: u32 = capture.get(3).unwrap().as_str().parse().unwrap();
    let width: u32 = capture.get(4).unwrap().as_str().parse().unwrap();
    let height: u32 = capture.get(5).unwrap().as_str().parse().unwrap();

    Claim {
        id,
        x,
        y,
        width,
        height,
    }
}

fn coords_to_index(x: u32, y: u32, width: u32) -> usize {
    (x + width * y) as usize
}

fn main() {
    let input = std::fs::read_to_string("./example.txt").expect("Can't read file");

    const GRID_WIDTH: u32 = 8;
    const GRID_HEIGHT: u32 = 8;

    #[derive(Debug)]
    let mut grid = [0; GRID_WIDTH as usize * GRID_HEIGHT as usize];

    let claims = input.lines().map(|claim| parse_claim(&claim));

    claims.for_each(|claim| {
        let i_start = coords_to_index(claim.x, claim.y, GRID_WIDTH);
        let i_stop = coords_to_index(claim.x + claim.width, claim.y + claim.height, GRID_WIDTH);

        println!("{} {}", i_start, i_stop);
        for i in i_start..i_stop {
            grid[i] += 1;
        }
    });

    println!("{:?}", grid.into_iter());

    let above_one = grid.into_iter().filter(|i| **i > 1);
    println!("{}", above_one.count());
}
