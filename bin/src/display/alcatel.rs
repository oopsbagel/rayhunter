/// Display module for Alcatel MW43, blink LEDs on the front of the device.
use log::{error, info};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio_util::task::TaskTracker;

use std::fs::write;
use std::thread::sleep;
use std::time::Duration;

use crate::config;
use crate::display::DisplayState;

macro_rules! led {
    ($l:expr) => {{
        format!("/sys/devices/soc:qcom,leds/leds/{}/brightness", $l)
    }};
}

/// Paused => blink wlan (far left)
/// Recording => blink purple signal and battery
/// WarningDetected => blink everything except wlan
pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: mpsc::Receiver<DisplayState>,
) {
    let mut invisible: bool = false;
    if config.ui_level == 0 {
        info!("Invisible mode, not spawning UI.");
        invisible = true;
    }

    // This is the left to right order of the LEDs on the device.
    // The red and blue1 LEDs overlap.
    let wlan = led!("wlan");
    let signal_red = led!("signal-red");
    let signal_blue1 = led!("signal-b1");
    let signal_blue2 = led!("signal-b2");
    let signal_blue3 = led!("signal-b3");
    let battery_red = led!("battery-red");
    let battery_blue1 = led!("battery-b1");
    let battery_blue2 = led!("battery-b2");
    let battery_blue3 = led!("battery-b3");
    let sms = led!("sms");

    let half_sec = Duration::from_millis(500);

    task_tracker.spawn_blocking(move || {
        // Try to blink each LED once on startup to test whether we can write to the device.
        for led in [
            &wlan,
            &signal_red,
            &signal_blue1,
            &signal_blue2,
            &signal_blue3,
            &battery_red,
            &battery_blue1,
            &battery_blue2,
            &battery_blue3,
            &sms,
        ] {
            write(led, "1").unwrap_or_else(|e| error!("failed to write to led: {e}"));
        }

        loop {
            match ui_shutdown_rx.try_recv() {
                Ok(_) => {
                    info!("received UI shutdown");
                    break;
                }
                Err(oneshot::error::TryRecvError::Empty) => {}
                Err(e) => panic!("error receiving shutdown message: {e}"),
            }
            let state = match ui_update_rx.try_recv() {
                Ok(state) => state,
                Err(mpsc::error::TryRecvError::Empty) => DisplayState::Paused,
                Err(e) => {
                    error!("error receiving ui update message: {e}");
                    DisplayState::Paused
                }
            };

            if invisible {
                sleep(Duration::from_secs(1));
                continue;
            }

            match state {
                DisplayState::Paused => {
                    write(&wlan, "1").ok();
                    sleep(half_sec);
                    write(&wlan, "0").ok();
                }
                DisplayState::Recording => {
                    let leds = [&signal_blue1, &signal_red, &battery_blue1, &battery_red];
                    for led in leds {
                        write(led, "1").ok();
                    }
                    sleep(half_sec);
                    for led in leds {
                        write(led, "0").ok();
                    }
                }
                DisplayState::WarningDetected => {
                    let leds = [
                        &signal_red,
                        &signal_blue1,
                        &signal_blue2,
                        &signal_blue3,
                        &battery_red,
                        &battery_blue1,
                        &battery_blue2,
                        &battery_blue3,
                        &sms,
                    ];
                    for led in leds {
                        write(led, "1").ok();
                    }
                    write(&wlan, "0").ok();
                    sleep(half_sec);
                    write(&wlan, "0").ok();
                    for led in leds {
                        write(led, "0").ok();
                    }
                }
            }
        }
    });
}
