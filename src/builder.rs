use core::ffi::c_void;

use smoltcp::iface::ReassemblyBuffer;
use smoltcp::iface::InterfaceBuilder;
use smoltcp::iface::NeighborCache;
use smoltcp::iface::Routes;
use smoltcp::iface::SocketSet;
use smoltcp::time::Instant;
use smoltcp::wire::{HardwareAddress::Ethernet, IpCidr, Ipv4Address};
use std::collections::BTreeMap;

use crate::address::CAddress;
use crate::address::CInterface;
use crate::address::CMacAddress;
use crate::device::CDevice;
use crate::device::CDevicePtr;
use crate::device::CMedium;
use crate::device::SansIO;



pub type CBuilderPtr = *mut c_void;

pub unsafe fn builder_from_opaque_ptr<'b, 'a>(
	c_builder: CBuilderPtr,
) -> &'b mut InterfaceBuilder<'a> {
	if c_builder.is_null() {
		panic!("Fatal error, got NULL `InterfaceBuilder` pointer");
	}
	&mut *(c_builder.cast())
}

pub fn box_builder(b: InterfaceBuilder) -> CBuilderPtr {
	return Box::into_raw(Box::new(b)).cast();
}

#[no_mangle]
pub extern "C" fn newBuilder() -> CBuilderPtr {
	return box_builder(InterfaceBuilder::new());
}

#[no_mangle]
pub extern "C" fn deleteBuilder(c_builder: CBuilderPtr) {
	unsafe { drop::<Box<InterfaceBuilder<'_>>>(Box::from_raw(builder_from_opaque_ptr(c_builder))) }
}

#[no_mangle]
pub extern "C" fn builderSetHardwareAddr(c_builder: CBuilderPtr, mac: CMacAddress) -> CBuilderPtr {
	let mut builder_box = unsafe { Box::from_raw(builder_from_opaque_ptr(c_builder)) };
	*builder_box = (*builder_box).hardware_addr(Ethernet(mac.into()));
	return Box::into_raw(builder_box).cast();
}

#[no_mangle]
pub extern "C" fn builderInitNeighbourCache(c_builder: CBuilderPtr) -> CBuilderPtr {
	let mut builder_box = unsafe { Box::from_raw(builder_from_opaque_ptr(c_builder)) };
	let neighbor_cache = NeighborCache::new(BTreeMap::new()); // [None; 8]
	*builder_box = (*builder_box).neighbor_cache(neighbor_cache);
	return Box::into_raw(builder_box).cast();
}

/*
#[no_mangle]
pub extern "C" fn builderInitSixlowpan(c_builder: CBuilderPtr) -> CBuilderPtr{
	let mut builder_box = unsafe { Box::from_raw(builder_from_opaque_ptr(c_builder)) };
	let mut out_packet_buffer = [0u8; 1280];
	let sixlowpan_frag_cache = ReassemblyBuffer::new(vec![], BTreeMap::new());
	*builder_box = (*builder_box)
	.sixlowpan_fragments_cache(sixlowpan_frag_cache)
	.sixlowpan_out_packet_cache(&mut out_packet_buffer[..]);
	return Box::into_raw(builder_box).cast();
}
*/

#[no_mangle]
pub extern "C" fn builderInitIPv4ReassemblyBuffer(c_builder: CBuilderPtr) -> CBuilderPtr {
	let mut builder_box = unsafe { Box::from_raw(builder_from_opaque_ptr(c_builder)) };
	let ipv4_frag_cache = ReassemblyBuffer::new(vec![], BTreeMap::new());
	*builder_box = (*builder_box).ipv4_reassembly_buffer(ipv4_frag_cache);
	return Box::into_raw(builder_box).cast();
}

#[no_mangle]
pub extern "C" fn builderSetIPAddr(c_builder: CBuilderPtr, my_ip: CInterface) -> CBuilderPtr {
	let mut builder_box = unsafe { Box::from_raw(builder_from_opaque_ptr(c_builder)) };
	let ifc = IpCidr::from(my_ip);
	let ip_addrs = [ifc];
	*builder_box = (*builder_box).ip_addrs(ip_addrs);
	return Box::into_raw(builder_box).cast();
}

#[no_mangle]
pub extern "C" fn builderSetRoutes(c_builder: CBuilderPtr, gateway_ip: CAddress) -> CBuilderPtr {
	let mut builder_box = unsafe { Box::from_raw(builder_from_opaque_ptr(c_builder)) };
	let default_v4_gw = Ipv4Address::from(gateway_ip);
	let mut routes_storage = [None; 2];
	let mut routes = Routes::new(&mut routes_storage[..]);
	routes.add_default_ipv4_route(default_v4_gw).unwrap();
	//routes.add_default_ipv6_route(default_v6_gw).unwrap();
	*builder_box = (*builder_box).routes(routes);
	return Box::into_raw(builder_box).cast();
}

#[no_mangle]
pub extern "C" fn builderFinalize(
	c_builder: CBuilderPtr,
	medium: CMedium,
	mtu: usize,
) -> CDevicePtr {
	let builder_box = unsafe { Box::from_raw(builder_from_opaque_ptr(c_builder)) };
	let mut device = SansIO::new(mtu, medium.into());
	let iface = (*builder_box).finalize(&mut device);
	let c_dev = CDevice {
		device: device,
		timestamp: Instant::now(),
		sockets: SocketSet::new(vec![]),
		iface: iface,
	};

	return Box::into_raw(Box::new(c_dev)).cast();
}
