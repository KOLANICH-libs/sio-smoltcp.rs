use std::ffi::c_void;

use std::slice;

use log::debug;

use smoltcp::phy::{Medium};
use smoltcp::iface::{Interface, SocketSet};
//use smoltcp::iface::interface::InterfaceInner;

use crate::device::SansIO;
use smoltcp::time::Instant;

pub struct CDevice<'a> {
	pub device: SansIO,
	pub timestamp: Instant,
	pub sockets: SocketSet<'a>,
	pub iface: Interface<'a>,
	//pub cx: &mut InterfaceInner<'a>
}

/// Type of medium of a device.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum CMedium {
	Invalid = 0,
	Ethernet = 2,
	Ip = 3,
	Ieee802154 = 4,
}

impl From<CMedium> for Medium {
	fn from(medium: CMedium) -> Self {
		match medium {
			CMedium::Invalid => {
				panic!("Invalid medium")
			}
			CMedium::Ethernet => Medium::Ethernet,
			CMedium::Ip => Medium::Ip,
			CMedium::Ieee802154 => {
				//Medium::Ieee802154
				panic!("Shared library was compiled without Ieee802154")
			}
		}
	}
}

pub type CDevicePtr = *mut c_void;

pub unsafe fn device_from_opaque_ptr<'b, 'a>(c_device: CDevicePtr) -> &'b mut CDevice<'a> {
	if c_device.is_null() {
		panic!("Fatal error, got NULL `CDevice` pointer");
	}
	&mut *(c_device.cast())
}

#[no_mangle]
pub extern "C" fn freeDevice(c_device: CDevicePtr) {
	unsafe { drop::<Box<CDevice<'_>>>(Box::from_raw(device_from_opaque_ptr(c_device))) }
}

#[no_mangle]
pub extern "C" fn getCountOfPacketsInTxQueue(c_device: CDevicePtr) -> usize {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	return c_device.device.tx.len();
}

#[no_mangle]
pub extern "C" fn getLastTxPacketSize(c_device: CDevicePtr) -> usize {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	match c_device.device.tx.get(0) {
		Some(last_packet) => {
			return last_packet.len();
		}
		None => {
			return 0;
		}
	}
}

#[no_mangle]
pub extern "C" fn getLastTxPacket(c_device: CDevicePtr, dst: *mut u8, size: u32) -> usize {
	let dst = unsafe { slice::from_raw_parts_mut(dst, size as usize) };

	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	match c_device.device.tx.pop_front() {
		Some(last_packet) => {
			dst.copy_from_slice(last_packet.as_slice());
			return last_packet.len();
		}
		None => {
			return 0;
		}
	}
}

#[no_mangle]
pub extern "C" fn putRxPacket(c_device: CDevicePtr, src: *const u8, size: u32) {
	let src = unsafe { slice::from_raw_parts(src, size as usize) };
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	c_device.device.rx.push_back(src.to_vec());
}

#[no_mangle]
pub extern "C" fn ifacePoll(c_device: CDevicePtr) {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };

	c_device.timestamp = Instant::now();
	match c_device.iface.poll(
		c_device.timestamp,
		&mut c_device.device,
		&mut c_device.sockets,
	) {
		Ok(_) => {}
		Err(e) => {
			debug!("poll error: {}", e);
		}
	}
}

/*
#[no_mangle]
pub extern "C" fn phyWait(c_device: CDevicePtr){
	let c_device = unsafe { device_from_opaque_ptr(c_device) };

	phy_wait(fd, c_device.iface.poll_delay(c_device.timestamp, &c_device.sockets)).expect("wait error");
}
*/ 
