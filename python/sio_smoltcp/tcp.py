import typing
from ctypes import c_uint16, c_uint32

from .address import IPEndpoint, PythonicEndpointT
from .ctypes.enums import TCPConnectError
from .ctypes.functions import newTcpSocket
from .ctypes.functions import tcpConnect as ct_tcpConnect
from .ctypes.functions import tcpIsActive as ct_tcpIsActive
from .ctypes.functions import tcpListen as ct_tcpListen
from .ctypes.functions import tcpReceive as ct_tcpReceive
from .ctypes.functions import tcpSend as ct_tcpSend
from .ctypes.opaque import CDevicePtr, TCPSocketPtr
from .ctypes.utils import byteStringToPointer

# from .Device import Device


def tcpConnect(c_device: CDevicePtr, sock: TCPSocketPtr, endpoint: PythonicEndpointT, local_port: int) -> None:
	res = TCPConnectError(int(ct_tcpConnect(c_device, sock, IPEndpoint.fromPythonic(endpoint), c_uint16(local_port))))
	if res != TCPConnectError.OK:
		raise RuntimeError("TCP connection failed", res)


def tcpSend(c_device: CDevicePtr, sock: TCPSocketPtr, data: bytes) -> None:
	buf, size = byteStringToPointer(data)
	return ct_tcpSend(c_device, sock, buf, c_uint32(size))


def tcpReceive(c_device: CDevicePtr, sock: TCPSocketPtr, data: bytes) -> None:
	buf, size = byteStringToPointer(data)
	return ct_tcpReceive(c_device, sock, buf, c_uint32(size))


def tcpListen(c_device: CDevicePtr, sock: TCPSocketPtr, port: int) -> None:
	return ct_tcpListen(c_device, sock, c_uint16(port))


class TCPSocket:
	__slots__ = ("parent", "ptr")

	def __init__(self, parent: "Device"):
		self.parent = parent
		self.ptr = newTcpSocket(parent.ptr)

	def free(self):
		if self.ptr:
			deleteTcpSocket(self.ptr)
			self.ptr = None

	def isActive(self) -> bool:
		return ct_tcpIsActive(self.parent.ptr, self.ptr)

	def connect(self, endpoint: PythonicEndpointT, local_port: int):
		tcpConnect(self.parent.ptr, self.ptr, endpoint, local_port)

	def listen(self, port: int):
		tcpConnect(self.parent.ptr, self.ptr, port)

	def send(self, data: bytes):
		tcpSend(self.parent.ptr, self.ptr, data)

	def receive(self, data: bytes):
		tcpReceive(self.parent.ptr, self.ptr, data)
