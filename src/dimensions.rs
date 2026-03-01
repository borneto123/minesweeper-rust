use crate::coords::{Coords, CoordsOutOfBoundsError};


pub struct Dimensions {
    row_count: usize,
    col_count: usize,
}

impl Dimensions {
    pub fn new(row_count: usize, col_count:usize) -> Self {
        Self {
            row_count,
            col_count,
        }
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn col_count(&self) -> usize{
        self.col_count
    }

    pub fn area(&self) -> usize {
        self.row_count * self.col_count
    }

    pub fn contains(&self, coords: &Coords) -> Result<(), CoordsOutOfBoundsError> {
        match (
            coords.row() >= self.row_count,
            coords.col() >= self.col_count,
        ) {
            (true, true) => Err(CoordsOutOfBoundsError::BothOutOfBounds),

            (true, false) => Err(CoordsOutOfBoundsError::RowOutOfBounds),

            (false, true) => Err(CoordsOutOfBoundsError::ColOutOfBounds),

            (false, false) => Ok(()),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_1() {
        let dim = Dimensions::new(10, 4);
        let cords = Coords::new(2, 2);
        let res = dim.contains(&cords);
        assert!(matches!(res, Ok(())));
    }

    #[test]
    fn contains_2() {
        let dim = Dimensions::new(10, 4);
        let cords = Coords::new(10, 2);
        let res = dim.contains(&cords);
        assert!(matches!(res, Err(CoordsOutOfBoundsError::RowOutOfBounds)));
    }

    #[test]
    fn contains_3() {
        let dim = Dimensions::new(10, 1);
        let cords = Coords::new(10, 2);
        let res = dim.contains(&cords);
        assert!(matches!(res, Err(CoordsOutOfBoundsError::BothOutOfBounds)));
    }

    #[test]
    fn contains_4() {
        let dim = Dimensions::new(2, 4);
        let cords = Coords::new(1   , 5);
        let res = dim.contains(&cords);
        assert!(matches!(res, Err(CoordsOutOfBoundsError::ColOutOfBounds)));
    }
}