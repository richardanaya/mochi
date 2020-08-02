# Mochi :dango: 

A mobile game engine using Gtk and Cairo written in Rust!

<a href="https://docs.rs/mochi"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

```
[dependencies]
mochi = "0.0"
```

```rust
init("game.gresource");

let img_mochi = image_from_resource("/game/mochi.png");
let img_mochi_eaten = image_from_resource("/game/mochi_eaten.png");

run_game(move |window, ctx, pointer, delta_time| {
    if pointer.is_down() {
        draw_image_centered(
            ctx,
            window.width / 2.0,
            window.height / 2.0,
            img_mochi_eaten,
        );
    } else {
        draw_image_centered(ctx, window.width / 2.0, window.height / 2.0, img_mochi);
    }
});
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `mochi` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.