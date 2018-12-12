extern crate itertools;

use itertools::Itertools;

trait Area {
    fn spaces(&self) -> Vec<SquareInch>;
}

trait Collides<T> {
    fn collides(&self, s: &T) -> bool;
}

pub struct Claim {
    pub id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl Area for Claim {
    fn spaces(&self) -> Vec<SquareInch> {
        let mut vec = Vec::new();
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.height) {
                vec.push(SquareInch { x, y });
            }
        }
        vec
    }
}

impl Collides<Claim> for Claim {
    fn collides(&self, other: &Claim) -> bool {
        for space in self.spaces() {
            for other_space in other.spaces() {
                if space.collides(&other_space) {
                    return true;
                }
            }
        }
        false
    }
}

impl std::cmp::PartialEq for Claim {
    fn eq(&self, other: &Claim) -> bool {
        self.id == other.id
            && self.left == other.left
            && self.top == other.top
            && self.width == other.width
            && self.height == other.height
    }
}

struct SquareInch {
    x: i32,
    y: i32,
}

impl Collides<SquareInch> for SquareInch {
    fn collides(&self, s: &SquareInch) -> bool {
        self.x == s.x && self.y == s.y
    }
}

fn package_claim(s: &str) -> Claim {
    let mut split = s.split_whitespace();
    let id = split.next().unwrap()[1..].parse::<i32>().unwrap();
    split.next(); // @
    let (left, top) = split.next().unwrap().split(",").next_tuple().unwrap();
    let top = top.trim_end_matches(":");
    let (width, height) = split.next().unwrap().split("x").next_tuple().unwrap();
    Claim {
        id,
        left: left.parse::<i32>().unwrap(),
        top: top.parse::<i32>().unwrap(),
        width: width.parse::<i32>().unwrap(),
        height: height.parse::<i32>().unwrap(),
    }
}

fn fill_board(board: &mut [[i32; 1000]; 1000], claims: &Vec<Claim>) {
    for claim in claims {
        for space in claim.spaces() {
            board[space.x as usize][space.y as usize] =
                match board[space.x as usize][space.y as usize] {
                    0 => claim.id,
                    _ => -1,
                };
        }
    }
}

pub fn square_inches(values: &[String]) -> u32 {
    let mut sum = 0;
    let mut board: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    for claim in values.iter().map(|x| package_claim(&x)) {
        for space in claim.spaces() {
            if board[space.x as usize][space.y as usize] == 1 {
                sum += 1;
            }
            board[space.x as usize][space.y as usize] += 1;
        }
    }
    sum
}

pub fn solo_claim(values: &[String]) -> i32 {
    let claims: Vec<Claim> = values.iter().map(|x| package_claim(x)).collect();
    let mut board = [[0i32; 1000]; 1000];
    fill_board(&mut board, &claims);
    for claim in claims.iter() {
        let mut dirty = false;
        for space in claim.spaces() {
            if board[space.x as usize][space.y as usize] < 0 {
                dirty = true;
            }
        }
        if !dirty {
            return claim.id;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_confirms_two_points_in_same_space() {
        let (p1, p2) = (SquareInch { x: 3, y: 5 }, SquareInch { x: 3, y: 5 });
        assert_eq!(true, p1.collides(&p2))
    }

    #[test]
    fn it_denies_two_points_in_different_space() {
        let (p1, p2) = (SquareInch { x: 2, y: 5 }, SquareInch { x: 3, y: 5 });
        assert_eq!(false, p1.collides(&p2))
    }

    #[test]
    fn it_packages_claims() {
        let input = "#1 @ 53,238: 26x24";
        let expected = Claim {
            id: 1,
            left: 53,
            top: 238,
            width: 26,
            height: 24,
        };
        let output = package_claim(input);
        assert_eq!(true, output.eq(&expected))
    }
}
