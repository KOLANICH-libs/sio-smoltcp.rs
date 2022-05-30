from ctypes import c_int, c_uint32

from .ctypes.functions import buildIcmpV4EchoPacket as ct_buildIcmpV4EchoPacket
from .ctypes.functions import c_uint8_p, deleteIcmpSocket, newIcmpSocket
from .ctypes.opaque import CDevicePtr, ICMPSocketPtr
from .utils.resource import ResourceWithParent


# ICMPEchoPacketType
def buildIcmpV4EchoPacket(tp: c_int, ident: c_uint32, seq_no: c_uint32, payload: bytes) -> bytearray:
	pBuf, pSize = byteStringToPointer(payload)
	resSize = ct_buildIcmpV4EchoPacket(tp, ident, seq_no, pBuf, pSize, None, 0)
	res = bytearray(resSize)
	rBuf, rSize = byteStringToPointer(res)
	if ct_buildIcmpV4EchoPacket(tp, ident, seq_no, pBuf, pSize, rBuf, rSize):
		raise RuntimeError("Error building ICMPv4 packet")
	return res


class ICMPSocket(ResourceWithParent):
	__slots__ = ("parent", "ptr")

	DTOR = deleteIcmpSocket

	def __init__(self, parent: "Device"):
		self.parent = parent
		self.ptr = newIcmpSocket(parent.ptr)
