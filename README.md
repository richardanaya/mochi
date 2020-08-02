# Mochi 

A mobile game engine using Gtk and Cairo written in Rust!

```rust
engine::init("game.gresource");

let img_sad = engine::image_from_resource("/game/sad.png");
let img_happy = engine::image_from_resource("/game/happy.png");

engine::run_game(move |window, ctx, pointer, delta_time| {
    if pointer.is_down() {
      engine::draw_image_centered(ctx,window.width/2.0,window.height/2.0,img_happy);
    } else {
      engine::draw_image_centered(ctx,window.width/2.0,window.height/2.0,img_sad);
    }
});
```