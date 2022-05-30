from ipaddress import IPv4Interface, _BaseAddress

from netaddr import EUI

from .address import CAddress, CInterface, CMacAddress
from .ctypes.enums import Medium, MediumIntT
from .ctypes.functions import builderFinalize as builderFinalize_ct
from .ctypes.functions import builderInitIPv4FragmentsCache, builderInitNeighbourCache, builderInitSixlowpan
from .ctypes.functions import builderSetHardwareAddr as builderSetHardwareAddr_ct
from .ctypes.functions import builderSetIPAddr as builderSetIPAddr_ct
from .ctypes.functions import builderSetRoutes as builderSetRoutes_ct
from .ctypes.functions import deleteBuilder, newBuilder, uintptr_t
from .ctypes.opaque import CBuilderPtr, CDevicePtr
from .Device import Device


def builderSetHardwareAddr(c_builder: CBuilderPtr, mac: EUI) -> CBuilderPtr:
	return builderSetHardwareAddr_ct(c_builder, CMacAddress.fromPythonic(mac))


def builderSetIPAddr(c_builder: CBuilderPtr, my_ip: IPv4Interface) -> CBuilderPtr:
	return builderSetIPAddr_ct(c_builder, CInterface.fromPythonic(my_ip))


def builderSetRoutes(c_builder: CBuilderPtr, gateway_ip: _BaseAddress) -> CBuilderPtr:
	return builderSetRoutes_ct(c_builder, CAddress.fromPythonic(gateway_ip))


def builderFinalize(c_builder: CBuilderPtr, medium: Medium, mtu: int) -> CDevicePtr:
	return builderFinalize_ct(c_builder, MediumIntT(medium), uintptr_t(mtu))


class DeviceBuilder:
	__slots__ = ("ptr",)

	def __init__(self) -> None:
		self.ptr = newBuilder()

	def setHardwareAddr(self, mac: EUI):
		self.ptr = builderSetHardwareAddr(self.ptr, mac)

	def initNeighbourCache(self):
		self.ptr = builderInitNeighbourCache(self.ptr)

	def initSixlowpan(self):
		self.ptr = builderInitSixlowpan(self.ptr)

	def initIPv4FragmentsCache(self) -> None:
		self.ptr = builderInitIPv4FragmentsCache(self.ptr)

	def setIPAddr(self, my_ip: IPv4Interface) -> None:
		self.ptr = builderSetIPAddr(self.ptr, my_ip)

	def setRoutes(self, gateway: _BaseAddress) -> None:
		self.ptr = builderSetRoutes(self.ptr, gateway)

	def finalize(self, medium: Medium, mtu: int) -> Device:
		devicePtr = builderFinalize(self.ptr, medium, mtu)
		self.ptr = None
		return Device(devicePtr)

	def __del__(self) -> None:
		if self.ptr:
			deleteBuilder(self.ptr)


def initL3Device(builder: DeviceBuilder, my_ip: IPv4Interface, gateway: _BaseAddress) -> None:
	builder.initIPv4FragmentsCache()
	# builder.initSixlowpan()

	builder.setIPAddr(my_ip)
	builder.setRoutes(gateway)


def initL2Device(builder: DeviceBuilder, mac: EUI, my_ip: IPv4Interface = None, gateway: _BaseAddress = None):
	builder.setHardwareAddr(mac)
	builder.initNeighbourCache()

	initL3Device(builder, my_ip, gateway)


def makeL2Device(mtu: int, mac: EUI, my_ip: IPv4Interface = None, gateway: _BaseAddress = None) -> Device:
	b = DeviceBuilder()
	initL2Device(b, mac, my_ip, gateway)
	return b.finalize(medium=Medium.Ethernet, mtu=mtu)


def makeL3Device(mtu: int, my_ip: IPv4Interface, gateway: _BaseAddress = None) -> Device:
	b = DeviceBuilder()
	initL3Device(b, my_ip, gateway)
	return b.finalize(medium=Medium.Ip, mtu=mtu)


def makeDevice(mtu: int, my_ip: IPv4Interface = None, gateway: _BaseAddress = None, mac: EUI = None) -> Device:
	if mac:
		return makeL2Device(mtu=mtu, mac=mac, my_ip=my_ip, gateway=gateway)
	else:
		return makeL3Device(mtu=mtu, my_ip=my_ip, gateway=gateway)
