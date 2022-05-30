import typing
from collections.abc import ByteString
from ctypes import POINTER, c_ubyte, cast

c_ubyte_p = POINTER(c_ubyte)


def byteStringToPointer(data: ByteString) -> typing.Tuple[c_ubyte_p, int]:
	size = len(data)
	if isinstance(data, bytes):
		buf = cast(data, c_ubyte_p)
	else:
		bufT = c_ubyte * size
		buf = bufT.from_buffer(data)
		buf = cast(buf, c_ubyte_p)

	return buf, size
