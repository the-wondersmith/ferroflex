"""Pytest configuration and fixtures for testing `ferroflex`."""

# Third-Party Imports
import pytest  # noqa: F401
from _pytest.config.argparsing import Parser


def pytest_addoption(parser: Parser) -> None:
    """Add some extra options for the `ferroflex` test suite."""
    parser.addoption(
        "--test-data-path",
        nargs="?",
        type=str,
        required=False,
        action="store",
        dest="test_data_path",
        default=None,
    )
