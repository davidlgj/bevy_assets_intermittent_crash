use bevy::prelude::*;

fn setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle = asset_server.load("hero.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 2, 4);
    let _texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tiles_handle = asset_server.load("alltiles.png");
    let tiles_texture_atlas = TextureAtlas::from_grid(tiles_handle, Vec2::new(16.0, 16.0), 3, 59);
    let _tiles_atlas_handle = texture_atlases.add(tiles_texture_atlas);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
