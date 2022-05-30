use std::slice;

use smoltcp::iface::SocketHandle;
//use smoltcp::iface::interface::InterfaceInner;

use smoltcp::socket::dns;

use super::socket::{delete_socket, wrap_socket_handle};
use crate::address::CAddress;
use crate::cdevice::device_from_opaque_ptr;
use crate::cdevice::CDevicePtr;
use smoltcp::wire::{DnsQueryType, IpAddress, Ipv4Address};

use crate::result_codes::ResultCode;

pub type DNSSocketHandle = SocketHandle;

error_code_enum! {
	/// Error returned by [`Socket::start_query`]
	pub enum DNSStartQueryError (dns::StartQueryError) {
		NoFreeSlot,
		InvalidName,
		NameTooLong,
	}
}

error_code_enum! {
	/// Error returned by [`Socket::get_query_result`]
	pub enum DNSGetQueryResultError (dns::GetQueryResultError) {
		/// Query is not done yet.
		Pending,
		/// Query failed.
		Failed,
	}
}

#[no_mangle]
pub extern "C" fn newDnsSocket(c_device: CDevicePtr, address: CAddress) -> *mut DNSSocketHandle {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let socket = dns::Socket::new(&[IpAddress::from(Ipv4Address::from(address))], vec![]);
	let handle = c_device.sockets.add(socket);

	wrap_socket_handle(handle)
}

#[no_mangle]
pub extern "C" fn deleteDnsSocket(c_handle: *mut DNSSocketHandle) {
	delete_socket(c_handle)
}

#[no_mangle]
pub extern "C" fn newDnsQuery(
	c_device: CDevicePtr,
	c_handle: *mut DNSSocketHandle,
	name: *const u8,
	name_size: u32,
) -> *const dns::QueryHandle {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let cx = c_device.iface.context();

	let name = unsafe { slice::from_raw_parts(name, name_size as usize) };
	match String::from_utf8(name.to_vec()) {
		Ok(name) => {
			let socket = c_device.sockets.get_mut::<dns::Socket>(handle);

			match socket.start_query(cx, &name, DnsQueryType::A) {
				Ok(query) => {
					let query_box = Box::new(query);
					return Box::into_raw(query_box).cast();
				}
				Err(_err) => {
					return 0 as *const dns::QueryHandle;
				}
			}
		}
		Err(_err) => {
			return 0 as *const dns::QueryHandle;
		}
	}
}
/*
#[no_mangle]
pub extern "C" fn checkDnsQueryResult(c_device: CDevicePtr, c_handle: *mut DNSSocketHandle, dnsQueryPtr: *const dns::QueryHandle){
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { c_handle.as_ref().unwrap() };
	let query = unsafe { dnsQueryPtr.as_ref().unwrap() };
	let cx = c_device.iface.context();

	let socket = c_device.sockets.get_mut::<dns::Socket>(handle.handle);

	match socket.get_query_result(query){
		Some(results) => {

		}
		Err(err) => {

		}
	}
}
*/
