use mochi::*;

fn main() {
    init(include_bytes!("sound.gresource"));
    let snd = sound_from_resource("/text/hello.mp3");

    play_sound(&snd);
    run_game(move |_window, _ctx, _pointer, _delta_time| {
        // do nothing
    });
}
