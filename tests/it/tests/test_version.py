import os
import toml

from tests.it.lib.utils import Utils
from tests.it.lib.base_test import BaseTest


class TestVersion(BaseTest):
    def test_normal(self):
        CARGO_TOML_PATH = os.path.join("Cargo.toml")
        COMMAND_LINE = [Utils.MainProgram, "--version"]

        cargo_toml = toml.load(CARGO_TOML_PATH)

        expected_return = 0
        expected_stdout = "envvar {}".format(cargo_toml["package"]["version"])
        expected_stderr = ""
        Utils.assert_exec_res(
            COMMAND_LINE,
            expected_return,
            expected_stdout,
            expected_stderr,
            True,
            True,
            True,
        )
