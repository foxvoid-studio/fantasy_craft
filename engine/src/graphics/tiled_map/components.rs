#[derive(Debug)]
pub struct TileMapComponent {
    pub name: String
}

#[derive(Debug)]
pub struct TileMapLayerComponent {
    pub tilemap_name: String,
    pub layer_name: String
}

#[derive(Debug)]
pub struct MainTileMap;
