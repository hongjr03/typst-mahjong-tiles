use std::io::Cursor;

use riichi_hand::image::ImageFormat;
use riichi_hand::parser::HandParser;
use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::YELLOW_FLUFFY_STUFF_TILE_SET;
use riichi_hand::raster_renderer::martin_persson_tile_sets::MARTIN_PERSSON_TILE_SET;
use riichi_hand::raster_renderer::{RasterRenderer, RenderOptions};
use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn mahjong(arg: &[u8], tile_set: &[u8]) -> Result<Vec<u8>, String> {
    let string = String::from_utf8(arg.to_vec()).map_err(|e| e.to_string())?;
    let hand = HandParser::parse(&string).map_err(|e| e.to_string())?;
    let tile_set_str = String::from_utf8(tile_set.to_vec()).map_err(|e| e.to_string())?;
    let image = match tile_set_str.as_str() {
        "fluffy-stuff" => RasterRenderer::render(
            &hand,
            &*YELLOW_FLUFFY_STUFF_TILE_SET,
            RenderOptions::default(),
        )
        .unwrap(),
        "martin-persson" => {
            RasterRenderer::render(&hand, &*MARTIN_PERSSON_TILE_SET, RenderOptions::default())
                .unwrap()
        }
        _ => return Err("Invalid tile set".to_string()),
    };

    let mut bytes = vec![];
    image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .unwrap();

    Ok(bytes)
}
