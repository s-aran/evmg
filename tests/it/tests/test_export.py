import os
import json

import pytest
import winreg

from tests.it.lib.utils import Utils
from tests.it.lib.base_test import BaseTest


class TestExport(BaseTest):
    TEST_FILENAME = "test.json"

    def teardown_method(self, method: str):
        if os.path.exists(TestExport.TEST_FILENAME):
            os.remove(TestExport.TEST_FILENAME)
        return super().teardown_method(method)

    @pytest.mark.skipif(Utils.is_windows(), reason="only for Linux")
    def test_normal_for_linux(self):
        OUTPUT_FILEPATH = os.path.join(TestExport.TEST_FILENAME)
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
            assert len(json_data["data"]) == 3

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

    @pytest.mark.skipif(Utils.is_windows(), reason="only for Linux")
    def test_empty_for_linux(self):
        OUTPUT_FILEPATH = os.path.join(TestExport.TEST_FILENAME)
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
            assert len(json_data["data"]) == 0

    @pytest.mark.skipif(not Utils.is_windows(), reason="only for Windows")
    def test_normal_for_windows(self):
        OUTPUT_FILEPATH = os.path.join(TestExport.TEST_FILENAME)
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
        )

        with open(OUTPUT_FILEPATH) as f:
            json_data: dict[
                str, str | int | list[dict[str, str | int | bool]]
            ] = json.load(f)

            print(json_data)
            print(os.environ)
            assert "version" in json_data.keys()
            assert "data" in json_data.keys()

            assert json_data["version"] == 1
            assert len(json_data["data"]) > 1

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
