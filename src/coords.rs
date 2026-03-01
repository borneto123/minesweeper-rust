use crate::dimensions::Dimensions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coords {
    row: usize,
    col: usize,
}

impl Coords {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn to_index(&self, dim: &Dimensions) -> Result<usize, CoordsOutOfBoundsError>{
       dim.contains(self)?;
       Ok(self.col + self.row * dim.col_count())
    }

    pub fn from_index(index: &usize, dim: &Dimensions) -> Result<Coords, IndexOutOfBounds> {
        let row = index / dim.col_count();
        let col = index % dim.col_count();

        let res = Coords { 
            row,
            col, 
        };

        dim.contains(&res).map_err(|_| IndexOutOfBounds)?;

        Ok(res)
    }

}

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
        let dim = Dimensions::new(5, 2);
        let coords = Coords::new(2, 1);
        let rez = coords.to_index(&dim);

        assert!(matches!(rez, Ok(5)));
    }

    #[test]
    fn to_index_2() {
        let dim = Dimensions::new(3, 4);
        let coords = Coords::new(2, 2);
        let rez = coords.to_index(&dim);

        assert!(matches!(rez, Ok(10)));
    }

    #[test]
    fn to_index_3() {
        let dim = Dimensions::new(3, 4);
        let coords = Coords::new(4, 4);
        let rez = coords.to_index(&dim);

        assert!(matches!(rez, Err(CoordsOutOfBoundsError::BothOutOfBounds)));
    }
    #[test]
    fn from_index_1() {
        let dim = Dimensions::new(3, 4);
        let index = 10;
        let rez = Coords::from_index(&index, &dim).unwrap();
        dbg!(rez);
        assert_eq!(rez, Coords::new(2, 2));
    }
    #[test]
    fn from_index_2() {
        let dim = Dimensions::new(3, 4);
        let index = 10;
        let rez = Coords::from_index(&index, &dim).unwrap();
        dbg!(rez);
        assert_eq!(rez, Coords::new(2, 2));
    }
}
