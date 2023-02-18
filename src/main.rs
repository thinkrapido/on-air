
#![allow(dead_code)]

use hidapi::HidApi;

const SIZE: usize = 25;

const RED:      [u8;  3] = [0x00, 0xff, 0x00];
const GREEN:    [u8;  3] = [0xff, 0x00, 0x00];
const BLUE:     [u8;  3] = [0x00, 0x00, 0xff];
const WHITE:    [u8;  3] = [0xff, 0xff, 0xff];

fn main() -> anyhow::Result<()> {

    let api = HidApi::new()?;
    let device = api.open(0x5131, 0x2007)?;

    let mut buf: [u8; SIZE] = [0u8; SIZE];
    
    cam(&mut buf, &RED);
    mike(&mut buf, &GREEN);
    //warning(&mut buf, &BLUE);
    warning(&mut buf, &WHITE);
    //off(&mut buf);

    device.write(&buf)?;

    Ok(())
}

fn cam(buf: &mut [u8], color: &[u8]) {
    unsafe { 
        std::ptr::copy_nonoverlapping(color as *const _ as *const u8, &mut buf[16..] as *mut _ as *mut u8, 3);
        std::ptr::copy_nonoverlapping(color as *const _ as *const u8, &mut buf[19..] as *mut _ as *mut u8, 3); 
    };
}
fn mike(buf: &mut [u8], color: &[u8]) {
    unsafe { 
        std::ptr::copy_nonoverlapping(color as *const _ as *const u8, &mut buf[4..] as *mut _ as *mut u8, 3);
        std::ptr::copy_nonoverlapping(color as *const _ as *const u8, &mut buf[7..] as *mut _ as *mut u8, 3); 
    };
}
fn warning(buf: &mut [u8], color: &[u8]) {
    unsafe { 
        std::ptr::copy_nonoverlapping(color as *const _ as *const u8, &mut buf[1..] as *mut _ as *mut u8, 3);
        std::ptr::copy_nonoverlapping(color as *const _ as *const u8, &mut buf[10..] as *mut _ as *mut u8, 3); 
        std::ptr::copy_nonoverlapping(color as *const _ as *const u8, &mut buf[13..] as *mut _ as *mut u8, 3);
        std::ptr::copy_nonoverlapping(color as *const _ as *const u8, &mut buf[22..] as *mut _ as *mut u8, 3); 
    };
}
fn off(buf: &mut [u8]) {
    const OFF:      [u8; 24] = [0x00; 24];
    unsafe {
        std::ptr::copy_nonoverlapping(&OFF as *const _ as *const u8, &mut buf[1..] as *mut _ as *mut u8, 24);
    }
}



