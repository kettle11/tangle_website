pub enum Event {
    FixedUpdate,
    Draw,
    PlayerJoined {
        player: u32,
    },
    PlayerLeft {
        player: u32,
    },
    PointerMove {
        player: u32,
        pointer_id: u32,
        x: f32,
        y: f32,
    },
    PointerDown {
        player: u32,
        pointer_id: u32,
        x: f32,
        y: f32,
    },
    PointerUp {
        player: u32,
        pointer_id: u32,
        is_mouse: bool,
        x: f32,
        y: f32,
    },
}

fn send_event(event: Event) {
    // This is safe because this Wasm program will only ever be single-threaded.
    unsafe {
        (PROGRAM_FUNCTION.as_mut().unwrap())(event);
    }
}

#[no_mangle]
extern "C" fn player_joined(player: u32) {
    send_event(Event::PlayerJoined { player })
}

#[no_mangle]
extern "C" fn peer_left(player: u32) {
    send_event(Event::PlayerLeft { player })
}

#[no_mangle]
extern "C" fn pointer_down(player: u32, pointer_id: u32, x: f32, y: f32) {
    send_event(Event::PointerDown {
        player,
        pointer_id,
        x: x,
        y: y,
    })
}

#[no_mangle]
extern "C" fn pointer_move(player: u32, pointer_id: u32, x: f32, y: f32) {
    log(&format!("POINTER ID: {:?}", pointer_id));
    send_event(Event::PointerMove {
        player,
        pointer_id,
        x: x,
        y: y,
    })
}

#[no_mangle]
extern "C" fn pointer_up(player: u32, pointer_id: u32, is_mouse: bool, x: f32, y: f32) {
    send_event(Event::PointerUp {
        player,
        pointer_id,
        is_mouse,
        x: x,
        y: y,
    })
}

#[no_mangle]
extern "C" fn fixed_update() {
    send_event(Event::FixedUpdate)
}

#[no_mangle]
extern "C" fn draw() {
    send_event(Event::Draw)
}

static mut PROGRAM_FUNCTION: Option<Box<dyn FnMut(Event) + Sync>> = None;

pub fn run(f: impl FnMut(Event) + 'static + Sync) {
    setup_panic_hook();
    unsafe {
        PROGRAM_FUNCTION = Some(Box::new(f));
    }
}

mod unsafe_external {
    extern "C" {
        pub(crate) fn set_color(r: u8, g: u8, b: u8, a: f32);
        pub(crate) fn draw_circle(x: f32, y: f32, r: f32);
        pub(crate) fn draw_rect(x: f32, y: f32, w: f32, h: f32);
        pub(crate) fn set_transform(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32);
        pub(crate) fn begin_path();
        pub(crate) fn move_to(x: f32, y: f32);
        pub(crate) fn line_to(x: f32, y: f32);
        pub(crate) fn fill();

    }
}

pub fn begin_path() {
    unsafe {
        unsafe_external::begin_path();
    }
}

pub fn fill() {
    unsafe {
        unsafe_external::fill();
    }
}

pub fn move_to(x: f32, y: f32) {
    unsafe {
        unsafe_external::move_to(x, y);
    }
}

pub fn line_to(x: f32, y: f32) {
    unsafe {
        unsafe_external::line_to(x, y);
    }
}

pub fn set_transform(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
    unsafe {
        unsafe_external::set_transform(a, b, c, d, e, f);
    }
}

/// Resets the transform to the identity transform.
pub fn reset_transform() {
    unsafe {
        unsafe_external::set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    }
}

pub fn draw_circle(x: f32, y: f32, radius: f32) {
    unsafe {
        unsafe_external::draw_circle(x, y, radius);
    }
}

pub fn draw_rect(x: f32, y: f32, width: f32, height: f32) {
    unsafe {
        unsafe_external::draw_rect(x, y, width, height);
    }
}

pub fn set_color(r: u8, g: u8, b: u8, a: u8) {
    unsafe {
        unsafe_external::set_color(r, g, b, a as f32 / 255.0);
    }
}

extern "C" {
    pub(crate) fn external_log(data: *const u8, data_length: u32);
}

pub fn log(s: &str) {
    unsafe {
        external_log(s.as_ptr(), s.len() as _);
    }
}

/// Sets up a panic hook to print a slightly more useful error-message to the console.
fn setup_panic_hook() {
    fn hook_impl(info: &std::panic::PanicInfo) {
        let message = info.to_string();
        log(&message);
    }

    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        std::panic::set_hook(Box::new(hook_impl));
    });
}
