use std::slice;

use smoltcp::iface::SocketHandle;
use smoltcp::socket::tcp;
use smoltcp::wire::IpEndpoint;

use crate::address::CIPEndpoint;
use crate::device::{device_from_opaque_ptr, CDevicePtr};
use crate::result_codes::ResultCode;
use super::socket::{delete_socket, wrap_socket_handle};

pub type TCPSocketHandle = SocketHandle;

#[no_mangle]
pub extern "C" fn newTcpSocket(c_device: CDevicePtr) -> *mut TCPSocketHandle {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };

	let socket = tcp::Socket::new(
		tcp::SocketBuffer::new(vec![0; 1024]),
		tcp::SocketBuffer::new(vec![0; 1024]),
	);
	return wrap_socket_handle(c_device.sockets.add(socket));
}

#[no_mangle]
pub extern "C" fn deleteTcpSocket(c_handle: *mut TCPSocketHandle) {
	delete_socket(c_handle)
}

error_code_enum! {
	pub enum TCPListenError (tcp::ListenError) {
		InvalidState,
		Unaddressable,
	}
}

error_code_enum! {
	pub enum TCPConnectError (tcp::ConnectError) {
		InvalidState,
		Unaddressable,
	}
}

error_code_enum! {
	pub enum TCPSendError (tcp::SendError) {
		InvalidState,
	}
}

error_code_enum! {
	pub enum TCPRecvError (tcp::RecvError) {
		InvalidState,
		Finished,
	}
}


#[no_mangle]
pub extern "C" fn tcpConnect(
	c_device: CDevicePtr,
	c_handle: *mut TCPSocketHandle,
	c_endpoint: CIPEndpoint,
	local_port: u16,
) -> TCPConnectError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle: TCPSocketHandle = unsafe { *c_handle.as_ref().unwrap() };
	let socket = c_device.sockets.get_mut::<tcp::Socket>(handle);

	let cx = c_device.iface.context();
	socket.connect(
		cx,
		IpEndpoint::from(c_endpoint),
		local_port,
	).into()
}

#[no_mangle]
pub extern "C" fn tcpSend(
	c_device: CDevicePtr,
	c_handle: *mut TCPSocketHandle,
	data: *const u8,
	size: u32,
) {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let socket = c_device.sockets.get_mut::<tcp::Socket>(handle);

	let slc = unsafe { slice::from_raw_parts(data, size as usize) };
	socket.send_slice(slc.as_ref()).expect("cannot send");
}

#[no_mangle]
pub extern "C" fn tcpReceive(
	c_device: CDevicePtr,
	c_handle: *mut TCPSocketHandle,
	data: *mut u8,
	size: u32,
) {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let socket = c_device.sockets.get_mut::<tcp::Socket>(handle);

	let slc = unsafe { slice::from_raw_parts_mut(data, size as usize) };

	socket
		.recv(|data| {
			let len = data.len();
			let (left, _right) = slc.split_at_mut(len);
			left.copy_from_slice(data);
			(left.len(), ())
		})
		.unwrap();

	//	let socket = c_handle.as_ref().unwrap();
	//let (payload, _) = socket.recv().unwrap();
}

#[no_mangle]
pub extern "C" fn tcpListen(c_device: CDevicePtr, c_handle: *mut TCPSocketHandle, port: u16) {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let socket = c_device.sockets.get_mut::<tcp::Socket>(handle);

	socket.listen(port).unwrap();
}

#[no_mangle]
pub extern "C" fn tcpIsActive(c_device: CDevicePtr, c_handle: *mut TCPSocketHandle) -> bool {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let socket = c_device.sockets.get_mut::<tcp::Socket>(handle);

	return socket.is_active();
}
