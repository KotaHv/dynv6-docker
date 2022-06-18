from enum import Enum

from pydantic import BaseSettings


class LogLevel(Enum):
    trace = "TRACE"
    debug = "DEBUG"
    info = "INFO"
    success = "SUCCESS"
    warning = "WARNING"
    error = "ERROR"
    critical = "CRITICAL"


class Settings(BaseSettings):
    hostname: str
    token: str
    interval: int | float = 600
    no_ipv4: bool = False
    no_ipv6: bool = False
    interface: str = "eth0"
    log_level: LogLevel = LogLevel.info

    class Config:
        env_prefix = "dynv6_"  # defaults to no prefix


settings = Settings()
