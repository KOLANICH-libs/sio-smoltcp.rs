//use smoltcp::iface::interface::InterfaceInner;

/*
#[no_mangle]
pub extern "C" fn newDHCPSocket(){
	let mut dhcp_socket = dhcpv4::Socket::new();
}

#[no_mangle]
pub extern "C" fn DHCPSOcketSetLeaseDuration(){
	dhcp_socket.set_max_lease_duration(Some(Duration::from_secs(10)));
}

#[no_mangle]
pub extern "C" fn DHCPSocketPoll(){
	let event = sockets.get_mut::<dhcpv4::Socket>(dhcp_handle).poll();
	match event {
		None => {}
		Some(dhcpv4::Event::Configured(config)) => {
			config.address);
			set_ipv4_addr(&mut iface, config.address);
			config.router
			config.dns_servers.iter().enumerate()
		}
		Some(dhcpv4::Event::Deconfigured) => {
			set_ipv4_addr(&mut iface, Ipv4Cidr::new(Ipv4Address::UNSPECIFIED, 0));
			iface.routes_mut().remove_default_ipv4_route();
		}
	}
}
*/
