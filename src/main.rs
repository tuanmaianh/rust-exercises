fn main() {
    println!("Hello, Rust Bootcamp by VBI Academy!");
    let direction = Direction::East;
    println!("{:?}",direction.opposite());
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East
        }
    }
}