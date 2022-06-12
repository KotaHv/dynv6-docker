import sys
from pathlib import Path

import httpx
import netifaces

from .config import settings
from .logger import logger

base_dir = Path("data")
base_dir.mkdir(parents=True, exist_ok=True)
ipv4_file = base_dir / ".dynv6.addr4"
ipv6_file = base_dir / ".dynv6.addr6"

TIMEOUT = 5


class Dynv6:
    def __init__(self) -> None:
        self.dynv6_url = "https://dynv6.com/api/update"
        self.ipv4_url = "https://api4.my-ip.io/ip"
        self.client = httpx.Client(timeout=TIMEOUT)
        self._ipv4_addr = ""
        self._ipv6_addr = ""
        self.load_ip()

    @property
    def ipv4_addr(self):
        return self._ipv4_addr

    @ipv4_addr.setter
    def ipv4_addr(self, ipv4_addr: str):
        self._ipv4_addr = ipv4_addr
        with open(ipv4_file, "w") as f:
            f.write(ipv4_addr)

    @property
    def ipv6_addr(self):
        return self._ipv6_addr

    @ipv6_addr.setter
    def ipv6_addr(self, ipv6_addr: str):
        self._ipv6_addr = ipv6_addr
        with open(ipv6_file, "w") as f:
            f.write(ipv6_addr)

    def get_ipv4(self) -> str:
        response = self.client.get(self.ipv4_url)
        return response.text

    def get_ipv6(self) -> str:
        return netifaces.ifaddresses(settings.interface)[netifaces.AF_INET6][0]["addr"]

    def load_ip(self):
        try:
            with open(ipv4_file, "r") as f:
                self._ipv4_addr = f.read()
        except FileNotFoundError:
            pass
        try:
            with open(ipv6_file, "r") as f:
                self._ipv6_addr = f.read()
        except FileNotFoundError:
            pass

    def update(self, params):
        logger.info("ipv4/ipv6 address changed, start update")
        for _ in range(3):
            try:
                respone = self.client.get(self.dynv6_url, params=params)
            except (
                httpx.ConnectTimeout,
                httpx.ReadTimeout,
            ):
                continue
            if not respone.is_success:
                self.ipv4_addr = ""
                self.ipv6_addr = ""
                logger.error("code: {}, msg: {}", respone.status_code, respone.text)
                sys.exit(0)
            logger.info(respone.text)
            break
        else:
            self.ipv4_addr = ""
            self.ipv6_addr = ""
