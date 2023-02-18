use rusb::{DeviceList, GlobalContext, DeviceHandle};





fn main() -> anyhow::Result<()> {
    
    let device_list = DeviceList::new()?;

    for device in device_list.iter() {

        let descriptor = device.device_descriptor()?;
        if let (0x5131, 0x2007) = (descriptor.vendor_id(), descriptor.product_id()) {

            let handle = device.open()?;

            // println!("read");
            // read(&handle, SIZE)?;
            // for i in 0..SIZE {
            //     match read(&handle, i) {
            //         Ok(_) => println!("success: {i}"),
            //         Err(_) => print!("{i}."),
            //     };
            // }

            println!("write");
            //write(&handle, SIZE)?;
            for i in 0..SIZE {
                match write(&handle, i) {
                    Ok(_) => println!("success: {i}"),
                    Err(err) => println!("{i}. {err:?}"),
                };
            }

            println!("finished");

            break;
        }

    }

    Ok(())
}

const SIZE: usize = 64;

fn read(handle: &DeviceHandle<GlobalContext>, i: usize) -> anyhow::Result<()> {

    let endpoint = 0x82u8;
    let mut buf: [u8; SIZE] = [0; SIZE];

    let bytes_read = handle.read_bulk(
        endpoint, 
        &mut buf[..i], 
        std::time::Duration::from_secs(1))?;

    println!("bytes read {bytes_read}");

    Ok(())
}

fn write(handle: &DeviceHandle<GlobalContext>, i: usize) -> anyhow::Result<()> {

    let endpoint = 0x02u8;
    let buf: [u8; SIZE] = [0u8; SIZE];

    let bytes_written = handle.write_interrupt(
        endpoint, 
        &buf[..i], 
        std::time::Duration::from_secs(1))?;

    println!("bytes written {bytes_written}");

    Ok(())
}

