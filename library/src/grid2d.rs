use glam::{IVec2, UVec2};
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Grid2D<T>
where
    T: Clone,
{
    width: u32,
    height: u32,
    cells: Vec<Option<T>>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct OutOfBoundsError;

impl Grid2D<char> {
    pub fn from_input(input: &str) -> Self {
        let width = input
            .lines()
            .nth(0)
            .and_then(|line| Some(line.len()))
            .unwrap() as u32;

        let height = input.lines().count() as u32;

        let cells = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => None,
                    x => Some(x),
                })
            })
            .collect_vec();

        Self {
            width,
            height,
            cells,
        }
    }
}

impl<T> Grid2D<T>
where
    T: Clone,
{
    pub fn new_empty(width: u32, height: u32) -> Self {
        let cells = vec![None; (width * height) as usize];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> Result<&Option<T>, OutOfBoundsError> {
        let coord = self.get_index(x, y);

        if let Some(idx) = coord {
            Ok(self.cells.get(idx).unwrap())
        } else {
            Err(OutOfBoundsError)
        }
    }

    pub fn get_vec(&self, pos: IVec2) -> Result<&Option<T>, OutOfBoundsError> {
        let coord = self.get_index_i32(pos.x, pos.y);

        if let Some(idx) = coord {
            Ok(self.cells.get(idx).unwrap())
        } else {
            Err(OutOfBoundsError)
        }
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Result<&mut Option<T>, OutOfBoundsError> {
        let coord = self.get_index(x, y);

        if let Some(idx) = coord {
            Ok(self.cells.get_mut(idx).unwrap())
        } else {
            Err(OutOfBoundsError)
        }
    }

    pub fn get_mut_vec(&mut self, pos: IVec2) -> Result<&mut Option<T>, OutOfBoundsError> {
        let coord = self.get_index_i32(pos.x, pos.y);

        if let Some(idx) = coord {
            Ok(self.cells.get_mut(idx).unwrap())
        } else {
            Err(OutOfBoundsError)
        }
    }
    pub fn iter_cells_row_dominant(&self) -> impl Iterator<Item = (IVec2, &Option<T>)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(idx, cell)| (self.get_coord(idx).unwrap(), cell))
    }

    fn get_index(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some((y * self.width + x) as usize)
        }
    }

    fn get_coord(&self, index: usize) -> Option<IVec2> {
        if index >= self.cells.len() {
            None
        } else {
            let width = self.width as usize;
            Some(IVec2 {
                x: (index % width) as i32,
                y: (index / width) as i32,
            })
        }
    }

    fn get_index_i32(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 00 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            Some((y * self.width as i32 + x) as usize)
        }
    }
}
