#![no_std]
#![no_main]

use em_graph::State;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, gpio::Io, i2c::I2C, peripherals::Peripherals, prelude::*,
    system::SystemControl,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

mod device;
mod em_graph;

type Instant = fugit::Instant<u64, 1, 1000000>;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    //let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    //// I2C init
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    // Create a new peripheral object with the described wiring
    // and standard I2C clock speed
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio33,
        io.pins.gpio34,
        1000.kHz(),
        &clocks,
        None,
    );

    //// Display init
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    //// State setup
    let mut state = State::new();
    em_graph::setup(&mut display, &mut state).unwrap();
    display.flush().unwrap();

    //// Main loop
    let mut t0_last_frame_time = esp_hal::time::current_time();
    loop {
        //// Update State
        let t1_update_state = esp_hal::time::current_time();
        em_graph::update(&mut state);

        //// Draw State
        let t2_draw_frame = esp_hal::time::current_time();
        em_graph::draw(&mut display, &mut state).unwrap();
        state.frame_count += 1;

        //// Push Data
        let t3_send_data = esp_hal::time::current_time();
        display.flush().unwrap();

        //// Meausure Time
        let t4_now = esp_hal::time::current_time();
        // Log time
        log::info!(
            "total frame: {}ms, update state, {}, draw time: {}, transmit time: {}",
            milis_spent(t0_last_frame_time, t1_update_state),
            milis_spent(t1_update_state, t2_draw_frame),
            milis_spent(t2_draw_frame, t3_send_data),
            milis_spent(t3_send_data, t4_now),
        );
        t0_last_frame_time = t4_now;
    }
}

fn milis_spent(start: Instant, end: Instant) -> f32 {
    (end - start).to_micros() as f32 / 1000.
}
