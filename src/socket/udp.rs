use std::slice;

use smoltcp::iface::SocketHandle;
use smoltcp::socket::udp;
use smoltcp::wire::IpEndpoint;

use crate::address::CIPEndpoint;
use crate::cdevice::device_from_opaque_ptr;
use crate::cdevice::CDevicePtr;

use super::socket::{delete_socket, wrap_socket_handle};
use crate::result_codes::ResultCode;

pub type UDPSocketHandle = SocketHandle;

#[no_mangle]
pub extern "C" fn newUdpSocket(c_device: CDevicePtr) -> *mut UDPSocketHandle {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };

	let rx = udp::PacketBuffer::new(vec![udp::PacketMetadata::EMPTY], vec![0; 65535]);
	let tx = udp::PacketBuffer::new(vec![udp::PacketMetadata::EMPTY], vec![0; 65535]);
	let socket = udp::Socket::new(rx, tx);

	return wrap_socket_handle(c_device.sockets.add(socket));
}

#[no_mangle]
pub extern "C" fn deleteUdpSocket(c_handle: *mut UDPSocketHandle) {
	delete_socket(c_handle)
}

error_code_enum! {
	/// Error returned by [`Socket::bind`]
	pub enum UDPBindError (udp::BindError) {
		InvalidState,
		Unaddressable,
	}
}

error_code_enum! {
	/// Error returned by [`Socket::send`]
	pub enum UDPSendError (udp::SendError) {
		Unaddressable,
		BufferFull,
	}
}

error_code_enum! {
	/// Error returned by [`Socket::recv`]
	pub enum UDPRecvError (udp::RecvError) {
		Exhausted,
	}
}

#[no_mangle]
pub extern "C" fn udpBind(
	c_device: CDevicePtr,
	c_handle: *mut UDPSocketHandle,
	port: u16,
) -> UDPBindError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let _cx = c_device.iface.context();

	let socket = c_device.sockets.get_mut::<udp::Socket>(handle);

	if !socket.is_open() {
		socket.bind(port).into()
	} else {
		UDPBindError::OK
	}
}

#[no_mangle]
pub extern "C" fn udpGetLastReceivedPacketSize(
	c_device: CDevicePtr,
	c_handle: *mut UDPSocketHandle,
) -> u32 {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	//let cx = c_device.iface.context();

	let socket = c_device.sockets.get_mut::<udp::Socket>(handle);

	match socket.peek() {
		Ok((data, _source_endpoint)) => {
			return data.len() as u32;
		}
		Err(_) => {
			return 0;
		}
	};
}

#[no_mangle]
pub extern "C" fn udpReceive(
	c_device: CDevicePtr,
	c_handle: *mut UDPSocketHandle,
	endpoint: &mut CIPEndpoint,
	dst: *mut u8,
	size: u32,
) -> UDPRecvError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	//let cx = c_device.iface.context();

	let socket = c_device.sockets.get_mut::<udp::Socket>(handle);

	match socket.recv() {
		Ok((data, source_endpoint)) => {
			let slc = unsafe { slice::from_raw_parts_mut(dst, size as usize) };
			*endpoint = source_endpoint.into();
			let len = data.len();
			let (left, _right) = slc.split_at_mut(len);
			left.copy_from_slice(data);
			UDPRecvError::OK
		}
		Err(err) => err.into(),
	}
}

#[no_mangle]
pub extern "C" fn udpSend(
	c_device: CDevicePtr,
	c_handle: *mut UDPSocketHandle,
	endpoint: CIPEndpoint,
	data: *const u8,
	size: u32,
) -> UDPSendError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	//let cx = c_device.iface.context();
	let slc = unsafe { slice::from_raw_parts(data, size as usize) };

	let socket = c_device.sockets.get_mut::<udp::Socket>(handle);

	let endpoint: IpEndpoint = endpoint.into();
	socket.send_slice(slc, endpoint).into()
}

//	let socket = *c_handle.as_ref().unwrap();
//let (payload, _) = socket.recv().unwrap();

/*
//use crate::wire::{IpEndpoint, IpListenEndpoint, IpProtocol, IpRepr, UdpRepr};
/// BindError
socket.send_slice(b"abcdef", REMOTE_END),
assert_eq!(socket.bind(LOCAL_PORT), Ok(()));
assert_eq!(
	socket.send_slice(
		b"abcdef",
		IpEndpoint {
			addr: IpvXAddress::UNSPECIFIED.into(),
			..REMOTE_END
		}
	),
	Err(SendError::Unaddressable)
);
assert_eq!(
	socket.send_slice(
		b"abcdef",
		IpEndpoint {
			port: 0,
			..REMOTE_END
		}
	),
	Err(SendError::Unaddressable)
);
assert_eq!(socket.send_slice(b"abcdef", REMOTE_END), Ok(()));

const REMOTE_END: IpEndpoint = IpEndpoint {
		addr: REMOTE_ADDR.into_address(),
		port: REMOTE_PORT,
	};

socket.dispatch(&mut cx, |_, (ip_repr, udp_repr, payload)| {
				assert_eq!(ip_repr, LOCAL_IP_REPR);
				assert_eq!(udp_repr, LOCAL_UDP_REPR);
				assert_eq!(payload, PAYLOAD);
				Err(Error::Unaddressable)
			}),
			Err(Error::Unaddressable)
		);
		assert!(!socket.can_send());

		assert_eq!(
			socket.dispatch(&mut cx, |_, (ip_repr, udp_repr, payload)| {
				assert_eq!(ip_repr, LOCAL_IP_REPR);
				assert_eq!(udp_repr, LOCAL_UDP_REPR);
				assert_eq!(payload, PAYLOAD);
				Ok::<_, Error>(())
			}),
			Ok(())
		);
SendError RecvError

let recv_buffer = PacketBuffer::new(vec![PacketMetadata::EMPTY; 1], vec![]);
		let mut socket = socket(recv_buffer, buffer(0));
		let mut cx = Context::mock();

		assert_eq!(socket.bind(LOCAL_PORT), Ok(()));

		let repr = UdpRepr {
			src_port: REMOTE_PORT,
			dst_port: LOCAL_PORT,
		};
		socket.process(&mut cx, &REMOTE_IP_REPR, &repr, &[]);
		assert_eq!(socket.recv(), Ok((&[][..], REMOTE_END)));

*/

///////////////
