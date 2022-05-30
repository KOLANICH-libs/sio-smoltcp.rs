import typing
from ipaddress import IPv4Address, IPv4Interface, IPv4Network, IPv6Address, IPv6Interface, IPv6Network, _BaseAddress, _BaseNetwork
from struct import Struct

import netaddr
from netaddr.eui import EUI

ipv4Prefix = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff"

singleU4LE = Struct("<I")

InterfaceT = typing.Union[IPv4Interface, IPv6Interface]


def netMaskV4IntoPrefixLengthFromBytes(d: bytes) -> int:
	return singleU4LE.unpack(d)[0].bit_length()


def ipv4IntoIpv6Packed(addr: IPv4Address) -> bytes:
	return ipv4Prefix + addr.packed


def ipv4IntoIpv6(addr: IPv4Address) -> IPv6Address:
	return IPv6Address(ipv4IntoIpv6Packed(addr))


class IPv4InterfaceFromAddrAndPrefix(IPv4Interface):
	__slots__ = ()

	def __init__(self, addr: _BaseAddress, prefix: int) -> None:
		IPv4Address.__init__(self, addr)
		self.network = IPv4Network((addr, prefix), strict=False)
		self.netmask = self.network.netmask
		self._prefixlen = self.network._prefixlen


class IPv6InterfaceFromAddrAndPrefix(IPv6Interface):
	__slots__ = ()

	def __init__(self, addr: _BaseAddress, prefix: int) -> None:
		IPv6Address.__init__(self, addr)
		self.network = IPv6Network((addr, prefix), strict=False)
		self.netmask = self.network.netmask
		self._prefixlen = self.network._prefixlen


def interfaceFromAddress(addr: _BaseAddress, prefix: int) -> InterfaceT:
	if isinstance(addr, IPv4Address):
		return IPv4InterfaceFromAddrAndPrefix(addr, prefix)

	return IPv6InterfaceFromAddrAndPrefix(addr, prefix)


def packAddressToBytes(addr: _BaseAddress) -> bytes:
	if isinstance(addr, IPv4Address):
		return ipv4IntoIpv6Packed(addr)

	return addr.packed


def interfaceFromRawBytes(addrB: bytes, prefix: int) -> InterfaceT:
	addr = addressFromRawBytes(addrB)

	# We transform IPv4 addr into IPv6, so transform the prefix too into IpV4 one, if the addr will be converted into IPv4
	if isinstance(addr, IPv4Address):
		prefix -= len(ipv4Prefix) * 8

	return interfaceFromAddress(addr, prefix)


def networkFromRawBytes(addrB: bytes, prefix: int) -> _BaseNetwork:
	return interfaceFromRawBytes(addrB, prefix).network


def addressFromRawBytes(addrB: bytes) -> _BaseAddress:
	if len(addrB) == 16:
		addr = IPv6Address(addrB)
		ipv4 = addr.ipv4_mapped
		if ipv4:
			return ipv4
		return addr
	if len(addrB) == 4:
		return IPv4Address(addrB)
	raise ValueError("Invalid packed IP address", addrB)


def macFromRawBytes(macB: bytes) -> netaddr.EUI:
	return netaddr.EUI(netaddr.eui._eui48.packed_to_int(macB), version=48)
