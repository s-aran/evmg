import os
import json

from tests.it.lib.utils import Utils
from tests.it.lib.base_test import BaseTest


class TestExport(BaseTest):
    def test_normal(self):
        OUTPUT_FILEPATH = os.path.join("/tmp", "test.json")
        COMMAND_LINE = [Utils.MainProgram, "--export={}".format(OUTPUT_FILEPATH)]
        ENV = {
            "hogehoge": "piyopiyo",
            "PATH": "/foo/bar/baz:qux:/hogehoge/piyopiyo:/fugafuga/mogemoge/hogerahogera",
            "foo": "bar",
        }

        expected_return = 0
        expected_stdout = ""
        expected_stderr = ""
        Utils.assert_exec_res(
            COMMAND_LINE,
            expected_return,
            expected_stdout,
            expected_stderr,
            True,
            True,
            True,
            env=ENV,
        )

        with open(OUTPUT_FILEPATH) as f:
            json_data: dict[
                str, str | int | list[dict[str, str | int | bool]]
            ] = json.load(f)

            print(json_data)
            assert "version" in json_data.keys()
            assert "data" in json_data.keys()

            assert json_data["version"] == 1
            assert len(json_data["data"]) == 3  # type: ignore

            assert isinstance(json_data["data"], list)
            data_list = json_data["data"]
            for data in data_list:
                assert len(data.keys()) == 5
                assert "key" in data.keys()
                assert "value" in data.keys()
                assert "overwrite" in data.keys()
                assert "delimiter" in data.keys()
                assert "insert" in data.keys()

            data = data_list[0]
            assert data["key"] == "foo"
            assert data["value"] == "bar"
            assert not data["overwrite"]
            assert data["delimiter"] == ""
            assert data["insert"] == -1

            data = data_list[1]
            assert data["key"] == "PATH"
            assert (
                data["value"]
                == "/foo/bar/baz:qux:/hogehoge/piyopiyo:/fugafuga/mogemoge/hogerahogera"
            )
            assert not data["overwrite"]
            assert data["delimiter"] == ";" if Utils.is_windows() else ":"
            assert data["insert"] == -1

            data = data_list[2]
            assert data["key"] == "hogehoge"
            assert data["value"] == "piyopiyo"
            assert not data["overwrite"]
            assert data["delimiter"] == ""
            assert data["insert"] == -1

    def test_empty(self):
        OUTPUT_FILEPATH = os.path.join("/tmp", "test.json")
        COMMAND_LINE = [Utils.MainProgram, "--export={}".format(OUTPUT_FILEPATH)]

        expected_return = 0
        expected_stdout = ""
        expected_stderr = ""
        Utils.assert_exec_res(
            COMMAND_LINE,
            expected_return,
            expected_stdout,
            expected_stderr,
            True,
            True,
            True,
            env={},
        )

        with open(OUTPUT_FILEPATH) as f:
            json_data: dict[str, str | int | list[object]] = json.load(f)

            print(json_data)
            assert "version" in json_data.keys()
            assert "data" in json_data.keys()

            assert json_data["version"] == 1
            assert len(json_data["data"]) == 0  # type: ignore
