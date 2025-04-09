use macroquad::prelude::*;

pub enum TileType {
    Ground,
    Solid,
    TallGrass
}


pub struct Tileset {
    tiles : Vec<Texture2D>,
    types : Vec<TileType>
}

impl Tileset {
    pub fn new() -> Tileset {
        Tileset {
            tiles : Vec::new(),
            types : Vec::new()
        }
    }

    pub async fn add_tile(&mut self, img_path: String, tile_type : TileType) {
        self.tiles.push(load_texture(img_path.as_str()).await.unwrap());
        self.types.push(tile_type);
    }

    pub async fn load_tileset(&mut self) {
        self.add_tile("tex/overworld/tiles/001.png".to_owned(), TileType::Solid ).await;
        self.add_tile("tex/overworld/tiles/002.png".to_owned(), TileType::Ground).await;
        self.add_tile("tex/overworld/tiles/003.png".to_owned(), TileType::Ground).await;
        self.add_tile("tex/overworld/tiles/004.png".to_owned(), TileType::TallGrass).await;
    }

    pub fn get_tile(&self, index: usize) -> &Texture2D{
        self.tiles.get(index-1).unwrap()
    }

    pub fn get_type(&self, index: usize) -> &TileType {
        self.types.get(index-1).unwrap()
    }
}