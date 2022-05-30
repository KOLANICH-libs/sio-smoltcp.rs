import typing
from ctypes import c_uint16
from ipaddress import IPv4Interface, _BaseAddress

from netaddr import EUI

from .ctypes.structs import CAddress as CAddress_ct
from .ctypes.structs import CInterface as CInterface_ct
from .ctypes.structs import CMacAddress as CMacAddress_ct
from .ctypes.structs import IPEndpoint as IPEndpoint_ct
from .utils.address import InterfaceT, addressFromRawBytes, interfaceFromRawBytes, ipv4Prefix, macFromRawBytes, packAddressToBytes

# pylint:disable=too-few-public-methods


class CAddress(CAddress_ct):
	__slots__ = ()

	@classmethod
	def fromPythonic(cls, addr: _BaseAddress) -> "CAddress":
		# pylint: disable=attribute-defined-outside-init
		res = cls()
		res.ip[:] = packAddressToBytes(addr)

		return res

	def toPythonic(self) -> _BaseAddress:
		return addressFromRawBytes(bytes(self.ip))


PythonicEndpointT = typing.Tuple[_BaseAddress, int]


class IPEndpoint(IPEndpoint_ct):
	__slots__ = ()

	@classmethod
	def fromPythonic(cls, endpoint: PythonicEndpointT) -> "IPEndpoint":
		# pylint: disable=attribute-defined-outside-init
		address, port = endpoint
		res = cls()
		res.port = c_uint16(port)
		res.addr = CAddress.fromPythonic(address)
		return res

	def toPythonic(self) -> PythonicEndpointT:
		return (CAddress.toPythonic(self.addr), int(self.port))  # some shit prevents us from calling the method directly


class CInterface(CInterface_ct):
	__slots__ = ()

	@classmethod
	def fromPythonic(cls, ifc: InterfaceT) -> "CInterface":
		# pylint: disable=attribute-defined-outside-init
		res = cls()
		prefix = ifc._prefixlen  # pylint: disable=protected-access

		# We transform IPv4 addr into IPv6, so transform the prefix too
		if isinstance(ifc, IPv4Interface):
			prefix += len(ipv4Prefix) * 8

		res.prefix = prefix
		res.addr = CAddress.fromPythonic(ifc.ip)
		return res

	def toPythonic(self) -> InterfaceT:
		return interfaceFromRawBytes(bytes(self.addr.ip), self.prefix)


class CMacAddress(CMacAddress_ct):
	__slots__ = ()

	@classmethod
	def fromPythonic(cls, eui: EUI) -> "CMacAddress":
		# pylint: disable=attribute-defined-outside-init
		res = cls()
		res.mac[:] = eui.packed
		return res

	def toPythonic(self) -> EUI:
		return macFromRawBytes(bytes(self.mac))
