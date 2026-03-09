use std::ops::{self};

use rand::RngExt;

use crate::dimensions::{ Dimensions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coords {
    row: i32,
    col: i32,
}

impl Coords {
    pub fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
        }
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&self) -> i32 {
        self.col
    }

    pub fn to_index(&self, dim: &Dimensions) -> Result<i32, CoordsOutOfBoundsError>{
       dim.contains(self)?;
       Ok(self.col + self.row * dim.col_count())
    }

    pub fn new_rand(dim: &Dimensions) -> Self {
        let mut rng = rand::rng();
        let row = rng.random_range(0..dim.row_count());
        let col = rng.random_range(0..dim.col_count());

        Coords{
            row,
            col,
        }
    }

    pub fn from_index(index: &i32, dim: &Dimensions) -> Result<Coords, IndexOutOfBounds> {
        let row = index / dim.col_count();
        let col = index % dim.col_count();

        let res = Coords { 
            row,
            col, 
        };

        dim.contains(&res).map_err(|_| IndexOutOfBounds)?;

        Ok(res)
    }

    pub fn offsets() -> Vec<Coords> {
        let offsets = vec![
            Coords {col: 0, row: 1},
            Coords {col: 0, row: -1},
            Coords {col: 1, row: 0},
            Coords {col: 1, row: 1},
            Coords {col: 1, row: -1},
            Coords {col: -1, row: 0},
            Coords {col: -1, row: 1},
            Coords {col: -1, row: -1},
        ];
        offsets
    }

     pub fn get_neighbours(&self, dim: &Dimensions) -> Vec<Coords> {
        let mut res: Vec<Coords> = Vec::new();
        for offset in Coords::offsets(){
            let coord_offset = *self + offset;
            if dim.contains(&coord_offset).is_ok() {
                res.push(coord_offset);
            }
        }
        res
    }

}

impl Default for Coords {
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
        }
    }
}

impl ops::Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Coords {
        Coords { 
            row: self.row + rhs.row,
            col: self.col + rhs.col
        }
    }
}

#[derive(Debug)]
pub enum CoordsOutOfBoundsError {
    BothOutOfBounds,
    RowOutOfBounds,
    ColOutOfBounds,  
}

#[derive(Debug)]
pub struct IndexOutOfBounds;
#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn to_index_1() {
        let dim = Dimensions::new(5, 2).ok().unwrap();
        let coords = Coords::new(2, 1);
        let rez = coords.to_index(&dim);

        assert!(matches!(rez, Ok(5)));
    }

    #[test]
    fn to_index_2() {
        let dim = Dimensions::new(3, 4).ok().unwrap();
        let coords = Coords::new(2, 2);
        let rez = coords.to_index(&dim);

        assert!(matches!(rez, Ok(10)));
    }

    #[test]
    fn to_index_3() {
        let dim = Dimensions::new(3, 4).ok().unwrap();
        let coords = Coords::new(4, 4);
        let rez = coords.to_index(&dim);

        assert!(matches!(rez, Err(CoordsOutOfBoundsError::BothOutOfBounds)));
    }
    #[test]
    fn from_index_1() {
        let dim = Dimensions::new(3, 4).ok().unwrap();
        let index = 10;
        let rez = Coords::from_index(&index, &dim).unwrap();
        dbg!(rez);
        assert_eq!(rez, Coords::new(2, 2));
    }
    #[test]
    fn from_index_2() {
        let dim = Dimensions::new(3, 4).ok().unwrap();
        let index = 10;
        let rez = Coords::from_index(&index, &dim).unwrap();
        dbg!(rez);
        assert_eq!(rez, Coords::new(2, 2));
    }

    #[test]

    fn get_neighbours_test(){
        let dim = Dimensions::new(3, 4).ok().unwrap();
        let coords = Coords::new(0, 0);
        for i in coords.get_neighbours(&dim) {
            dbg!(i);
        }
    }
}
