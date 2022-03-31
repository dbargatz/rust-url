#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate idna;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = idna::punycode::encode_str(s);
    }
});
