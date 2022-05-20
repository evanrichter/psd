#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(psd) = psd::Psd::from_bytes(data) {
        let _ = psd.flatten_layers_rgba(&|(_, _)| true);
    }
});
