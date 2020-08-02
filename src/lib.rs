use cairo::{Context, Format, ImageSurface, ImageSurfaceData};
use gdk_pixbuf::*;
use gtk::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

const FPS: u32 = 60;

pub struct GameWindow {
    pub width: f64,
    pub height: f64,
}

pub struct Input {
    pub x: f64,
    pub y: f64,
    pub is_down: bool,
}

pub fn init(resource_bytes: &'static [u8]) {
    // we embed our image,glade file, and css  in a glib gresource file generated
    //  from app.xml, let's load it in from bytes embedded in our app
    let bytes = glib::Bytes::from_static(resource_bytes);
    let res = gio::Resource::from_data(&bytes).unwrap();
    gio::resources_register(&res);
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_sign() -> f64 {
    let s = random() - 0.5;
    if s < 0.0 {
        -1.0
    } else {
        1.0
    }
}

pub fn image_from_resource(path: &str) -> ImageSurface {
    let pb = Pixbuf::from_resource(path).unwrap();
    let pixels = unsafe { pb.get_pixels().to_owned() };
    let has_alpha = pb.get_has_alpha();
    let mut img = ImageSurface::create(Format::ARgb32, pb.get_width(), pb.get_height()).unwrap();
    {
        let mut d: ImageSurfaceData = img.get_data().unwrap();
        let data = &mut d;
        let w = pb.get_width();
        for x in 0..w {
            for y in 0..pb.get_height() {
                if has_alpha {
                    let sp = ((y * w + x) * 4) as usize;
                    let p = ((y * w + x) * 4) as usize;
                    data[p] = pixels[sp + 2];
                    data[p + 1] = pixels[sp + 1];
                    data[p + 2] = pixels[sp];
                    data[p + 3] = pixels[sp + 3];
                } else {
                    // TODO, there's a bug with pngs without transparency... not sure where..
                    let sp = ((y * w + x) * 3) as usize;
                    let p = ((y * w + x) * 4) as usize;
                    data[p] = pixels[sp + 2];
                    data[p + 1] = pixels[sp + 1];
                    data[p + 2] = pixels[sp];
                    data[p + 3] = 255;
                }
            }
        }
    }
    img
}

pub trait MochiCairoExt {
    fn clear(&self, r: f64, g: f64, b: f64);
    fn draw_image_centered(&self, x: f64, y: f64, img: &ImageSurface);
}

impl MochiCairoExt for cairo::Context {
    fn clear(&self, r: f64, g: f64, b: f64) {
        self.set_source_rgb(r, g, b);
        self.paint();
    }

    fn draw_image_centered(&self, x: f64, y: f64, img: &ImageSurface) {
        self.save();
        self.translate(
            x - (img.get_width() / 2) as f64,
            y - (img.get_height() / 2) as f64,
        );
        self.set_source_surface(img, 0.0, 0.0);
        self.paint();
        self.restore();
    }
}

pub fn run_game<T>(run: T)
where
    T: 'static + Fn(GameWindow, &Context, &Input, f64),
{
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // grab the controls we'll be using
    let window: gtk::Window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_decorated(false);
    let event_box: gtk::EventBox = gtk::EventBox::new();
    event_box.set_events(gdk::EventMask::STRUCTURE_MASK | gdk::EventMask::TOUCH_MASK);
    window.add(&event_box);
    let drawing_area = gtk::DrawingArea::new();
    event_box.add(&drawing_area);
    let canvas: Rc<RefCell<gtk::DrawingArea>> = Rc::new(RefCell::new(drawing_area));

    window.connect_window_state_event(|w, e| {
        if e.get_new_window_state().contains(gdk::WindowState::FOCUSED) {
            w.fullscreen();
        }
        Inhibit(false)
    });

    let input = Rc::new(RefCell::new(Input {
        x: 0.0,
        y: 0.0,
        is_down: false,
    }));

    let input2 = input.clone();
    let canvas2 = canvas.clone();
    // handle draw and use cairo context
    canvas.borrow_mut().connect_draw(move |_, ctx| {
        run(
            GameWindow {
                width: canvas2.borrow().get_allocated_width() as f64,
                height: canvas2.borrow().get_allocated_height() as f64,
            },
            ctx,
            &input2.borrow(),
            1_f64 / FPS as f64,
        );
        Inhibit(false)
    });

    let input3 = input.clone();
    event_box.connect_button_press_event(move |_, e| {
        let mut inp = input3.borrow_mut();
        let pos = e.get_coords().unwrap();
        inp.is_down = true;
        inp.x = pos.0;
        inp.y = pos.1;
        Inhibit(false)
    });

    let input4 = input.clone();
    event_box.connect_button_release_event(move |_, e| {
        let mut inp = input4.borrow_mut();
        let pos = e.get_coords().unwrap();
        inp.is_down = false;
        inp.x = pos.0;
        inp.y = pos.1;
        Inhibit(false)
    });

    let input5 = input.clone();
    event_box.connect_motion_notify_event(move |_, e| {
        let mut inp = input5.borrow_mut();
        let pos = e.get_coords().unwrap();
        inp.x = pos.0;
        inp.y = pos.1;
        Inhibit(false)
    });

    let input6 = input;

    event_box.connect_touch_event(move |_, e| {
        let mut inp = input6.borrow_mut();
        let pos = e.get_coords().unwrap();
        inp.x = pos.0;
        inp.y = pos.1;
        Inhibit(false)
    });

    // show the window
    window.show_all();

    window.fullscreen();

    let canvas2 = canvas;
    let tick = move || {
        canvas2.borrow_mut().queue_draw();
        glib::Continue(true)
    };

    // executes the game every 30 seconds
    gtk::timeout_add(1000 / FPS, tick);

    // exit properly if our window is closing
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
