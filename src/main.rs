//! SigmaOS-cups-pk-helper — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_cups_pk_helper_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-cups-pk-helper: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-cups-pk-helper: ok");
}
