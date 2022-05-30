use smoltcp::wire::Ipv4Packet;

pub fn fragment_ipv4_packet(data: &mut [u8], mtu: usize) -> Vec<Vec<u8>> {
	let packet = Ipv4Packet::new_unchecked(&data);
	let header_size = packet.header_len() as usize;
	let header = &data[..header_size];

	let chunks = packet.payload().chunks(mtu - header_size);
	let chunks_len = chunks.len();
	let mut result = Vec::<Vec<u8>>::with_capacity(chunks_len);

	//smoltcp::wire::ipv4::Repr
	for (i, chunk) in chunks.enumerate() {
		let mut new_packet_bytes = vec![0_u8; mtu + header.len()];
		new_packet_bytes.copy_from_slice(&header);
		new_packet_bytes[header_size..].copy_from_slice(&chunk);
		let mut new_packet = Ipv4Packet::new_unchecked(&mut new_packet_bytes);
		new_packet.set_dont_frag(false);
		new_packet.set_more_frags(i != chunks_len - 1);
		new_packet.fill_checksum();
		let new_packet_bytes = new_packet_bytes;
		result.push(new_packet_bytes);
	}

	result
}
