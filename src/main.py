import time

from dynv6.dynv6 import Dynv6
from dynv6.config import settings
from dynv6.logger import logger


def main():
    dynv6 = Dynv6()
    params = {"hostname": settings.hostname, "token": settings.token}
    while True:
        logger.debug("checking")
        diff = False
        if not settings.no_ipv4:
            ipv4_addr = dynv6.get_ipv4()
            if ipv4_addr != dynv6.ipv4_addr:
                logger.info(
                    "old ipv4: {}, current ipv4: {}", dynv6.ipv4_addr, ipv4_addr
                )
                params["ipv4"] = ipv4_addr
                dynv6.ipv4_addr = ipv4_addr
                diff = True
        if not settings.no_ipv6:
            ipv6_addr = dynv6.get_ipv6()
            if ipv6_addr != dynv6.ipv6_addr:
                logger.info(
                    "old ipv6: {}, current ipv6: {}", dynv6.ipv6_addr, ipv6_addr
                )
                params["ipv6"] = ipv6_addr
                dynv6.ipv6_addr = ipv6_addr
                diff = True
        if diff:
            dynv6.update(params)
        time.sleep(settings.interval)


if __name__ == "__main__":
    main()
