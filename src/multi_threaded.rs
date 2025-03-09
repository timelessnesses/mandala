use sdl2::mixer::LoaderRWops;
use sdl2::{
    self,
    mixer::{Channel, AUDIO_F32, DEFAULT_CHANNELS},
    sys::SDL_Delay,
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{collections::HashMap, io::Write, process::exit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Note {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
}

impl Note {
    fn to_bytes(&self) -> &'static [u8] {
        match self {
            Note::Mercury => rotation_around_sun_days::MERCURY_NOTE,
            Note::Venus => rotation_around_sun_days::VENUS_NOTE,
            Note::Earth => rotation_around_sun_days::EARTH_NOTE,
            Note::Mars => rotation_around_sun_days::MARS_NOTE,
            Note::Jupiter => rotation_around_sun_days::JUPITER_NOTE,
            Note::Saturn => rotation_around_sun_days::SATURN_NOTE,
            Note::Uranus => rotation_around_sun_days::URANUS_NOTE,
            Note::Neptune => rotation_around_sun_days::NEPTUNE_NOTE,
            Note::Pluto => rotation_around_sun_days::PLUTO_NOTE,
        }
    }
}

pub mod rotation_around_sun_days {
    pub const MERCURY_FACTOR: f32 = 88.0;
    pub const VENUS_FACTOR: f32 = 224.7;
    pub const EARTH_FACTOR: f32 = 365.256;
    pub const MARS_FACTOR: f32 = 686.93;
    pub const JUPITER_FACTOR: f32 = 4_333.0;
    pub const SATURN_FACTOR: f32 = 10_756.0;
    pub const URANUS_FACTOR: f32 = 30_687.0;
    pub const NEPTUNE_FACTOR: f32 = 60_190.0;
    pub const PLUTO_FACTOR: f32 = (248.0 * EARTH_FACTOR) as f32;

    pub const MERCURY: f32 = MERCURY_FACTOR / MERCURY_FACTOR;
    pub const VENUS: f32 = VENUS_FACTOR / MERCURY_FACTOR;
    pub const EARTH: f32 = EARTH_FACTOR / MERCURY_FACTOR;
    pub const MARS: f32 = MARS_FACTOR / MERCURY_FACTOR;
    pub const JUPITER: f32 = JUPITER_FACTOR / MERCURY_FACTOR;
    pub const SATURN: f32 = SATURN_FACTOR / MERCURY_FACTOR;
    pub const URANUS: f32 = URANUS_FACTOR / MERCURY_FACTOR;
    pub const NEPTUNE: f32 = NEPTUNE_FACTOR / MERCURY_FACTOR;
    pub const PLUTO: f32 = PLUTO_FACTOR / MERCURY_FACTOR;

    pub const MERCURY_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_1_61Hz.wav");
    pub const VENUS_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_2_123Hz.wav");
    pub const EARTH_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_3_185Hz.wav");
    pub const MARS_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_4_246Hz.wav");
    pub const JUPITER_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_5_311Hz.wav");
    pub const SATURN_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_6_370Hz.wav");
    pub const URANUS_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_7_440Hz.wav");
    pub const NEPTUNE_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_8_493Hz.wav");
    pub const PLUTO_NOTE: &[u8] = include_bytes!("../notes/harmonic_series_9_554Hz.wav");

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

struct Planet {
    note: Note,
    bpm: f32,
    orbit_radius: i32,
    color: sdl2::pixels::Color,
}

fn spawn_thread_planet(
    bpm: f32,
    note: Note,
    main_audio_thread: std::sync::mpsc::Sender<Note>,
    vis_state: Arc<Mutex<HashMap<Note, Instant>>>,
) {
    std::thread::spawn(move || loop {
        {
            let mut state = vis_state.lock().unwrap();
            state.insert(note, Instant::now());
        }
        main_audio_thread.send(note).unwrap();
        std::thread::sleep(Duration::from_secs_f32(60.0 / bpm));
    });
}

pub fn multi_threaded_main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Mandala", 800, 600).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let _mixer = sdl2::mixer::init(sdl2::mixer::InitFlag::all()).unwrap();
    sdl2::mixer::open_audio(44100, AUDIO_F32, DEFAULT_CHANNELS, 2048).unwrap();
    sdl2::mixer::allocate_channels(20);
    sdl2::mixer::Channel::all().set_volume(100);

    let (main_audio, main_audio_rx) = std::sync::mpsc::channel::<Note>();

    std::thread::spawn(move || {
        let audio_channel = main_audio_rx;
        let mut preloaded_chunks = HashMap::new();
        let mut max_channels_used = 0;
        loop {
            let note = audio_channel.recv().unwrap();
            let chunk = preloaded_chunks.entry(note).or_insert_with(|| {
                sdl2::rwops::RWops::from_bytes(note.to_bytes())
                    .unwrap()
                    .load_wav()
                    .unwrap()
            });
            Channel::all().play(chunk, 0).unwrap();
            max_channels_used = max_channels_used.max(sdl2::mixer::get_playing_channels_number());
            print!(
                "\rMax channels used: {} Playing channels: {}",
                max_channels_used,
                sdl2::mixer::get_playing_channels_number()
            );
            std::io::stdout().flush().unwrap();
        }
    });

    let vis_state: Arc<Mutex<HashMap<Note, Instant>>> = Arc::new(Mutex::new(HashMap::new()));

    let planets = vec![
        Planet {
            note: Note::Mercury,
            bpm: rotation_around_sun_days::MERCURY_ORIGINAL_BPM,
            orbit_radius: 50,
            color: sdl2::pixels::Color::GREY,
        },
        Planet {
            note: Note::Venus,
            bpm: rotation_around_sun_days::VENUS_ORIGINAL_BPM,
            orbit_radius: 75,
            color: sdl2::pixels::Color::RGB(255, 165, 0),
        },
        Planet {
            note: Note::Earth,
            bpm: rotation_around_sun_days::EARTH_ORIGINAL_BPM,
            orbit_radius: 100,
            color: sdl2::pixels::Color::RGB(0, 0, 255),
        },
        Planet {
            note: Note::Mars,
            bpm: rotation_around_sun_days::MARS_ORIGINAL_BPM,
            orbit_radius: 125,
            color: sdl2::pixels::Color::RGB(255, 100, 0),
        },
        Planet {
            note: Note::Jupiter,
            bpm: rotation_around_sun_days::JUPITER_ORIGINAL_BPM,
            orbit_radius: 150,
            color: sdl2::pixels::Color::RGB(218, 165, 200),
        },
        Planet {
            note: Note::Saturn,
            bpm: rotation_around_sun_days::SATURN_ORIGINAL_BPM,
            orbit_radius: 175,
            color: sdl2::pixels::Color::RGB(210, 180, 140),
        },
        Planet {
            note: Note::Uranus,
            bpm: rotation_around_sun_days::URANUS_ORIGINAL_BPM,
            orbit_radius: 200,
            color: sdl2::pixels::Color::RGB(0, 255, 255),
        },
        Planet {
            note: Note::Neptune,
            bpm: rotation_around_sun_days::NEPTUNE_ORIGINAL_BPM,
            orbit_radius: 225,
            color: sdl2::pixels::Color::RGB(0, 0, 139),
        },
        Planet {
            note: Note::Pluto,
            bpm: rotation_around_sun_days::PLUTO_ORIGINAL_BPM,
            orbit_radius: 250,
            color: sdl2::pixels::Color::RGB(128, 128, 128),
        },
    ];

    for planet in &planets {
        let vis_state_clone = Arc::clone(&vis_state);
        spawn_thread_planet(planet.bpm, planet.note, main_audio.clone(), vis_state_clone);
    }

    let start_time = Instant::now();
    let mut first_time = false;
    let mut first_time_instant = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'running;
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        let center_x = 400;
        let center_y = 300;
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(start_time).as_secs_f32();

        let mut flash_count = 0;

        for planet in &planets {
            let period = 60.0 / planet.bpm;
            let angle = (elapsed / period) * 2.0 * std::f32::consts::PI;
            let x = center_x as f32 + planet.orbit_radius as f32 * angle.cos();
            let y = center_y as f32 + planet.orbit_radius as f32 * angle.sin();

            canvas.set_draw_color(planet.color);
            let _ = canvas.fill_rect(sdl2::rect::Rect::new(x as i32 - 5, y as i32 - 5, 10, 10));

            let flash_duration = Duration::from_millis(200);
            let mut flash = false;
            {
                if let Ok(state) = vis_state.lock() {
                    if let Some(&last_hit) = state.get(&planet.note) {
                        if current_time.duration_since(last_hit) < flash_duration {
                            flash = true;
                        }
                    }
                }
            }
            if flash {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                let _ = canvas.draw_line(
                    sdl2::rect::Point::new(center_x, center_y),
                    sdl2::rect::Point::new(x as i32, y as i32),
                );
                flash_count += 1;
            }
        }
        if flash_count == 9 && !first_time {
            // all planet orbited the same time
            first_time = true;
            first_time_instant = Instant::now();
        } else if flash_count == 9 && first_time && first_time_instant.elapsed().as_secs() > 3 {
            println!("Completed");
            std::thread::sleep(Duration::from_secs(20));
            break 'running;
        }
        canvas.present();
        unsafe {
            SDL_Delay(Duration::from_secs_f32(1.0 / 60.0).as_millis() as u32);
        }
    }
    exit(0);
}
