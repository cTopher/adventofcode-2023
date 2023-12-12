use super::model::{PipeIterator, DIRECTIONS};

pub struct Finder {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Option<Tile>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Loop,
    Outside,
}

impl Finder {
    pub fn new(width: usize, height: usize, mut pipe: PipeIterator) -> Self {
        let width = width * 2 - 1;
        let height = height * 2 - 1;
        let tiles = vec![vec![None; width]; height];
        let mut this = Self {
            width,
            height,
            tiles,
        };
        while let Some((row, column)) = pipe.next() {
            let position_1 = (row * 2, column * 2);
            let position_2 = pipe.direction().apply(position_1).unwrap();
            this.set(position_1, Tile::Loop);
            this.set(position_2, Tile::Loop);
        }
        this.fill();
        this
    }

    pub fn inside_area(&self) -> usize {
        self.tiles
            .iter()
            .step_by(2)
            .map(|row| {
                row.iter()
                    .step_by(2)
                    .filter(|&&tile| tile.is_none())
                    .count()
            })
            .sum()
    }

    fn set(&mut self, (row, column): (usize, usize), tile: Tile) {
        self.tiles[row][column] = Some(tile);
    }

    fn get(&self, (row, column): (usize, usize)) -> Option<Tile> {
        *self.tiles.get(row)?.get(column)?
    }

    fn fill(&mut self) {
        let mut queue = self.border();
        while let Some(position) = queue.pop() {
            if self.get(position).is_some() {
                continue;
            }
            self.set(position, Tile::Outside);
            DIRECTIONS
                .iter()
                .filter_map(|direction| direction.apply(position))
                .filter(|&(row, column)| {
                    row < self.height && column < self.width && self.get((row, column)).is_none()
                })
                .for_each(|position| queue.push(position));
        }
    }

    fn border(&self) -> Vec<(usize, usize)> {
        let mut border = vec![(0, 0)];
        let mut row = 0;
        let mut column = 0;
        while column < self.width - 1 {
            column += 1;
            border.push((row, column));
        }
        while row < self.height - 1 {
            row += 1;
            border.push((row, column));
        }
        while column > 0 {
            column -= 1;
            border.push((row, column));
        }
        while row > 1 {
            row -= 1;
            border.push((row, column));
        }
        border
    }
}
