from ctypes import POINTER, c_int, c_ubyte, c_uint8, c_uint16, c_uint32, c_ulong

from ._funcToCtypesSignatureConvertor import assignTypesFromFunctionSignature as atffs
from .enums import ErrorCodeIntT, MediumIntT
from .library import lib
from .opaque import CBuilderPtr, CDevicePtr, DNSQueryHandlePtr, DNSSocketPtr, ICMPSocketPtr, TCPSocketPtr, UDPSocketPtr
from .structs import CAddress, CInterface, CMacAddress, IPEndpoint, IPEndpointPtr

# pylint:disable=too-many-arguments

uintptr_t = c_ulong
c_ubyte_p = POINTER(c_ubyte)
c_uint8_p = POINTER(c_uint8)
size_t = c_ulong


def newBuilder() -> CBuilderPtr:
	return _newBuilder()


_newBuilder = atffs(newBuilder, lib)


def deleteBuilder(c_builder: CBuilderPtr) -> None:
	return _deleteBuilder(c_builder)


_deleteBuilder = atffs(deleteBuilder, lib)


def builderSetHardwareAddr(c_builder: CBuilderPtr, mac: CMacAddress) -> CBuilderPtr:
	return _builderSetHardwareAddr(c_builder, mac)


_builderSetHardwareAddr = atffs(builderSetHardwareAddr, lib)


def builderInitNeighbourCache(c_builder: CBuilderPtr) -> CBuilderPtr:
	return _builderInitNeighbourCache(c_builder)


_builderInitNeighbourCache = atffs(builderInitNeighbourCache, lib)


def builderInitSixlowpan(c_builder: CBuilderPtr) -> CBuilderPtr:
	return _builderInitSixlowpan(c_builder)


try:
	_builderInitSixlowpan = atffs(builderInitSixlowpan, lib)
except AttributeError:
	builderInitSixlowpan = None


def builderInitIPv4ReassemblyBuffer(c_builder: CBuilderPtr, size: c_uint32) -> CBuilderPtr:
	return _builderInitIPv4ReassemblyBuffer(c_builder, size)

_builderInitIPv4ReassemblyBuffer = atffs(builderInitIPv4ReassemblyBuffer, lib)


def builderSetIPAddr(c_builder: CBuilderPtr, my_ip: CInterface) -> CBuilderPtr:
	return _builderSetIPAddr(c_builder, my_ip)


_builderSetIPAddr = atffs(builderSetIPAddr, lib)


def builderSetRoutes(c_builder: CBuilderPtr, gateway_ip: CAddress) -> CBuilderPtr:
	return _builderSetRoutes(c_builder, gateway_ip)


_builderSetRoutes = atffs(builderSetRoutes, lib)


def builderFinalize(c_builder: CBuilderPtr, medium: MediumIntT, mtu: uintptr_t) -> CDevicePtr:
	return _builderFinalize(c_builder, medium, mtu)


_builderFinalize = atffs(builderFinalize, lib)


def freeDevice(c_device: CDevicePtr) -> None:
	return _freeDevice(c_device)


_freeDevice = atffs(freeDevice, lib)


def getCountOfPacketsInTxQueue(c_device: CDevicePtr) -> size_t:
	return _getCountOfPacketsInTxQueue(c_device)


_getCountOfPacketsInTxQueue = atffs(getCountOfPacketsInTxQueue, lib)


def getLastTxPacketSize(c_device: CDevicePtr) -> size_t:
	return _getLastTxPacketSize(c_device)


_getLastTxPacketSize = atffs(getLastTxPacketSize, lib)


def getLastTxPacket(c_device: CDevicePtr, dst: c_uint8_p, size: c_uint32) -> size_t:
	return _getLastTxPacket(c_device, dst, size)


_getLastTxPacket = atffs(getLastTxPacket, lib)


def putRxPacket(c_device: CDevicePtr, src: c_uint8_p, size: c_uint32) -> None:
	return _putRxPacket(c_device, src, size)


_putRxPacket = atffs(putRxPacket, lib)


def newTcpSocket(c_device: CDevicePtr) -> TCPSocketPtr:
	return _newTcpSocket(c_device)


_newTcpSocket = atffs(newTcpSocket, lib)


def deleteTcpSocket(sock: TCPSocketPtr) -> None:
	return _deleteTcpSocket(sock)


_deleteTcpSocket = atffs(deleteTcpSocket, lib)


