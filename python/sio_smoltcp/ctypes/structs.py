from ctypes import POINTER, Structure, c_ubyte, c_uint8, c_uint16, c_uint64

# pylint:disable=too-few-public-methods


class CAddress(Structure):
	__slots__ = ("ip",)
	_fields_ = (("ip", c_ubyte * 16),)


class IPEndpoint(Structure):
	__slots__ = ("port", "addr")
	_fields_ = (
		("port", c_uint16),
		("addr", CAddress),
	)


IPEndpointPtr = POINTER(IPEndpoint)


class CInterface(Structure):
	__slots__ = ("prefix", "addr")
	_fields_ = [
		("prefix", c_uint8),
		("addr", CAddress),
	]


class CMacAddress(Structure):
	__slots__ = ("mac",)
	_fields_ = (("mac", c_ubyte * 6),)
