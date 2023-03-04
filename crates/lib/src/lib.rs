#![allow(dead_code)]

use hidapi::HidApi;

const SIZE: usize = 25;

const OFF: [u8; 24] = [0x00; 24];
pub const RED: [u8; 3] = [0x00, 0xff, 0x00];
pub const GREEN: [u8; 3] = [0xff, 0x00, 0x00];
pub const BLUE: [u8; 3] = [0x00, 0x00, 0xff];
pub const WHITE: [u8; 3] = [0xff, 0xff, 0xff];

pub fn cam(buf: &mut [u8], color: &[u8]) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            color as *const _ as *const u8,
            &mut buf[16..] as *mut _ as *mut u8,
            3,
        );
        std::ptr::copy_nonoverlapping(
            color as *const _ as *const u8,
            &mut buf[19..] as *mut _ as *mut u8,
            3,
        );
    };
}
pub fn mike(buf: &mut [u8], color: &[u8]) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            color as *const _ as *const u8,
            &mut buf[4..] as *mut _ as *mut u8,
            3,
        );
        std::ptr::copy_nonoverlapping(
            color as *const _ as *const u8,
            &mut buf[7..] as *mut _ as *mut u8,
            3,
        );
    };
}
pub fn warning(buf: &mut [u8], color: &[u8]) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            color as *const _ as *const u8,
            &mut buf[1..] as *mut _ as *mut u8,
            3,
        );
        std::ptr::copy_nonoverlapping(
            color as *const _ as *const u8,
            &mut buf[10..] as *mut _ as *mut u8,
            3,
        );
        std::ptr::copy_nonoverlapping(
            color as *const _ as *const u8,
            &mut buf[13..] as *mut _ as *mut u8,
            3,
        );
        std::ptr::copy_nonoverlapping(
            color as *const _ as *const u8,
            &mut buf[22..] as *mut _ as *mut u8,
            3,
        );
    };
}
pub fn cam_off(buf: &mut [u8]) {
    cam(buf, &OFF[..3]);
}
pub fn mike_off(buf: &mut [u8]) {
    mike(buf, &OFF[..3]);
}
pub fn warning_off(buf: &mut [u8]) {
    warning(buf, &OFF[..3]);
}
pub fn off(buf: &mut [u8]) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            &OFF as *const _ as *const u8,
            &mut buf[1..] as *mut _ as *mut u8,
            24,
        );
    }
}

type AirFn = Box<dyn FnOnce(&mut [u8])>;
pub fn air(f: AirFn) -> anyhow::Result<()> {
    let api = HidApi::new()?;

    if let Ok(device) = api.open(0x5131, 0x2007) {
        let mut buf: [u8; SIZE] = [0u8; SIZE];
        f(&mut buf);
        device.write(&buf)?;
    }

    Ok(())
}

#[macro_export]
macro_rules! air {
    ($($id:ident($($col:expr)?));*) => {
        {
            air(Box::new(|buf: &mut [u8]| {
                $(
                    $id(buf$(, &$col)?);
                )*
            }))
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        air!(
            mike(GREEN);
            //warning(&BLUE);
            warning(WHITE);
            //off();
            cam_off()
        )
    }
}
