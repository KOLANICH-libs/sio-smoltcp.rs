#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef void SocketHandle;


/**
 * Type of medium of a device.
 */
enum CMedium
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
	Invalid = 0,
	Ethernet = 2,
	Ip = 3,
	Ieee802154 = 4,
};
#ifndef __cplusplus
typedef uint8_t CMedium;
#endif // __cplusplus

enum ICMPEchoPacketType
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
	EchoRequest = 1,
	EchoReply = 2,
};
#ifndef __cplusplus
typedef uint8_t ICMPEchoPacketType;
#endif // __cplusplus

typedef void *CBuilderPtr;

typedef struct CMacAddress {
	unsigned char mac[6];
} CMacAddress;

typedef struct CAddress {
	unsigned char ip[16];
} CAddress;

typedef struct CInterface {
	uint8_t prefix;
	struct CAddress addr;
} CInterface;

typedef void *CDevicePtr;

typedef SocketHandle DNSSocketHandle;

typedef SocketHandle ICMPSocketHandle;

typedef struct CIPEndpoint {
	uint16_t port;
	struct CAddress addr;
} CIPEndpoint;

typedef SocketHandle TCPSocketHandle;

typedef SocketHandle UDPSocketHandle;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void initLogging(void);

CBuilderPtr newBuilder(void);

void deleteBuilder(CBuilderPtr c_builder);

CBuilderPtr builderSetHardwareAddr(CBuilderPtr c_builder, struct CMacAddress mac);

CBuilderPtr builderInitNeighbourCache(CBuilderPtr c_builder);

CBuilderPtr builderInitIPv4ReassemblyBuffer(CBuilderPtr c_builder, uint32_t size);

CBuilderPtr builderSetIPAddr(CBuilderPtr c_builder, struct CInterface my_ip);

CBuilderPtr builderSetRoutes(CBuilderPtr c_builder, struct CAddress gateway_ip);

CDevicePtr builderFinalize(CBuilderPtr c_builder, CMedium medium, uintptr_t mtu);

void freeDevice(CDevicePtr c_device);

uintptr_t getCountOfPacketsInTxQueue(CDevicePtr c_device);

uintptr_t getLastTxPacketSize(CDevicePtr c_device);

uintptr_t getLastTxPacket(CDevicePtr c_device, uint8_t *dst, uint32_t size);

void putRxPacket(CDevicePtr c_device, const uint8_t *src, uint32_t size);

void ifacePoll(CDevicePtr c_device);

DNSSocketHandle *newDnsSocket(CDevicePtr c_device, struct CAddress address);

void deleteDnsSocket(DNSSocketHandle *c_handle);

const QueryHandle *newDnsQuery(CDevicePtr c_device, DNSSocketHandle *c_handle, const uint8_t *name, uint32_t name_size);

ICMPSocketHandle *newIcmpSocket(CDevicePtr c_device);

void deleteIcmpSocket(ICMPSocketHandle *c_handle);

uint32_t buildIcmpV4EchoPacket(ICMPEchoPacketType tp, uint16_t ident, uint16_t seq_no, const uint8_t *payload, uint32_t payload_size, uint8_t *dst, uint32_t dst_size);

ICMPBindError icmpBindAny(CDevicePtr c_device, ICMPSocketHandle *c_handle);

ICMPBindError icmpBindIdent(CDevicePtr c_device, ICMPSocketHandle *c_handle, uint16_t ident);

ICMPBindError icmpBindUDP(CDevicePtr c_device, ICMPSocketHandle *c_handle, struct CIPEndpoint endpoint);

ICMPSendError icmpSend(CDevicePtr c_device, ICMPSocketHandle *c_handle, struct CAddress address, const uint8_t *data, uint32_t size);

ICMPRecvError icmpReceive(CDevicePtr c_device, ICMPSocketHandle *c_handle, struct CAddress *address, uint8_t *dst, uint32_t size);

/**
 * Deletes a socket using its handle. While you currently can use ths function to delete sockets, using specialized functions is more future-proof since I can imagine that we can have to store some auxillary info besides the handle.
 */
void delete_socket(SocketHandle *c_handle);

TCPSocketHandle *newTcpSocket(CDevicePtr c_device);

void deleteTcpSocket(TCPSocketHandle *c_handle);

TCPConnectError tcpConnect(CDevicePtr c_device, TCPSocketHandle *c_handle, struct CIPEndpoint c_endpoint, uint16_t local_port);

void tcpSend(CDevicePtr c_device, TCPSocketHandle *c_handle, const uint8_t *data, uint32_t size);

void tcpReceive(CDevicePtr c_device, TCPSocketHandle *c_handle, uint8_t *data, uint32_t size);

void tcpListen(CDevicePtr c_device, TCPSocketHandle *c_handle, uint16_t port);

bool tcpIsActive(CDevicePtr c_device, TCPSocketHandle *c_handle);

UDPSocketHandle *newUdpSocket(CDevicePtr c_device);

void deleteUdpSocket(UDPSocketHandle *c_handle);

UDPBindError udpBind(CDevicePtr c_device, UDPSocketHandle *c_handle, uint16_t port);

uint32_t udpGetLastReceivedPacketSize(CDevicePtr c_device, UDPSocketHandle *c_handle);

UDPRecvError udpReceive(CDevicePtr c_device, UDPSocketHandle *c_handle, struct CIPEndpoint *endpoint, uint8_t *dst, uint32_t size);

UDPSendError udpSend(CDevicePtr c_device, UDPSocketHandle *c_handle, struct CIPEndpoint endpoint, const uint8_t *data, uint32_t size);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
