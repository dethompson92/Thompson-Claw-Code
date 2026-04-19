#!/usr/bin/env python3
"""Focused tests for gh_fetch.py without requiring CLI dependencies."""

from __future__ import annotations

import asyncio
import importlib.util
import sys
import types
import unittest
from pathlib import Path


class FakeTyperApp:
    def command(self, *args, **kwargs):
        def decorator(func):
            return func

        return decorator


class FakeConsole:
    def __init__(self):
        self.messages: list[str] = []

    def print(self, message="", *args, **kwargs):
        self.messages.append(str(message))


class FakeProgress:
    pass


class FakeTable:
    def __init__(self, *args, **kwargs):
        pass

    def add_column(self, *args, **kwargs):
        pass

    def add_row(self, *args, **kwargs):
        pass


class FakePanel:
    def __init__(self, *args, **kwargs):
        pass


def load_gh_fetch_module():
    typer_module = types.ModuleType("typer")
    typer_module.Typer = lambda *args, **kwargs: FakeTyperApp()
    typer_module.Option = lambda *args, **kwargs: None
    typer_module.Exit = SystemExit

    rich_module = types.ModuleType("rich")
    rich_console_module = types.ModuleType("rich.console")
    rich_console_module.Console = FakeConsole
    rich_panel_module = types.ModuleType("rich.panel")
    rich_panel_module.Panel = FakePanel
    rich_progress_module = types.ModuleType("rich.progress")
    rich_progress_module.Progress = FakeProgress
    rich_progress_module.TaskID = int
    rich_table_module = types.ModuleType("rich.table")
    rich_table_module.Table = FakeTable

    original_modules = {
        name: sys.modules.get(name)
        for name in [
            "typer",
            "rich",
            "rich.console",
            "rich.panel",
            "rich.progress",
            "rich.table",
        ]
    }
    sys.modules.update(
        {
            "typer": typer_module,
            "rich": rich_module,
            "rich.console": rich_console_module,
            "rich.panel": rich_panel_module,
            "rich.progress": rich_progress_module,
            "rich.table": rich_table_module,
        }
    )

    try:
        module_path = Path(__file__).with_name("gh_fetch.py")
        spec = importlib.util.spec_from_file_location("gh_fetch_under_test", module_path)
        module = importlib.util.module_from_spec(spec)
        assert spec is not None and spec.loader is not None
        spec.loader.exec_module(module)
        return module
    finally:
        for name, original in original_modules.items():
            if original is None:
                sys.modules.pop(name, None)
            else:
                sys.modules[name] = original


class GhFetchTests(unittest.TestCase):
    def test_fetch_items_page_returns_empty_list_for_invalid_json(self):
        module = load_gh_fetch_module()

        async def fake_run_gh_command(_args):
            return ("not valid json", "", 0)

        module.run_gh_command = fake_run_gh_command

        result = asyncio.run(
            module.fetch_items_page("owner/repo", "issue", "open", 10)
        )

        self.assertEqual(result, [])
        self.assertTrue(
            any("Error parsing issue response" in message for message in module.console.messages)
        )


if __name__ == "__main__":
    unittest.main()
