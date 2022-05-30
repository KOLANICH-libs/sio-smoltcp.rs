use smoltcp::wire::{
	EthernetAddress, IpAddress, IpCidr, IpEndpoint, IpListenEndpoint, Ipv4Address, Ipv6Address,
};

#[repr(C)]
pub struct CMacAddress {
	pub mac: [::std::os::raw::c_uchar; 6],
}

impl From<CMacAddress> for EthernetAddress {
	fn from(address: CMacAddress) -> Self {
		return EthernetAddress(address.mac);
	}
}

#[repr(C)]
pub struct CAddress {
	pub ip: [::std::os::raw::c_uchar; 16],
}

#[repr(C)]
pub struct CInterface {
	pub prefix: u8,
	pub addr: CAddress,
}

static IPV4_PREFIX: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255];

impl From<CAddress> for Ipv4Address {
	fn from(address: CAddress) -> Self {
		Ipv6Address::from(address).as_ipv4().unwrap()
	}
}

impl From<CAddress> for Ipv6Address {
	fn from(address: CAddress) -> Self {
		Ipv6Address::from_bytes(&address.ip)
	}
}

impl From<CAddress> for IpAddress {
	fn from(address: CAddress) -> Self {
		let ipv6: Ipv6Address = address.into();
		match ipv6.as_ipv4() {
			Some(ipv4) => IpAddress::Ipv4(ipv4),
			None => IpAddress::Ipv6(ipv6),
		}
	}
}

impl From<IpAddress> for CAddress {
	fn from(address: IpAddress) -> Self {
		let mut b = [0_u8; 16];
		match address {
			IpAddress::Ipv6(addr) => {
				b.copy_from_slice(addr.as_bytes());
			}
			IpAddress::Ipv4(addr) => {
				b.copy_from_slice(Ipv6Address::from(addr).as_bytes());
			}
		}
		Self { ip: b }
	}
}

impl From<CInterface> for IpCidr {
	fn from(ifc: CInterface) -> Self {
		let addr = ifc.addr.into();
		let mut prefix = ifc.prefix;
		match addr {
			IpAddress::Ipv4(_) => prefix -= (IPV4_PREFIX.len() * 8) as u8,
			IpAddress::Ipv6(_) => {}
		}
		return IpCidr::new(addr, prefix);
	}
}

#[repr(C)]
pub struct CIPEndpoint {
	pub port: u16,
	pub addr: CAddress,
}

impl From<CIPEndpoint> for IpEndpoint {
	fn from(ep: CIPEndpoint) -> Self {
		return Self {
			addr: ep.addr.into(),
			port: ep.port,
		};
	}
}

impl From<IpEndpoint> for CIPEndpoint {
	fn from(ep: IpEndpoint) -> Self {
		Self {
			addr: ep.addr.into(),
			port: ep.port,
		}
	}
}

fn optional_ip_addr_from_possibly_unspecified_ip_addr(addr: IpAddress) -> Option<IpAddress> {
	if addr.is_unspecified() {
		None
	} else {
		Some(addr)
	}
}

impl From<CIPEndpoint> for IpListenEndpoint {
	fn from(ep: CIPEndpoint) -> Self {
		return Self {
			addr: optional_ip_addr_from_possibly_unspecified_ip_addr(ep.addr.into()),
			port: ep.port,
		};
	}
}

impl From<IpListenEndpoint> for CIPEndpoint {
	fn from(ep: IpListenEndpoint) -> Self {
		return Self {
			addr: match ep.addr {
				Some(addr) => addr.into(),
				None => CAddress { ip: [0; 16] },
			},
			port: ep.port,
		};
	}
}
