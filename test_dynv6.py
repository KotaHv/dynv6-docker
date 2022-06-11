import ipaddress

import pytest

from main import Dynv6, ipv4_file, ipv6_file


def set_env(monkeypatch):
    monkeypatch.setenv("dynv6_hostname", "example.com")
    monkeypatch.setenv("dynv6_token", "example")


def test_init(monkeypatch):
    with pytest.raises(KeyError, match=r".*dynv6_hostname.*"):
        Dynv6()
    monkeypatch.setenv("dynv6_hostname", "example.com")
    with pytest.raises(KeyError, match=r".*dynv6_token.*"):
        Dynv6()
    monkeypatch.delenv("dynv6_hostname")
    monkeypatch.setenv("dynv6_token", "example")
    with pytest.raises(KeyError, match=r".*dynv6_hostname.*"):
        Dynv6()
    set_env(monkeypatch)
    Dynv6()


def test_load_ip(monkeypatch):
    set_env(monkeypatch)
    ipv4_addr = "127.0.0.1"
    ipv6_addr = "::"
    with open(ipv6_file, "w") as f:
        f.write(ipv6_addr)

    with open(ipv4_file, "w") as f:
        f.write(ipv4_addr)

    dynv6 = Dynv6()
    assert dynv6.ipv4_addr == ipv4_addr
    assert dynv6.ipv6_addr == ipv6_addr


def test_change_ip(monkeypatch):
    set_env(monkeypatch)
    ipv4_addr = "127.0.1.1"
    ipv6_addr = "::1"
    dynv6 = Dynv6()
    dynv6.ipv4_addr = ipv4_addr
    dynv6.ipv6_addr = ipv6_addr

    with open(ipv6_file, "r") as f:
        assert f.read() == ipv6_addr

    with open(ipv4_file, "r") as f:
        assert f.read() == ipv4_addr


def test_get_ip(monkeypatch):
    set_env(monkeypatch)
    dynv6 = Dynv6()
    ipv4_addr = dynv6.get_ipv4()
    assert isinstance(ipaddress.ip_address(ipv4_addr), ipaddress.IPv4Address)
    ipv6_addr = dynv6.get_ipv6()
    assert isinstance(ipaddress.ip_address(ipv6_addr), ipaddress.IPv6Address)
