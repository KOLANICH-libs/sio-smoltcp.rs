use smoltcp::iface::SocketHandle;

/// Deletes a socket using its handle. While you currently can use ths function to delete sockets, using specialized functions is more future-proof since I can imagine that we can have to store some auxillary info besides the handle.
#[no_mangle]
pub extern "C" fn delete_socket(c_handle: *mut SocketHandle) {
	unsafe {
		Box::from_raw(c_handle);
	}
}

/// Wraps a socket handle into a box and returns a pointer, so C++ can use it.
pub fn wrap_socket_handle(handle: SocketHandle) -> *mut SocketHandle {
	let c_handle_box = Box::new(handle);
	return Box::into_raw(c_handle_box).cast();
}
