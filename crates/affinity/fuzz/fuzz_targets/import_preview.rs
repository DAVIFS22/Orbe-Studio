#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // The complete import: native document mapping, or the preview fallback.
    if orbe_affinity::is_affinity(data) {
        let _ = orbe_io::import("fuzz.af", data);
    }
});
