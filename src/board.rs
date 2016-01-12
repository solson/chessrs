use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

#[derive(Clone, Debug)]
pub struct Board<T> {
    cells: Vec<T>,
    width: u32,
    height: u32,
}

impl Board<bool> {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            cells: vec![false; width * height],
            width: width as u32,
            height: height as u32,
        }
    }

    pub fn new_test_board() -> Self {
        let mut board = Board::new(2, 3);
        board[2][0] = true;
        board[1][0] = true;
        board[0][0] = true;
        board[2][1] = true;
        board
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

impl<T> Index<usize> for Board<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width as usize;
        &self.cells[start..start + self.width as usize]
    }
}

impl<T> IndexMut<usize> for Board<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.width as usize;
        &mut self.cells[start..start + self.width as usize]
    }
}

impl<T> IntoIterator for Board<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Board<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut Board<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
    }
}
