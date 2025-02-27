import click
import sys
from pathlib import Path
import re

@click.group()
def cli():
    pass

@cli.command()
@click.argument("problem_name", type=str)
def new(problem_name: str):
    create_problem(problem_name)

@cli.command()
@click.argument("url", type=str)
def leetcode(url: str):
    m = re.match("https://leetcode.com/problems/([^/]+)", url)

    if m is None:
        print("invalid url")
        return

    problem_name = m.group(1)
    create_problem(problem_name, url)


def create_problem(problem_name: str, url: str | None = None):
    problem_name = problem_name.replace("-", "_")

    script_dir = Path(sys.argv[0]).parent
    root_dir = script_dir / "src"

    problem_dir = root_dir / problem_name

    if problem_dir.exists():
        print(f"problem with name {problem_name} already exists")
        return
    else:
        problem_dir.mkdir()

    file_path = problem_dir / "mod.rs"
    lib_path = root_dir / "lib.rs"

    with open(script_dir / "template.rs.tmpl", "r") as f:
        template = f.read()
        template = template.replace("%%PROBLEM_NAME%%", problem_name)

    with open(file_path, "w") as f:
        f.write(template)

    with open(lib_path, "a") as f:
        f.write(f"mod {problem_name};\n")

    if url is not None:
        link_path = problem_dir / "link.txt"
        with open(link_path, "w") as f:
            f.write(url)

if __name__ == "__main__":
    cli()
