use crate::dimensions::Dimensions;

#[derive(Debug)]

pub struct BoardConfig{
    pub(crate) dimensions: Dimensions,
    pub(crate) mine_count: i32,
}

impl BoardConfig {
    pub fn new(dimensions: Dimensions, mine_count: i32) -> Result<Self, BoardConfigError> {
        if mine_count < 0 {
            return Err(BoardConfigError::IvalidMineCount);
        }

        Ok (BoardConfig{
            dimensions,
            mine_count,
        })
    }
}
#[derive(Debug)]
pub enum BoardConfigError {
    IvalidMineCount,
}