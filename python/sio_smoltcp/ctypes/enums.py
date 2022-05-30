from ctypes import c_uint8
from enum import IntEnum


class Medium(IntEnum):
	Invalid = 0
	Ethernet = 2
	Ip = 3
	Ieee802154 = 4


class ErrorCode(IntEnum):
	OK = 0
	Exhausted = 1
	Illegal = 2
	Unaddressable = 3
	Finished = 4
	Truncated = 5
	Checksum = 6
	Unrecognized = 7
	Fragmented = 8
	Malformed = 9
	Dropped = 10
	ReassemblyTimeout = 11
	PacketAssemblerNotInit = 12
	PacketAssemblerBufferTooSmall = 13
	PacketAssemblerIncomplete = 14
	PacketAssemblerTooManyHoles = 15
	PacketAssemblerOverlap = 16
	PacketAssemblerSetFull = 17
	PacketAssemblerSetKeyNotFound = 18
	NotSupported = 19

	InvalidState = 20
	BufferFull = 21
	NoFreeSlot = 22
	InvalidName = 23
	NameTooLong = 24
	Pending = 25
	Failed = 26

	BufferInsufficient = 0xFF


UDPBindError = TCPConnectError = UDPSendError = ErrorCode


MediumIntT = c_uint8
IPVersionIntT = c_uint8
ErrorCodeIntT = c_uint8
