quad-svg
=========

render svg to macroquad::texture::Texture2D using resvg

# examples
display 50 svg drawn ferris moving from left to right
```
cargo run --example="ferris"
```

display 100 svg drawn stars with text "Text" that are moving from top to bottom
```
cargo run --example="star"
```

# TODO

I want to include the font in the binary to not rely on system font.
The font example is currently only working if you have font-family "Caladea" installed. Otherwise the star is empty.