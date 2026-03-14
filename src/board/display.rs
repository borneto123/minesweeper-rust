use std::fmt::Display;

use crate::board::Board;

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_row = 0;
        for (coords, tile) in self.iter() {
            if current_row != coords.row() {
                writeln!(f)?;
                current_row += 1;
            }
            write!(f, "{} ", tile)?;
        }
        writeln!(f)?;
        Ok(())  
    }
}