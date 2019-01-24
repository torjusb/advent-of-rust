extern crate regex;

use regex::Regex;

#[derive(Copy, Clone)]
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

fn get_index(x: u32, y: u32, width: u32) -> usize {
    (x + width * y) as usize
}

fn get_square_indices(claim: Claim, grid_width: u32) -> Vec<usize> {
    let mut vec = vec![];

    for y in 0..claim.height {
        for x in 0..claim.width {
            let index = get_index(x + claim.x, y + claim.y, grid_width);
            vec.push(index);
        }
    }

    vec
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Can't read file");

    const GRID_WIDTH: u32 = 1000;
    const GRID_HEIGHT: u32 = 1000;

    let mut grid = [0; GRID_WIDTH as usize * GRID_HEIGHT as usize];

    let claims = input.lines().map(|claim| parse_claim(&claim));

    for claim in claims.clone() {
        let indices = get_square_indices(claim, GRID_WIDTH);
        for i in indices {
            grid[i] += 1;
        }
    }

    let above_one = grid.into_iter().filter(|i| **i > 1);
    println!("Overlapping cells: {}", above_one.count());

    for claim in claims {
        let indices = get_square_indices(claim, GRID_WIDTH);
        let free = indices.into_iter().all(|i| grid[i] == 1);

        if free {
            println!("Claim #{} does not overlap", claim.id);
            break;
        }
    }
}
