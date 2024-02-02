import structlog
import logging

get_logger = logging.getLogger


def setup_root_handler(level="info"):
    """
    Setup the root logger with a human-readable output.
    """
    root = logging.getLogger()
    root.setLevel(level.upper())
    handler = logging.StreamHandler()
    root.addHandler(handler)

    _setup_human_output(handler)

    log = get_logger("root")
    log.debug(f"Setting up root logger with level '{level}'")


def _setup_human_output(handler):
    shared_processors = [
        structlog.stdlib.add_log_level,
        structlog.stdlib.add_logger_name,
        structlog.processors.TimeStamper(fmt="%Y-%m-%d %H:%M:%S"),
    ]

    formatter = structlog.stdlib.ProcessorFormatter(
        foreign_pre_chain=shared_processors,
        processors=[structlog.dev.ConsoleRenderer()],
    )

    handler.setFormatter(formatter)
