from netaddr import EUI

from .address import CAddress, CInterface, CMacAddress
from .ctypes.enums import Medium, MediumIntT
from .ctypes.functions import c_uint32, freeDevice
from .ctypes.functions import getCountOfPacketsInTxQueue as getCountOfPacketsInTxQueue_ct
from .ctypes.functions import getLastTxPacket as getLastTxPacket_ct
from .ctypes.functions import getLastTxPacketSize as getLastTxPacketSize_ct
from .ctypes.functions import ifacePoll
from .ctypes.functions import putRxPacket as putRxPacket_ct
from .ctypes.functions import size_t, uintptr_t
from .ctypes.opaque import CDevicePtr
from .ctypes.utils import byteStringToPointer
from .dns import DNSSocket
from .icmp import ICMPSocket
from .tcp import TCPSocket
from .udp import UDPSocket
from .utils.address import InterfaceT
from .utils.resource import Resource


def getCountOfPacketsInTxQueue(c_device: CDevicePtr) -> int:
	return int(getCountOfPacketsInTxQueue_ct(c_device))


def getLastTxPacketSize(c_device: CDevicePtr) -> size_t:
	return int(getLastTxPacketSize_ct(c_device))


def getLastTxPacket(c_device: CDevicePtr, dst: bytearray) -> int:
	buf, size = byteStringToPointer(dst)
	return int(getLastTxPacket_ct(c_device, buf, c_uint32(size)))


def putRxPacket(c_device: CDevicePtr, data: bytes) -> None:
	buf, size = byteStringToPointer(data)
	return putRxPacket_ct(c_device, buf, c_uint32(size))


class Device(Resource):
	__slots__ = ()
	DTOR = freeDevice

	def getCountOfPacketsInTxQueue(self):
		if self._ptr:
			return getCountOfPacketsInTxQueue(self.ptr)
		return 0

	def poll(self) -> None:
		ifacePoll(self.ptr)

	def pop(self) -> bytearray:
		sz = getLastTxPacketSize(self.ptr)
		res = bytearray(sz)
		getLastTxPacket(self.ptr, res)
		return res

	def put(self, data: bytearray) -> None:
		putRxPacket(self.ptr, data)

	def tcpSocket(self) -> TCPSocket:
		return TCPSocket(self)

	def udpSocket(self) -> UDPSocket:
		return UDPSocket(self)

	def icmpSocket(self) -> ICMPSocket:
		return ICMPSocket(self)

	def dnsSocket(self, server) -> DNSSocket:
		return DNSSocket(self, server)
