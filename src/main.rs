use sdl2::{self, mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS}};

fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let window = video.window("Mandala", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = ctx.event_pump().unwrap();
    let _mixer = sdl2::mixer::init(sdl2::mixer::InitFlag::all()).unwrap();
    let audio = ctx.audio().unwrap();
    sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024).unwrap();
    sdl2::mixer::allocate_channels(100); // just in case
    let volume = 20;
    // max volume is 128 so uhh just idk
    sdl2::mixer::Channel::all().set_volume(((volume as f32 / 100.0) * 128.0) as i32);
    let chunked = sdl2::mixer::Chunk::from_raw_buffer(rotation_around_sun_days::MERCURY_NOTE.into()).unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    }
}


/// this will also be sped up by like 12million times
pub mod rotation_around_sun_days {
    pub const MERCURY_FACTOR: f32 = 88.0;
    pub const VENUS_FACTOR: f32 = 224.7;
    /// > The 6 hours, 9 minutes adds up to about an extra day every fourth year
    /// lmao
    /// float ref from [this](https://en.wikipedia.org/wiki/Earth%27s_orbit)
    pub const EARTH_FACTOR: f32 = 365.256;
    pub const MARS_FACTOR: f32 = 686.93;
    pub const JUPITER_FACTOR: f32 = 4_333.0;
    pub const SATURN_FACTOR: f32 = 10_756.0;
    pub const URANUS_FACTOR: f32 = 30_687.0;
    pub const NEPTUNE_FACTOR: f32 = 60_190.0;
    pub const PLUTO_FACTOR: f32 = (248.0 * EARTH_FACTOR) as f32;

    // mercury as base cuz its fastest
    // this is like base bpm? (prob multiply this by x or something)
    pub const MERCURY: f32 = MERCURY_FACTOR / MERCURY_FACTOR;
    pub const VENUS: f32 = VENUS_FACTOR / MERCURY_FACTOR;
    pub const EARTH: f32 = EARTH_FACTOR / MERCURY_FACTOR;
    pub const MARS: f32 = MARS_FACTOR / MERCURY_FACTOR;
    pub const JUPITER: f32 = JUPITER_FACTOR / MERCURY_FACTOR;
    pub const SATURN: f32 = SATURN_FACTOR / MERCURY_FACTOR;
    pub const URANUS: f32 = URANUS_FACTOR / MERCURY_FACTOR;
    pub const NEPTUNE: f32 = NEPTUNE_FACTOR / MERCURY_FACTOR;
    pub const PLUTO: f32 = PLUTO_FACTOR / MERCURY_FACTOR;

    pub const MERCURY_NOTE: &[u8] = include_bytes!("../notes/db1_piano.wav");
    pub const VENUS_NOTE: &[u8] = include_bytes!("../notes/ab4_piano.wav");
    pub const EARTH_NOTE: &[u8] = include_bytes!("../notes/g4_piano.wav");
    pub const MARS_NOTE: &[u8] = include_bytes!("../notes/gb4_piano.wav");
    pub const JUPITER_NOTE: &[u8] = include_bytes!("../notes/bb4_piano.wav");
    pub const SATURN_NOTE: &[u8] = include_bytes!("../notes/a4_piano.wav");
    pub const URANUS_NOTE: &[u8] = include_bytes!("../notes/c5_piano.wav");
    pub const NEPTUNE_NOTE: &[u8] = include_bytes!("../notes/d5_piano.wav");
    pub const PLUTO_NOTE: &[u8] = include_bytes!("../notes/b4_piano.wav");

    pub const MERCURY_ORIGINAL_BPM: f32 = 124.56;
    pub const VENUS_ORIGINAL_BPM: f32 = 48.766;
    pub const EARTH_ORIGINAL_BPM: f32 = 30.0;
    pub const MARS_ORIGINAL_BPM: f32 = 15.9505;
    pub const JUPITER_ORIGINAL_BPM: f32 = 2.52913;
    pub const SATURN_ORIGINAL_BPM: f32 = 1.01845;
    pub const URANUS_ORIGINAL_BPM: f32 = 0.3571;
    pub const NEPTUNE_ORIGINAL_BPM: f32 = 0.1821;
    pub const PLUTO_ORIGINAL_BPM: f32 = 0.12113;

}
