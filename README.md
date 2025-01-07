# Typst-Mahjong-Tiles

[riichi-hand-rs](https://github.com/m4tx/riichi-hand-rs) in Typst, powered by wasm.

## Usage

Start with `#import "src/lib.typ": mahjong`. Reminds that `mahjong_tiles.wasm` should be in the same directory as where `src/` is.

Parameters:

- `hand`: A string of tiles in the hand.
- `tile-set`: The tile set to use. Default is `"fluffy-stuff"`. `"martin-persson"` is also available.
- `..args`: Other arguments to pass to the `image.decode()` fuction.

```typst
#mahjong("21w2www7w28p", tile-set: "martin-persson", alt: "majhong")
```

![21w2www7w28p](assets/21w2www7w28p.png)

```typst
#mahjong("2312936963s", alt: "majhong")
```

![2312936963s](assets/2312936963s.png)

## Credits

- [riichi-hand-rs](https://github.com/m4tx/riichi-hand-rs)
