use bevy::{app::App, asset::AddAsset};
use ra2_asset::{
    asset::{IniAsset, MapAsset, ShpAsset, TileAsset},
    loader::{IniFileAssetLoader, MapLoader, PaletteLoader, ShpLoader, TilesAssetLoader},
    DebugGameState
};
use ra2_data::color::Palette;

pub fn add_assets_and_loaders(app: &mut App) {
    app.add_asset::<MapAsset>()
        .add_asset::<Palette>()
        .add_asset::<TileAsset>()
        .add_asset::<IniAsset>()
        .add_asset::<ShpAsset>()
        .add_asset_loader(PaletteLoader)
        .add_asset_loader(MapLoader)
        .add_asset_loader(TilesAssetLoader)
        .add_asset_loader(IniFileAssetLoader)
        .add_asset_loader(ShpLoader)
        .add_state::<DebugGameState>();
}
