# -*- coding: utf-8 -*-

import sys
import pytest
import subprocess
import os
import os.path
import shutil
import shlex
import platform
import re
import datetime
import stat


class Utils:
    @staticmethod
    def is_windows():
        return platform.system() == "Windows"

    MainProgramName = "envvar{}".format(".exe" if is_windows() else "")
    MainProgram = os.path.join("..", "..", "target", "release", MainProgramName)

    @staticmethod
    def execute(command_line: str) -> tuple[int, str, str]:
        enc = "shiftjis" if Utils.is_windows() else "utf-8"
        sl = shlex.split(
            command_line if Utils.is_windows() else shlex.quote(command_line),
            posix=not Utils.is_windows(),
        )
        p = subprocess.Popen(
            sl, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, encoding=enc
        )

        p.wait()

        try:
            out, err = p.communicate()
        except IndexError:
            print("Index error in p.communicate()")
            out = ""
            err = ""

        ret = int(p.returncode)

        return ret, out.strip(), err.strip()

    @staticmethod
    def assert_exec_res(
        command_lines: str | list[str],
        expected_r: int,
        expected_o: str,
        expected_e: str,
        show_r: bool = False,
        show_o: bool = False,
        show_e: bool = False,
    ):
        command_line = (
            " ".join(command_lines)
            if type(command_lines) == list
            else "{}".format(command_lines)
        )
        r, o, e = Utils.execute(command_line)

        if show_r:
            print("======== return code ========")
            print(r)
        if show_o:
            print("======== stdout ========")
            print(o)
        if show_e:
            print("======== stderr ========")
            print(e)

        assert r == expected_r
        assert o.rstrip() == expected_o
        assert e.rstrip() == expected_e

    @staticmethod
    def list2string(something_list: list[str]):
        return "\n".join(something_list)

    @staticmethod
    def string2list(something_string: str):
        return something_string.split(os.linesep)
