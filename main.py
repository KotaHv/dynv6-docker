import os
import sys
import time
import logging

import netifaces
import requests

__version__ = "0.1.0"

TIMEOUT = 5

logger = logging.getLogger("dynv6")
logger.setLevel(os.environ.get("dynv6_logger_level", "INFO").upper())
formatter = logging.Formatter(
    fmt=r"%(asctime)s [%(name)s] [%(levelname)s]: %(message)s",
    datefmt=r"%Y/%m/%d %H:%M:%S",
)

stream_handler = logging.StreamHandler(sys.stdout)
stream_handler.setFormatter(formatter)
logger.addHandler(stream_handler)


class Dynv6:
    def __init__(self) -> None:
        self.dynv6_url = "https://dynv6.com/api/update"
        self.ipv4_url = "https://api4.my-ip.io/ip"
        self.session = requests.Session()
        self._ipv4_addr = ""
        self._ipv6_addr = ""
        self.load_ip()
        self.load_env()

    @property
    def ipv4_addr(self):
        return self._ipv4_addr

    @ipv4_addr.setter
    def ipv4_addr(self, ipv4_addr: str):
        self._ipv4_addr = ipv4_addr
        with open(".dynv6.addr4", "w") as f:
            f.write(ipv4_addr)

    @property
    def ipv6_addr(self):
        return self._ipv6_addr

    @ipv6_addr.setter
    def ipv6_addr(self, ipv6_addr: str):
        self._ipv6_addr = ipv6_addr
        with open(".dynv6.addr6", "w") as f:
            f.write(ipv6_addr)

    def get_ipv4(self) -> str:
        response = self.session.get(self.ipv4_url, timeout=TIMEOUT)
        return response.text

    def get_ipv6(self) -> str:
        return netifaces.ifaddresses(self.interface)[netifaces.AF_INET6][0]["addr"]

    def load_env(self):
        self.hostname = os.environ["dynv6_hostname"]
        self.token = os.environ["dynv6_token"]
        self.interval = float(os.environ.get("dynv6_interval", 600))
        self.no_ipv6 = (
            True
            if os.environ.get("dynv6_no_ipv6", "false").lower() == "true"
            else False
        )
        self.no_ipv4 = (
            True
            if os.environ.get("dynv6_no_ipv4", "false").lower() == "true"
            else False
        )
        self.interface = os.environ.get("dynv6_interface", "eth0")

    def load_ip(self):
        try:
            with open(".dynv6.addr6", "r") as f:
                self._ipv6_addr = f.read()
        except FileNotFoundError:
            pass
        try:
            with open(".dynv6.addr4", "r") as f:
                self._ipv4_addr = f.read()
        except FileNotFoundError:
            pass


def main():
    dynv6 = Dynv6()
    params = {"hostname": dynv6.hostname, "token": dynv6.token}
    while True:
        logger.info("checking")
        diff = False
        if not dynv6.no_ipv4:
            ipv4_addr = dynv6.get_ipv4()
            logger.info(ipv4_addr)
            if ipv4_addr != dynv6.ipv4_addr:
                params["ipv4"] = ipv4_addr
                dynv6.ipv4_addr = ipv4_addr
                diff = True
        if not dynv6.no_ipv6:
            ipv6_addr = dynv6.get_ipv6()
            logger.info(ipv6_addr)
            if ipv6_addr != dynv6.ipv6_addr:
                params["ipv6"] = ipv6_addr
                dynv6.ipv6_addr = ipv6_addr
                diff = True
        if diff:
            logger.info("ipv4/ipv6 address changed, start update")
            for _ in range(3):
                try:
                    respone = dynv6.session.get(
                        dynv6.dynv6_url, params=params, timeout=TIMEOUT
                    )
                except (
                    requests.exceptions.ConnectTimeout,
                    requests.exceptions.ReadTimeout,
                ):
                    continue
                if respone.status_code != 200:
                    dynv6.ipv4_addr = ""
                    dynv6.ipv6_addr = ""
                    logger.error(
                        "code: %s,\tmsg: %s", respone.status_code, respone.text
                    )
                    sys.exit(0)
                logger.info(respone.text)
                break
            else:
                dynv6.ipv4_addr = ""
                dynv6.ipv6_addr = ""
        time.sleep(dynv6.interval)


if __name__ == "__main__":
    main()
