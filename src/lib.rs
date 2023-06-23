use sacn_unofficial::packet::ACN_SDT_MULTICAST_PORT;
use sacn_unofficial::source::SacnSource;
use std::net::{IpAddr, SocketAddr};

const SYNC_UNI: Option<u16> = None;

pub struct LedStrip {
    src: SacnSource,
    dst_ip: Option<SocketAddr>,
    universe: u16,
    priority: u8,
}

impl LedStrip {
    pub fn new(ip: IpAddr, universe: u16, priority: u8) -> Self {
        let destination_address: SocketAddr = SocketAddr::new(ip, ACN_SDT_MULTICAST_PORT);

        let dst_ip: Option<SocketAddr> = Some(destination_address);

        let local_addr: SocketAddr = SocketAddr::new(
            IpAddr::V4("0.0.0.0".parse().unwrap()),
            ACN_SDT_MULTICAST_PORT + 1,
        );

        let mut src: SacnSource = SacnSource::with_ip("Source", local_addr).unwrap();

        src.register_universe(universe).unwrap(); // Register with the source that will be sending on the given universe.
        Self {
            src,
            dst_ip,
            universe,
            priority,
        }
    }

    pub fn send(&mut self, data: Vec<u8>) {
        self.src
            .send(
                &[self.universe],
                &data,
                Some(self.priority),
                self.dst_ip,
                SYNC_UNI,
            )
            .unwrap(); // Actually send the data
    }
}
