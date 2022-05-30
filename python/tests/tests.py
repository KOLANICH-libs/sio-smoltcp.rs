#!/usr/bin/env python3
import itertools
import secrets
import sys
import typing
import unittest
from functools import partial
from ipaddress import IPv4Address, IPv4Interface, IPv4Network, IPv6Address, IPv6Interface, IPv6Network, _BaseAddress, _BaseNetwork
from pathlib import Path
from unittest.mock import Mock, patch

from netaddr import EUI

from os import environ

thisDir = Path(__file__).parent

sys.path.insert(0, str(thisDir.parent))

from sio_smoltcp.address import CAddress, CInterface, CMacAddress, IPEndpoint
from sio_smoltcp.utils.address import addressFromRawBytes, interfaceFromAddress, interfaceFromRawBytes, ipv4IntoIpv6, ipv4IntoIpv6Packed, ipv4Prefix, macFromRawBytes, netMaskV4IntoPrefixLengthFromBytes, networkFromRawBytes, packAddressToBytes
from sio_smoltcp.Device import Device, Medium
from sio_smoltcp.tcp import TCPSocket

from sio_smoltcp.utils.address import ipv4IntoIpv6, addressFromRawBytes
from sio_smoltcp.builder import makeDevice


environ["RUST_BACKTRACE"] = "1"


class UtilsTests(unittest.TestCase):
	def testNetMaskV4IntoPrefixLengthFromBytes(self):
		m = EUI("12:34:56:78:90:ab")
		self.assertEqual(macFromRawBytes(m.packed), m)

	def testNetMaskV4IntoPrefixLengthFromBytes(self):
		self.assertEqual(netMaskV4IntoPrefixLengthFromBytes(IPv4Interface("192.168.1.1/24").netmask.packed), 24)

	def testIpv4IntoIpv6Packed(self):
		self.assertEqual(ipv4IntoIpv6Packed(IPv4Address("192.168.1.1")), IPv6Address("::ffff:c0a8:101").packed)

	def testIpv4IntoIpv6(self):
		self.assertEqual(ipv4IntoIpv6(IPv4Address("192.168.1.1")), IPv6Address("::ffff:c0a8:101"))

	packTestMatrix = (
		(IPv4Address("192.168.1.1"), 24, 24 + len(ipv4Prefix) * 8, IPv4Network("192.168.1.1/24", strict=False), IPv4Interface("192.168.1.1/24"), ipv4Prefix + b"\xc0\xa8\x01\x01"),
		(IPv6Address("2001:0db8:11a3::765d"), 120, 120, IPv6Network("2001:0db8:11a3::765d/120", strict=False), IPv6Interface("2001:0db8:11a3::765d/120"), b" \x01\r\xb8\x11\xa3" + b"\x00" * 8 + b"v]"),
	)

	def testPackAddressToBytes(self):
		for a, p, rp, n, i, b in self.__class__.packTestMatrix:
			with self.subTest(a=a, b=b):
				self.assertEqual(packAddressToBytes(a), b)

	def testUnpackAddressFromBytes(self):
		for a, p, rp, n, i, b in self.__class__.packTestMatrix:
			with self.subTest(a=a, b=b):
				self.assertEqual(addressFromRawBytes(b), a)

	def testInterfaceFromRawBytes(self):
		for a, p, rp, n, i, b in self.__class__.packTestMatrix:
			with self.subTest(b=b, rp=rp, i=i):
				self.assertEqual(interfaceFromRawBytes(b, rp), i)

	def testNetworkFromRawBytes(self):
		for a, p, rp, n, i, b in self.__class__.packTestMatrix:
			with self.subTest(b=b, rp=rp, n=n):
				self.assertEqual(networkFromRawBytes(b, rp), n)

	def testInterfaceFromAddress(self):
		for a, p, rp, n, i, b in self.__class__.packTestMatrix:
			with self.subTest(a=a, p=p):
				i_n = interfaceFromAddress(a, p)
				self.assertEqual(i_n, i)
				self.assertEqual(i_n.network, n)


class SerializationRoundTripTests(unittest.TestCase):
	def testCMacAddress(self):
		m = EUI("12:34:56:78:90:ab")
		self.assertEqual(CMacAddress.fromPythonic(m).toPythonic(), m)

	def testCInterface(self):
		i = IPv4Interface("192.168.1.1/24")
		self.assertEqual(CInterface.fromPythonic(i).toPythonic(), i)

	def testCAddress(self):
		a = IPv4Address("192.168.1.1")
		self.assertEqual(CAddress.fromPythonic(a).toPythonic(), a)

	def testIPEndpoint(self):
		e = (IPv4Address("192.168.1.1"), 1234)
		self.assertEqual(IPEndpoint.fromPythonic(e).toPythonic(), e)


class TestConnections(unittest.TestCase):
	def testUDP(self):
		mtu = 42

		ifc1 = IPv4Interface("192.168.1.10/24")
		prt1 = 5678
		d1 = makeDevice(
			mtu=mtu,
			my_ip=ifc1,
			gateway=IPv4Address("192.168.1.1"),
			#mac=EUI("12:34:56:78:90:ab"),
		)

		ifc2 = IPv4Interface("192.168.2.11/24")
		d2 = makeDevice(
			mtu=mtu,
			my_ip=ifc2,
			gateway=IPv4Address("192.168.2.1"),
			#mac=EUI("12:34:56:78:90:ab"),
		)

		tgtAddr = IPv4Address("192.168.1.1")
		d1.poll()
		#print(d.getCountOfPacketsInTxQueue())
		#s = d1.tcpSocket()
		d1.poll()
		#print(d.getCountOfPacketsInTxQueue())
		#s.connect((tgtAddr, 80), 1234)
		d1.poll()
		#print(d.getCountOfPacketsInTxQueue())

		s1 = d1.udpSocket()
		s2 = d2.udpSocket()
		d1.poll()
		d2.poll()

		s1.bind(prt1)
		s2.bind(1234)

		sent = b"hello" * 20

		s1.send((ifc2, 1234), sent)
		d1.poll()

		packets = []

		while True:
			p = d1.pop()
			if not p:
				break
			print("raw packet", p)
			packets.append(p)
			d2.put(p)

		d2.poll()
		received, senderEndpoint = s2.receive()
		received = bytes(received)
		senderAddr, senderPort = senderEndpoint
		self.assertEqual(senderAddr, ifc1.ip)
		self.assertEqual(senderPort, prt1)
		self.assertEqual(received, sent)



if __name__ == "__main__":
	unittest.main()
