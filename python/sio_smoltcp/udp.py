import typing
from ctypes import byref, c_uint16, c_uint32

from .address import IPEndpoint, PythonicEndpointT
from .ctypes.enums import ErrorCode
from .ctypes.functions import deleteUdpSocket, newUdpSocket
from .ctypes.functions import udpBind as ct_udpBind
from .ctypes.functions import udpGetLastReceivedPacketSize as ct_udpGetLastReceivedPacketSize
from .ctypes.functions import udpReceive as ct_udpReceive
from .ctypes.functions import udpSend as ct_udpSend
from .ctypes.opaque import CDevicePtr, UDPSocketPtr
from .ctypes.utils import byteStringToPointer
from .utils.resource import ResourceWithParent

# from .Device import Device


def udpSend(c_device: CDevicePtr, sock: UDPSocketPtr, endpoint: PythonicEndpointT, data: bytes) -> None:
	buf, size = byteStringToPointer(data)
	res = ErrorCode(ct_udpSend(c_device, sock, IPEndpoint.fromPythonic(endpoint), buf, c_uint32(size)))
	if res != ErrorCode.OK:
		raise RuntimeError(res)


def udpGetLastReceivedPacketSize(c_device: CDevicePtr, sock: UDPSocketPtr) -> int:
	return int(ct_udpGetLastReceivedPacketSize(c_device, sock))


def udpReceive(c_device: CDevicePtr, sock: UDPSocketPtr, dst: bytearray) -> PythonicEndpointT:
	buf, size = byteStringToPointer(dst)
	ep = IPEndpoint()
	res = ErrorCode(ct_udpReceive(c_device, sock, byref(ep), buf, c_uint32(size)))
	if res != ErrorCode.OK:
		raise RuntimeError(res)

	return ep.toPythonic()


def udpBind(c_device: CDevicePtr, sock: UDPSocketPtr, port: int) -> None:
	res = ErrorCode(int(ct_udpBind(c_device, sock, c_uint16(port))))
	if res != ErrorCode.OK:
		raise RuntimeError(res)


class UDPSocket(ResourceWithParent):
	__slots__ = ("parent", "ptr")

	CTOR = newUdpSocket
	DTOR = deleteUdpSocket

	def _ensure(self):
		self.parent._ensureDevice()  # pylint:disable=protected-access
		if self.ptr is None:
			raise RuntimeError("smoltcp UDP socket was freed")

	def isActive(self) -> bool:
		return ct_isActive(self.parent.ptr, self.ptr)

	def bind(self, port: int) -> None:
		udpBind(self.parent.ptr, self.ptr, port)

	def send(self, endpoint: PythonicEndpointT, data: bytes) -> None:
		udpSend(self.parent.ptr, self.ptr, endpoint, data)

	def receive(self) -> typing.Tuple[PythonicEndpointT, bytes]:
		sz = udpGetLastReceivedPacketSize(self.parent.ptr, self.ptr)
		res = bytearray(sz)
		endpoint = udpReceive(self.parent.ptr, self.ptr, res)
		return res, endpoint
