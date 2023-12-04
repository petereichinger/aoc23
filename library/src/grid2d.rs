#[derive(Debug, Default)]
pub struct Grid2D<T>
where
    T: Clone,
{
    width: usize,
    height: usize,
    cells: Vec<Option<T>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct OutOfBoundsError;

impl<T> Grid2D<T>
where
    T: Clone,
{
    pub fn new_empty(width: usize, height: usize) -> Self {
        let cells = vec![None; width * height];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&Option<T>, OutOfBoundsError> {
        let coord = self.get_coord(x, y);

        if let Some(idx) = coord {
            Ok(self.cells.get(idx).unwrap())
        } else {
            Err(OutOfBoundsError)
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut Option<T>, OutOfBoundsError> {
        let coord = self.get_coord(x, y);

        if let Some(idx) = coord {
            Ok(self.cells.get_mut(idx).unwrap())
        } else {
            Err(OutOfBoundsError)
        }
    }

    fn get_coord(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(y * self.width + x)
        }
    }
}
