#!/usr/bin/env python3

from pathlib import Path
import re

bot_path = Path(__file__).parent.parent / "bot" / "src"
commands_path = bot_path / "commands"

fn_pattern = re.compile(r"pub async fn (\w+)")
cmds_pattern = re.compile(r"commands: vec!\[.*?\]")

if __name__ == "__main__":
    command_modules = []
    commands = []

    for child in commands_path.iterdir():
        if child.name == "mod.rs":
            continue

        module = child.name.split(".")[0]
        command_modules += [f"pub mod {module};"]

        with open(child, "r") as f:
            matches = fn_pattern.findall(f.read(), re.MULTILINE)
            commands += [f"{module}::{command}()" for command in matches]

    with open(commands_path / "mod.rs", "w") as f:
        f.write("\n".join(command_modules) + "\n")

    with open(bot_path / "main.rs", "r") as f:
        content = f.read()

    content = re.sub(
        cmds_pattern, f"commands: vec![{', '.join(commands)}]", content, count=1
    )

    with open(bot_path / "main.rs", "w") as f:
        f.write(content)
