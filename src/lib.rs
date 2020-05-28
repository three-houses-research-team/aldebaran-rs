#![feature(proc_macro_hygiene)]

use skyline::{c_str, hook, install_hook};
mod forge;
use lazy_static;

// Required to prevent Skyline from crashing the game
#[hook(offset = 0xAFCCF0)]
fn socket_stub() -> u32 {
    0
}

// Game::Menu::TitleScreen::GetVersionString()
#[hook(offset = 0x3E63E0)]
fn get_version_string() -> *const u64 {
    c_str(&format!("Aldebaran v{}\0", env!("CARGO_PKG_VERSION"))) as _
}

#[skyline::main(name = "aldebaran")]
pub unsafe fn main() {
    println!(
        "Aldebaran-rs v{} - Fire Emblem Three Houses file replacement plugin",
        env!("CARGO_PKG_VERSION")
    );

    // Kill the original nn::os::SocketInitialize method
    install_hook!(socket_stub);

    // Replace the version string on the title screen
    install_hook!(get_version_string);

    // Intercept EntryId loadings to replace files on the fly
    lazy_static::initialize(&forge::FORGE);
}
