use log::{error, info};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;
use tokio_util::task::TaskTracker;

use crate::config;
use crate::display::DisplayState;

use std::thread::sleep;
use std::time::Duration;

pub fn update_ui(
    task_tracker: &TaskTracker,
    _config: &config::Config,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    info!("Dummy mode, not spawning UI.");
    return;
    task_tracker.spawn_blocking(move || loop {
        match ui_shutdown_rx.try_recv() {
            Ok(_) => {
                info!("received UI shutdown");
                break;
            }
            Err(TryRecvError::Empty) => {}
            Err(e) => panic!("error receiving shutdown message: {e}"),
        }
        match ui_update_rx.try_recv() {
            Err(e) => {
                error!("error receiving ui update message: {e}");
            }
            _ => ()
        }
        sleep(Duration::from_secs(1));
    });
}
