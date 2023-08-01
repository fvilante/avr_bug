#![no_std]
#![no_main]

use panic_halt as _;

use core::fmt::Debug;
use ufmt::uWrite;

fn print_hex_arr<S>(tag: &str, serial: &mut S, arr: &[u8])
where
    S: uWrite,
    <S as uWrite>::Error: Debug,
{
    ufmt::uwrite!(serial, "{} = ", tag).unwrap();
    for e in arr.iter() {
        ufmt::uwrite!(serial, "{:02x}", *e).unwrap();
    }
    ufmt::uwrite!(serial, "\r\n").unwrap();
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let h = hmac_sha256::HMAC::mac(b"hello", b"key");
    print_hex_arr(" mac", &mut serial, &h);
    let h = hmac_sha256::Hash::hash(b"hello");
    print_hex_arr("hash", &mut serial, &h);

    loop {}
}
