use algo_toolbox::union_find::{DenseUF, UnionFind};
use itertools::Itertools;

pub struct Percolation {
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
    uf: DenseUF,
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
        self.uf.capacity() - 1
    }

    fn dsu_id_for(&self, row: usize, col: usize) -> usize {
        1 + self.width * row + col
    }
}

// public
impl Percolation {
    pub fn new(rows: usize, cols: usize) -> Percolation {
        assert_ne!(rows, 0);
        assert_ne!(cols, 0);

        // + two hidden cells for the top and bottom
        let mut this = Percolation {
            uf: DenseUF::new(rows * cols + 2),
            width: cols,
        };

        this.uf.insert(this.top_id());
        this.uf.insert(this.bottom_id());

        this
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        (self.uf.capacity() - 2) / self.width
    }

    pub fn open(&mut self, row: usize, col: usize) {
        let current_cell_id = self.dsu_id_for(row, col);
        self.uf.insert(current_cell_id);

        let neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter() // 4-connected neighbors relative pos.
            .map(|(dy, dx)| (row as i64 + dy, col as i64 + dx)) // neighbor absolute pos.
            .filter(|(y, x)| {
                y.between(0, self.height() as i64) && x.between(0, self.width() as i64)
            }) // inside grid bounds
            .map(|(y, x)| (self.dsu_id_for(y as usize, x as usize))) // dsu id
            .filter(|id| self.uf.contains(*id))
            .collect_vec();

        for id in neighbors {
            self.uf.join(id, current_cell_id);
        }

        if row == 0 {
            self.uf.join(current_cell_id, self.top_id());
        }

        if row == self.height() - 1 {
            self.uf.join(current_cell_id, self.bottom_id());
        }
    }

    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.uf.contains(self.dsu_id_for(row, col))
    }

    pub fn is_full(&mut self, row: usize, col: usize) -> bool {
        self.uf.connected(self.top_id(), self.dsu_id_for(row, col))
    }

    pub fn count_open(&self) -> usize {
        self.uf.len() - 2
    }

    pub fn percolates(&mut self) -> bool {
        self.uf.connected(self.top_id(), self.bottom_id())
    }

    pub fn grid(&mut self) -> ndarray::Array2<u8> {
        let mut grid = ndarray::Array2::<u8>::zeros((self.height(), self.width()));

        for ((row, col), cell) in grid.indexed_iter_mut() {
            let cell_state = if self.is_open(row, col) {
                if self.is_full(row, col) {
                    2
                } else {
                    1
                }
            } else {
                0
            };

            *cell = cell_state
        }

        grid
    }
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
        let mut p = Percolation::new(100, 100);
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
