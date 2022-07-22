use hex::encode;

fn main() {
    // get the default Device
    let device = pcap::Device::lookup()
        .expect("device lookup failed");
    println!("Using device {}", device.name);

    // Setup Capture
    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    cap.filter("host login-universe.flyff.com", true).unwrap();

    while let Ok(packet) = cap.next() {
       
        println!("got packet! {:?}", encode(packet.data));
    }
}
