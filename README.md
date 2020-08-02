# Mochi 

A mobile game engine using Gtk and Cairo written in Rust!

```rust
engine::init("game.gresource");

let img_mochi = engine::image_from_resource("/game/mochi.png");
let img_mochi_eaten = engine::image_from_resource("/game/mochi_eaten.png");

engine::run_game(move |window, ctx, pointer, delta_time| {
    if pointer.is_down() {
      engine::draw_image_centered(ctx,window.width/2.0,window.height/2.0,img_mochi_eaten);
    } else {
      engine::draw_image_centered(ctx,window.width/2.0,window.height/2.0,img_mochi);
    }
});
```