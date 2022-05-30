from ctypes import c_uint32

from .ctypes.functions import c_uint8_p
from .ctypes.functions import newDnsQuery as newDnsQuery_ct
from .ctypes.functions import newDnsSocket as newDnsSocket_ct
from .ctypes.opaque import CDevicePtr, DNSQueryHandlePtr, DNSSocketPtr
from .ctypes.structs import CAddress


def newDnsSocket(c_device: CDevicePtr, address: CAddress) -> DNSSocketPtr:
	return newDnsSocket_ct(c_device, address)


def newDnsQuery(c_device: CDevicePtr, sock: DNSSocketPtr, name: c_uint8_p, name_size: c_uint32) -> DNSQueryHandlePtr:
	return newDnsQuery_ct(c_device, sock, name, name_size)


class DNSSocket:
	__slots__ = ("parent", "ptr")

	def __init__(self, parent: "Device", server):
		self.parent = parent
		self.ptr = newDnsSocket(parent.ptr, server)

	def free(self):
		if self.ptr:
			deleteDnsSocket(self.ptr)
			self.ptr = None

	def query(self, name: str) -> "DNSQuery":
		return DNSQuery(self, name)


class DNSQuery:
	__slots__ = ("parent", "ptr")

	def __init__(self, parent: DNSSocket, name: str):
		self.parent = parent
		self.ptr = newDnsQuery(parent.parent.ptr, parent.ptr, name)
