import platform
from ctypes import CDLL

if platform.system() == "Windows":
	lib = CDLL("./libsio_smoltcp.dll")
else:
	lib = CDLL("./libsio_smoltcp.so")
