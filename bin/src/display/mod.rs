#[cfg(not(any(feature = "alcatel", feature = "dummy")))]
mod generic_framebuffer;

#[cfg(feature = "dummy")]
mod dummy;
#[cfg(feature = "dummy")]
pub use dummy::update_ui;

#[cfg(feature = "alcatel")]
mod alcatel;
#[cfg(feature = "alcatel")]
pub use alcatel::update_ui;

#[cfg(feature = "tplink")]
mod tplink;
#[cfg(feature = "tplink")]
mod tplink_framebuffer;
#[cfg(feature = "tplink")]
mod tplink_onebit;
#[cfg(feature = "tplink")]
pub use tplink::update_ui;

#[cfg(feature = "orbic")]
mod orbic;
#[cfg(feature = "orbic")]
pub use orbic::update_ui;

pub enum DisplayState {
    Recording,
    Paused,
    WarningDetected,
}

#[cfg(all(feature = "orbic", feature = "alcatel"))]
compile_error!("cannot compile for many devices at once");

#[cfg(all(feature = "orbic", feature = "tplink"))]
compile_error!("cannot compile for many devices at once");

#[cfg(all(feature = "tplink", feature = "alcatel"))]
compile_error!("cannot compile for many devices at once");

#[cfg(not(any(
    feature = "orbic",
    feature = "tplink",
    feature = "alcatel",
    feature = "dummy"
)))]
compile_error!("cannot compile for no device at all");
