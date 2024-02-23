import click
import sys
from pathlib import Path
from urllib.parse import urlparse

@click.command()
@click.argument("url", type=str)
def clone_problem(url: str):
    script_dir = Path(sys.argv[0]).parent
    root_dir = script_dir / "src"

    problem_name = urlparse(url).path.strip("/").split("/")[-1].replace("-", "_")
    problem_dir = root_dir / problem_name

    if problem_dir.exists():
        print("problem is already cloned")
        return
    else:
        problem_dir.mkdir()

    file_path = problem_dir / "mod.rs"
    link_path = problem_dir / "link.txt"
    lib_path = root_dir / "lib.rs"

    with open(link_path, "w") as f:
        f.write(url)

    with open(file_path, "w") as f:
        f.write("struct Solution;\n\nimpl Solution {\n}\n")

    with open(lib_path, "a") as f:
        f.write(f"mod {problem_name};\n")

if __name__ == "__main__":
    clone_problem()
