use mochi::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Game {
    frame: usize,
    time: f64,
}

fn main() {
    init(include_bytes!("atlas.gresource"));

    let img_horse = Rc::new(RefCell::new(image_from_resource("/atlas/horse.png")));
    let atlas_horse = Atlas::new(img_horse, 7, 3, 18);

    let game = Rc::new(RefCell::new(Game {
        frame: 0,
        time: 0.0,
    }));

    run_game(move |window, ctx, _pointer, delta_time| {
        let mut g = game.borrow_mut();
        g.time += delta_time;
        if g.time > 0.1 {
            g.frame = (g.frame + 1) % atlas_horse.frames.len();
            g.time -= 0.1;
        }
        ctx.draw_atlas_frame_centered(
            window.width / 2.0,
            window.height / 2.0,
            &atlas_horse,
            g.frame,
        );
    });
}
