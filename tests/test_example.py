"""A simple dummy test."""

# Third-Party Imports
import pytest
from _pytest.config import Config as PyTestConfig


def test_example(pytestconfig: PyTestConfig,) -> None:
    """A dummy test."""

    if pytestconfig.getoption("test_data_path", default=None):
        pytest.fail("Oh no! Our table! It's broken!")
