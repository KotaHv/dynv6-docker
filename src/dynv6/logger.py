import sys
from loguru import logger
from .config import settings

logger.remove()
logger.add(sys.stderr, level=settings.log_level)
