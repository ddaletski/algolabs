use rand::seq::SliceRandom;
use std::borrow::Borrow;

use itertools::Itertools;
use rand::Rng;

use union_find::FastUnionUF;

struct Percolation {
    /*
           (top cell)
          /   |     \
        0_0 0_1 ... 0_N
        1_0 1_1 ... 1_N
        ...............
        M_0 M_1 ... M_N
          \  ׀     /
        (bottom cell)
    */
    dsu: FastUnionUF,
    width: usize,
}

pub trait IntExt<T: Ord + PartialOrd> {
    fn between(&self, min: T, max: T) -> bool;
}

impl IntExt<i64> for i64 {
    fn between(&self, min: i64, max: i64) -> bool {
        min <= *self && *self < max
    }
}

// private
impl Percolation {
    fn top_id(&self) -> usize {
        0
    }

    fn bottom_id(&self) -> usize {
        self.dsu.max_size() - 1
    }

    fn dsu_id_for(&self, row: usize, col: usize) -> usize {
        1 + self.width * row + col
    }
}

// public
impl Percolation {
    fn new(rows: usize, cols: usize) -> Percolation {
        assert_ne!(rows, 0);
        assert_ne!(cols, 0);

        // + two hidden cells for the top and bottom
        let mut this = Percolation {
            dsu: FastUnionUF::new(rows * cols + 2),
            width: cols,
        };

        this.dsu.insert(this.top_id());
        this.dsu.insert(this.bottom_id());

        this
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        (self.dsu.max_size() - 2) / self.width
    }

    pub fn open(&mut self, row: usize, col: usize) {
        let current_cell_id = self.dsu_id_for(row, col);
        self.dsu.insert(current_cell_id);

        let neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter() // 4-connected neighbors relative pos.
            .map(|(dy, dx)| (row as i64 + dy, col as i64 + dx)) // neighbor absolute pos.
            .filter(|(y, x)| {
                y.between(0, self.height() as i64) && x.between(0, self.width() as i64)
            }) // inside grid bounds
            .map(|(y, x)| (self.dsu_id_for(y as usize, x as usize))) // dsu id
            .filter(|id| self.dsu.contains(*id))
            .collect_vec();

        for id in neighbors {
            self.dsu.join(id, current_cell_id)
        }

        if row == 0 {
            self.dsu.join(current_cell_id, self.top_id())
        }

        if row == self.height() - 1 {
            self.dsu.join(current_cell_id, self.bottom_id())
        }
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.dsu.contains(self.dsu_id_for(row, col))
    }

    pub fn is_full(&self, row: usize, col: usize) -> bool {
        self.dsu.connected(self.top_id(), self.dsu_id_for(row, col))
    }

    pub fn count_open(&self) -> usize {
        self.dsu.size() - 2
    }

    pub fn percolates(&self) -> bool {
        self.dsu.connected(self.top_id(), self.bottom_id())
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut perc = Percolation::new(100, 100);
    let mut all_points = (0..100).cartesian_product(0..100).collect_vec();
    all_points.shuffle(&mut rng);

    for (y, x) in all_points {
        perc.open(y, x);

        if perc.percolates() {
            break;
        }
    }

    println!(
        "percolates at {:.2}% open sites",
        perc.count_open() as f64 / 100.0 / 100.0
    );
}

#[cfg(test)]
mod tests {
    use crate::Percolation;

    #[test]
    fn height_valid() {
        for rows in 1..100 {
            for cols in 1..100 {
                let p = Percolation::new(rows, cols);
                assert_eq!(p.height(), rows);
            }
        }
    }

    #[test]
    fn default_percolation_is_empty() {
        let p = Percolation::new(100, 100);
        assert_eq!(p.count_open(), 0);
    }

    #[test]
    fn default_grid_doesnt_percolate() {
        let p = Percolation::new(100, 100);
        assert!(!p.percolates());
    }

    #[test]
    fn full_grid_percolates() {
        let mut p = Percolation::new(100, 100);
        for i in 0..100 {
            for j in 0..100 {
                p.open(i, j);
            }
        }
        assert!(p.percolates());
    }

    #[test]
    fn open_works() {
        let mut p = Percolation::new(100, 100);
        for row in 0..100 {
            for col in 0..100 {
                assert!(!p.is_open(row, col));
                p.open(row, col);
                assert!(p.is_open(row, col));
            }
        }
    }
}