def tcpConnect(c_device: CDevicePtr, sock: TCPSocketPtr, endpoint: IPEndpoint, local_port: c_uint16) -> ErrorCodeIntT:
	return _tcpConnect(c_device, sock, endpoint, local_port)


_tcpConnect = atffs(tcpConnect, lib)


def tcpSend(c_device: CDevicePtr, sock: TCPSocketPtr, data: c_uint8_p, size: c_uint32) -> None:
	return _tcpSend(c_device, sock, data, size)


_tcpSend = atffs(tcpSend, lib)


def tcpReceive(c_device: CDevicePtr, sock: TCPSocketPtr, data: c_uint8_p, size: c_uint32) -> None:
	return _tcpReceive(c_device, sock, data, size)


_tcpReceive = atffs(tcpReceive, lib)


def tcpListen(c_device: CDevicePtr, sock: TCPSocketPtr, port: c_uint16) -> None:
	return _tcpListen(c_device, sock, port)


_tcpListen = atffs(tcpListen, lib)


def tcpIsActive(c_device: CDevicePtr, sock: TCPSocketPtr) -> bool:
	return _isTcpActive(c_device, sock)


_isTcpActive = atffs(tcpIsActive, lib)


def ifacePoll(c_device: CDevicePtr) -> None:
	return _ifacePoll(c_device)


_ifacePoll = atffs(ifacePoll, lib)


def newDnsSocket(c_device: CDevicePtr, address: CAddress) -> DNSSocketPtr:
	return _newDnsSocket(c_device, address)


_newDnsSocket = atffs(newDnsSocket, lib)


def deleteDnsSocket(sock: ICMPSocketPtr) -> None:
	return _deleteDnsSocket(sock)


_deleteDnsSocket = atffs(deleteDnsSocket, lib)


def newDnsQuery(c_device: CDevicePtr, sock: DNSSocketPtr, name: c_uint8_p, name_size: c_uint32) -> DNSQueryHandlePtr:
	return _newDnsQuery(c_device, sock, name, name_size)


_newDnsQuery = atffs(newDnsSocket, lib)


def newIcmpSocket(c_device: CDevicePtr) -> ICMPSocketPtr:
	return _newIcmpSocket(c_device)


_newIcmpSocket = atffs(newIcmpSocket, lib)


def deleteIcmpSocket(sock: ICMPSocketPtr) -> None:
	return _deleteIcmpSocket(sock)


_deleteIcmpSocket = atffs(deleteIcmpSocket, lib)

# ICMPEchoPacketType
def buildIcmpV4EchoPacket(tp: c_int, ident: c_uint32, seq_no: c_uint32, payload: c_uint8_p, payload_size: c_uint32, dst: c_uint8_p, dst_size: c_uint32) -> c_uint32:
	return _buildIcmpV4EchoPacket(tp, ident, seq_no, payload, payload_size, dst, dst_size)


_buildIcmpV4EchoPacket = atffs(buildIcmpV4EchoPacket, lib)


def newUdpSocket(c_device: CDevicePtr) -> UDPSocketPtr:
	return _newUdpSocket(c_device)


_newUdpSocket = atffs(newUdpSocket, lib)


def deleteUdpSocket(sock: UDPSocketPtr) -> None:
	return _deleteUdpSocket(sock)


_deleteUdpSocket = atffs(deleteUdpSocket, lib)


def udpBind(c_device: CDevicePtr, sock: UDPSocketPtr, port: c_uint16) -> ErrorCodeIntT:
	return _udpBind(c_device, sock, port)


_udpBind = atffs(udpBind, lib)


def udpReceive(c_device: CDevicePtr, sock: UDPSocketPtr, endpoint: IPEndpointPtr, dst: c_uint8_p, size: c_uint32) -> ErrorCodeIntT:
	return _udpReceive(c_device, sock, endpoint, dst, size)


_udpReceive = atffs(udpReceive, lib)


def udpGetLastReceivedPacketSize(c_device: CDevicePtr, sock: UDPSocketPtr) -> c_uint32:
	return _udpGetLastReceivedPacketSize(c_device, sock)


_udpGetLastReceivedPacketSize = atffs(udpGetLastReceivedPacketSize, lib)


def udpSend(c_device: CDevicePtr, sock: UDPSocketPtr, endpoint: IPEndpoint, data: c_uint8_p, size: c_uint32) -> ErrorCodeIntT:
	return _udpSend(c_device, sock, endpoint, data, size)


_udpSend = atffs(udpSend, lib)


def initLogging() -> None:
	return _initLogging()


_initLogging = atffs(initLogging, lib)

initLogging()
