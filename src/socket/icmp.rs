use std::slice;

use smoltcp::iface::SocketHandle;
//use smoltcp::iface::interface::InterfaceInner;
use smoltcp::phy::ChecksumCapabilities;

use smoltcp::socket::icmp;

use smoltcp::wire::{Icmpv4DstUnreachable, Icmpv4Packet, Icmpv4Repr, Icmpv4TimeExceeded, Ipv4Repr};

use crate::address::{CAddress, CIPEndpoint};
use crate::device::device_from_opaque_ptr;
use crate::device::CDevicePtr;
use crate::result_codes::ResultCode;
use super::socket::{delete_socket, wrap_socket_handle};

pub type ICMPSocketHandle = SocketHandle;

error_code_enum! {
	/// Error returned by [`Socket::bind`]
	pub enum ICMPBindError (icmp::BindError) {
		InvalidState,
		Unaddressable,
	}
}

error_code_enum! {
	/// Error returned by [`Socket::send`]
	pub enum ICMPSendError (icmp::SendError) {
		Unaddressable,
		BufferFull,
	}
}

error_code_enum! {
	/// Error returned by [`Socket::recv`]
	pub enum ICMPRecvError (icmp::RecvError) {
		Exhausted,
	}
}

#[no_mangle]
pub extern "C" fn newIcmpSocket(c_device: CDevicePtr) -> *mut ICMPSocketHandle {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let rx = icmp::PacketBuffer::new(vec![icmp::PacketMetadata::EMPTY], vec![0; 256]);
	let tx = icmp::PacketBuffer::new(vec![icmp::PacketMetadata::EMPTY], vec![0; 256]);
	let socket = icmp::Socket::new(rx, tx);

	wrap_socket_handle(c_device.sockets.add(socket))
}

#[no_mangle]
pub extern "C" fn deleteIcmpSocket(c_handle: *mut ICMPSocketHandle) {
	delete_socket(c_handle)
}

#[repr(u8)]
pub enum ICMPEchoPacketType {
	EchoRequest = 1,
	EchoReply = 2,
}

fn icmp_echo_packet_type_from_mac_addr<'a>(
	tp: ICMPEchoPacketType,
	ident: u16,
	seq_no: u16,
	payload: *const u8,
	payload_size: u32,
) -> Icmpv4Repr<'a> {
	let payload_slc = unsafe { slice::from_raw_parts(payload, payload_size as usize) };
	match tp {
		ICMPEchoPacketType::EchoRequest => Icmpv4Repr::EchoRequest {
			ident: ident,
			seq_no: seq_no,
			data: &payload_slc,
		},
		ICMPEchoPacketType::EchoReply => Icmpv4Repr::EchoReply {
			ident: ident,
			seq_no: seq_no,
			data: &payload_slc,
		},
	}
}

fn finalize_icmp_packet<'a>(icmp_repr: Icmpv4Repr<'a>, dst: *mut u8, dst_size: u32) -> u32 {
	let dst_size = dst_size as usize;
	let needed_len = icmp_repr.buffer_len();
	if dst_size < needed_len {
		return needed_len.try_into().unwrap();
	} else {
		let mut dst_slc = unsafe { slice::from_raw_parts_mut(dst, dst_size) };
		icmp_repr.emit(
			&mut Icmpv4Packet::new_unchecked(&mut dst_slc),
			&ChecksumCapabilities::default(),
		);
		return 0;
	}
}

#[no_mangle]
pub extern "C" fn buildIcmpV4EchoPacket(
	tp: ICMPEchoPacketType,
	ident: u16,
	seq_no: u16,
	payload: *const u8,
	payload_size: u32,
	dst: *mut u8,
	dst_size: u32,
) -> u32 {
	finalize_icmp_packet(
		icmp_echo_packet_type_from_mac_addr(tp, ident, seq_no, payload, payload_size),
		dst,
		dst_size,
	)
}

/*enum_with_unknown! {
	/// Internet protocol control message subtype for type "Time Exceeded".
	pub enum TimeExceeded(u8) {
		/// TTL expired in transit
		TtlExpired  = 0,
		/// Fragment reassembly time exceeded
		FragExpired = 1
	}
}*/

#[repr(u8)]
pub enum ICMPErrorPacketType {
	DstUnreachable(Icmpv4DstUnreachable),
	TimeExceeded(Icmpv4TimeExceeded),
}

fn icmp_error_packet_type_from_mac_addr<'a>(
	tp: ICMPErrorPacketType,
	header: Ipv4Repr,
	payload: *const u8,
	payload_size: u32,
) -> Icmpv4Repr<'a> {
	let payload_slc = unsafe { slice::from_raw_parts(payload, payload_size as usize) };
	match tp {
		ICMPErrorPacketType::DstUnreachable(reason) => Icmpv4Repr::DstUnreachable {
			reason: reason,
			header: header,
			data: payload_slc,
		},
		ICMPErrorPacketType::TimeExceeded(reason) => Icmpv4Repr::TimeExceeded {
			reason: reason,
			header: header,
			data: payload_slc,
		},
	}
}

