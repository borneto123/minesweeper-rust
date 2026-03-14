use crate::{board::error::BoardError, coords::Coords};

impl super::Board {
    pub(crate) fn fill_board(&mut self) -> Result<(), BoardError> {
        let mut mine_count = self.mine_count;
        while mine_count !=0 {

            let rand_coords = Coords::new_rand(&self.dimensions);

            let tile = self
                .get_tile_mut(&rand_coords)
                .ok_or(BoardError::MinePlacementFailed)?;

            if tile.place_mine().is_ok() {
                mine_count -= 1;
            }
        }
        Ok(())
    }

    pub(crate) fn update_mine_neighbours(&mut self) -> Result<(), BoardError>{

        let mines_coords : Vec<Coords> = self.mines_iter()
            .map(|(coords, _ )|{
                coords
            }).collect();

        for mine_coords in mines_coords {
            for tile_coords in mine_coords.get_neighbours(&self.dimensions){
                let tile = self
                    .get_tile_mut(&tile_coords)
                    .ok_or(BoardError::UpdateMineNeighboursFailed)?;
                if tile.is_mine() {
                    continue;
                }
                tile.increment_empty().map_err(|_| BoardError::UpdateMineNeighboursFailed)?;
            }
        }
        Ok(())
    }

}