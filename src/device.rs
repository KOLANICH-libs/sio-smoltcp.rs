use std::collections::VecDeque;

use std::vec::Vec;

use smoltcp::phy::{self, Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;
use smoltcp::wire::EthernetFrame;
use smoltcp::Result;

/// A queue for "sent" and "received" data
pub type SansIOQueue = VecDeque<Vec<u8>>;

/// A "device" that just allows to get frames as memory buffers.
#[derive(Debug)]
pub struct SansIO {
	medium: Medium,
	mtu: usize,
	pub tx: SansIOQueue,
	pub rx: SansIOQueue,
}

impl SansIO {
	pub fn new(mut mtu: usize, medium: Medium) -> SansIO {
		if medium == Medium::Ethernet {
			// SIOCGIFMTU returns the IP MTU (typically 1500 bytes.)
			// smoltcp counts the entire Ethernet packet in the MTU, so add the Ethernet header size to it.
			mtu += EthernetFrame::<&[u8]>::header_len()
		}

		SansIO {
			medium,
			mtu,
			tx: SansIOQueue::default(),
			rx: SansIOQueue::default(),
		}
	}
}

impl<'a> Device<'a> for SansIO {
	type RxToken = RxToken;
	type TxToken = TxToken<'a>;

	fn capabilities(&self) -> DeviceCapabilities {
		let mut res = DeviceCapabilities::default();
		res.max_transmission_unit = self.mtu;
		res.medium = self.medium;
		return res;
	}

	fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
		let item = self.rx.pop_back();
		eprintln!("item {:?}", item);
		match item {
			Some(el) => {
				eprintln!("el {:?}", el);
				let rx = RxToken { buffer: el };
				let tx = TxToken { parent: self };
				Some((rx, tx))
			}
			None => None,
		}
	}

	fn transmit(&'a mut self) -> Option<Self::TxToken> {
		// lower: Rc<RefCell<sys::TunTapInterfaceDesc>>,
		// Some(TxToken {lower: self.lower.clone(),})

		Some(TxToken { parent: self })
	}
}

#[doc(hidden)]
pub struct RxToken {
	buffer: Vec<u8>,
}

impl phy::RxToken for RxToken {
	fn consume<R, F>(mut self, _timestamp: Instant, f: F) -> Result<R>
	where
		F: FnOnce(&mut [u8]) -> Result<R>,
	{
		f(&mut self.buffer[..])
	}
}

#[doc(hidden)]
pub struct TxToken<'a> {
	parent: &'a mut SansIO,
}

impl<'a> phy::TxToken for TxToken<'a> {
	fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> Result<R>
	where
		F: FnOnce(&mut [u8]) -> Result<R>,
	{
		let parent = self.parent;
		let mut buffer = vec![0; len];
		let result = f(&mut buffer);
		parent.tx.push_back(buffer[..].to_vec());
		result
	}
}
