import os
import ipaddress

os.environ["dynv6_hostname"] = "example.com"
os.environ["dynv6_token"] = "example"

from src.dynv6.dynv6 import Dynv6, ipv4_file, ipv6_file


def test_init():
    Dynv6()


def test_load_ip():
    ipv4_addr = "127.0.0.1"
    ipv6_addr = "::"
    with open(ipv6_file, "w") as f:
        f.write(ipv6_addr)

    with open(ipv4_file, "w") as f:
        f.write(ipv4_addr)

    dynv6 = Dynv6()
    assert dynv6.ipv4_addr == ipv4_addr
    assert dynv6.ipv6_addr == ipv6_addr


def test_change_ip():
    ipv4_addr = "127.0.1.1"
    ipv6_addr = "::1"
    dynv6 = Dynv6()
    dynv6.ipv4_addr = ipv4_addr
    dynv6.ipv6_addr = ipv6_addr

    with open(ipv6_file, "r") as f:
        assert f.read() == ipv6_addr

    with open(ipv4_file, "r") as f:
        assert f.read() == ipv4_addr


def test_get_ip():
    dynv6 = Dynv6()
    ipv4_addr = dynv6.get_ipv4()
    assert isinstance(ipaddress.ip_address(ipv4_addr), ipaddress.IPv4Address)
    ipv6_addr = dynv6.get_ipv6()
    assert isinstance(ipaddress.ip_address(ipv6_addr), ipaddress.IPv6Address)
