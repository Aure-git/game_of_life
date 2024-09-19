pub struct GameGrid {
    rows: usize,
    cols: usize,
    values: Vec<Vec<bool>>, // true = alive | false = dead
}


impl GameGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        // let mut values = Vec::with_capacity(rows);
        // (0..rows).for_each(|_| {
        //     values.push(vec![false;cols]);
        // });
        // GameGrid {rows, cols, values}
        GameGrid { rows, cols, values: vec![vec![false; cols]; rows] }
    }

    pub fn get_shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn is_in(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
        if self.is_in(row, col) {
            return Some(self.values[row][col]);
        }
        None
    }

    /// ## Errors
    /// If the given position is outside the grid
    pub fn switch_state_at(&mut self, row: usize, col: usize) -> Result<(), ()> {
        if self.is_in(row, col) {
            let state = self.values[row][col];
            self.values[row][col] = !state;
            return Ok(());
        }
        Err(())
    }

    /// Clear the board to an empty board
    pub fn clear(&mut self) {
        self.values.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                *cell = false;
            });
        });
    }

    /// ## Panics
    /// Panics if the given position is **NOT** in the grid
    fn alive_neighbors(&self, row: usize, col: usize) -> u8 {
        assert!(self.is_in(row, col));
        let mut count = 0;
        for dx in 0..3_usize { // dx ranges from -1 to 1 so we use dx-1 everywhere
            if !(dx == 0 && col == 0) && !(dx == 2 && col == self.cols-1) {
                for dy in 0..3_usize {
                    if !(dy == 0 && row == 0) && !(dy == 2 && row == self.rows-1) && !(dx == dy && dx == 1) {
                        if self.values[row + dy -1][col + dx -1] {
                            count += 1;
                        }
                    }
                }
            } 
        }

        count
    }

    /// Compute the next generation of the board
    /// 
    /// Return every updated cells in the grid (row,col,new_state)
    pub fn next_generation(&mut self) {
        let mut new_values = vec![vec![false; self.cols]; self.rows];
        for row in 0..self.rows {
            for col in 0..self.cols {
                let alive_neighbors = self.alive_neighbors(row,col);
                if self.values[row][col] && (alive_neighbors == 2 || alive_neighbors == 3) {
                    new_values[row][col] = true;
                }
                else if alive_neighbors == 3 {
                    new_values[row][col] = true;
                }
            }
        }
        
        self.values = new_values;
    }
}