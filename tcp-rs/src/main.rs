use std::io;

use tun_tap::Iface;

fn main() -> io::Result<()> {
    let nic = Iface::new("tun0",tun_tap::Mode::Tun)?;
    let mut buf = [0u8;1504];
    let nbytes = nic.recv(&mut buf)?;
    eprint!("Received {} bytes: {:x?}",nbytes,&buf[..nbytes]);
    Ok(())
}
