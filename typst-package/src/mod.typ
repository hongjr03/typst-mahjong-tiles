#let p = plugin("../mahjong_tiles.wasm")

#let mahjong(hand, ..args, tile-set: "fluffy-stuff") = image.decode(p.mahjong(bytes(hand), bytes(tile-set)), ..args)