#[no_mangle]
pub extern "C" fn icmpBindAny(c_device: CDevicePtr, c_handle: *mut ICMPSocketHandle) -> ICMPBindError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let _cx = c_device.iface.context();

	let socket = c_device.sockets.get_mut::<icmp::Socket>(handle);

	if !socket.is_open() {
		socket.bind(icmp::Endpoint::Unspecified).into()
	} else {
		ICMPBindError::OK
	}
}

#[no_mangle]
pub extern "C" fn icmpBindIdent(
	c_device: CDevicePtr,
	c_handle: *mut ICMPSocketHandle,
	ident: u16,
) -> ICMPBindError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let _cx = c_device.iface.context();

	let socket = c_device.sockets.get_mut::<icmp::Socket>(handle);

	if !socket.is_open() {
		socket.bind(icmp::Endpoint::Ident(ident)).into()
	} else {
		ICMPBindError::OK
	}
}

#[no_mangle]
pub extern "C" fn icmpBindUDP(
	c_device: CDevicePtr,
	c_handle: *mut ICMPSocketHandle,
	endpoint: CIPEndpoint,
) -> ICMPBindError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	let _cx = c_device.iface.context();

	let socket = c_device.sockets.get_mut::<icmp::Socket>(handle);

	if !socket.is_open() {
		socket.bind(icmp::Endpoint::Udp(endpoint.into())).into()
	} else {
		ICMPBindError::OK
	}
}

#[no_mangle]
pub extern "C" fn icmpSend(
	c_device: CDevicePtr,
	c_handle: *mut ICMPSocketHandle,
	address: CAddress,
	data: *const u8,
	size: u32,
) -> ICMPSendError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	//let cx = c_device.iface.context();
	let slc = unsafe { slice::from_raw_parts(data, size as usize) };

	let socket = c_device.sockets.get_mut::<icmp::Socket>(handle);

	socket.send_slice(slc, address.into()).into()
}

#[no_mangle]
pub extern "C" fn icmpReceive(
	c_device: CDevicePtr,
	c_handle: *mut ICMPSocketHandle,
	address: &mut CAddress,
	dst: *mut u8,
	size: u32,
) -> ICMPRecvError {
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let handle = unsafe { *c_handle.as_ref().unwrap() };
	//let cx = c_device.iface.context();

	let socket = c_device.sockets.get_mut::<icmp::Socket>(handle);

	match socket.recv() {
		Ok((data, source_addr)) => {
			let slc = unsafe { slice::from_raw_parts_mut(dst, size as usize) };
			*address = source_addr.into();
			let len = data.len();
			let (left, _right) = slc.split_at_mut(len);
			left.copy_from_slice(data);
			ICMPRecvError::OK
		}
		Err(err) => err.into(),
	}
}

/*
	/// Create an ICMP socket with the given buffers.
	pub fn new(rx_buffer: PacketBuffer<'a>, tx_buffer: PacketBuffer<'a>) -> Socket<'a>;

	/// Return the time-to-live (IPv4) or hop limit (IPv6) value used in outgoing packets.
	///
	/// See also the [set_hop_limit](#method.set_hop_limit) method
	pub fn hop_limit(&self);
	pub fn set_hop_limit(&mut self, hop_limit: Option<u8>);

	pub fn bind<T: Into<Endpoint>>(&mut self, endpoint: T) -> Result<(), BindError>;

	/// Check whether the transmit buffer is full.
	#[inline]
	pub fn can_send(&self) -> bool;

	/// Check whether the receive buffer is not empty.
	#[inline]
	pub fn can_recv(&self) -> bool;

	/// Return the maximum number packets the socket can receive.
	#[inline]
	pub fn packet_recv_capacity(&self) -> usize;

	/// Return the maximum number packets the socket can transmit.
	#[inline]
	pub fn packet_send_capacity(&self) -> usize;

	/// Return the maximum number of bytes inside the recv buffer.
	#[inline]
	pub fn payload_recv_capacity(&self) -> usize;

	/// Return the maximum number of bytes inside the transmit buffer.
	#[inline]
	pub fn payload_send_capacity(&self) -> usize;

	/// Check whether the socket is open.
	#[inline]
	pub fn is_open(&self) -> bool;
*/

/*
#[no_mangle]
pub extern "C" fn ping(c_device: CDevicePtr, c_handle: *mut ICMPSocketHandle){
	let c_device = unsafe { device_from_opaque_ptr(c_device) };
	let c_socket = unsafe { *c_handle.as_ref().unwrap() };
	let socket = c_device.sockets.get_mut::<icmp::Socket>(c_socket.handle);

	let icmp_payload = socket.send(icmp_repr.buffer_len(), remote_addr).unwrap();
	let icmp_packet = $packet_type::new_unchecked(icmp_payload);
}

*/
