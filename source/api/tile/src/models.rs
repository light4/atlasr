#[derive(Debug, Queryable)]
pub struct Tiles {
    pub zoom_level: i32,
    pub tile_column: i32,
    pub tile_row: i32,
    pub tile_data: Vec<u8>,
}

#[derive(Debug, Queryable)]
pub struct Metadata {
    pub name: String,
    pub value: String
}
