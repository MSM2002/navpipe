import os
import re
import subprocess
import sys
from datetime import date
from pathlib import Path

import toml


def run(cmd):
    """Helper to run shell commands and return output."""
    return subprocess.check_output(cmd, shell=True, text=True).strip()


def get_current_branch():
    return run("git rev-parse --abbrev-ref HEAD")


def update_version_files(version):
    """Updates version in pyproject.toml, Cargo.toml, and navpipe/__init__.py."""
    # 1. Update pyproject.toml
    with open("pyproject.toml", "r") as f:
        py_data = toml.load(f)
    py_data["project"]["version"] = version
    with open("pyproject.toml", "w") as f:
        toml.dump(py_data, f)

    # 2. Update Cargo.toml
    with open("Cargo.toml", "r") as f:
        cargo_data = toml.load(f)
    cargo_data["package"]["version"] = version
    with open("Cargo.toml", "w") as f:
        toml.dump(cargo_data, f)

    # 3. Update navpipe/__init__.py
    init_path = os.path.join("navpipe", "__init__.py")
    if os.path.exists(init_path):
        with open(init_path, "r") as f:
            content = f.read()
        new_content = re.sub(
            r'__version__\s*=\s*".*?"', f'__version__ = "{version}"', content
        )
        with open(init_path, "w") as f:
            f.write(new_content)
    else:
        print(f"Warning: {init_path} not found.")


def update_changelog(version):
    today = date.today().isoformat()
    with open("CHANGELOG.md", "r") as f:
        lines = f.readlines()

    new_lines = []
    release_notes = []
    capture = False

    for line in lines:
        if "## [Unreleased]" in line:
            new_lines.append(line)
            new_lines.append(
                "\n### Added\n- \n\n### Changed\n- \n\n### Fixed\n- \n\n---\n\n"
            )
            new_lines.append(f"## [{version}] - {today}\n")
            capture = True
            continue

        # Stop capturing once we hit the next divider
        if capture and line.startswith("---"):
            capture = False

        if capture:
            release_notes.append(line)

        if not capture or (capture and line.strip() != ""):
            if not line.startswith("## [Unreleased]"):
                new_lines.append(line)

    with open("CHANGELOG.md", "w") as f:
        f.writelines(new_lines)


def make_pypi_readme():
    """
    Extract all <!-- PYPI-START --> ... <!-- PYPI-END --> blocks
    from README.md and write them to docs/README_PyPI.md.
    """

    source = Path("README.md")
    target = Path("docs/README_PyPI.md")

    if not source.exists():
        raise FileNotFoundError("README.md not found")

    text = source.read_text(encoding="utf-8")

    matches = re.findall(
        r"<!--\s*PYPI-START\s*-->(.*?)<!--\s*PYPI-END\s*-->",
        text,
        flags=re.DOTALL,
    )

    if not matches:
        raise ValueError("No PYPI blocks found in README.md")

    content = "\n\n".join(section.strip() for section in matches)

    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(content + "\n", encoding="utf-8")


def main():
    # Branch Protection
    current_branch = get_current_branch()
    if current_branch != "main":
        print(
            f"Error: Release script must be run on 'main' branch. You are on '{current_branch}'."
        )
        sys.exit(1)

    if len(sys.argv) < 2:
        print("Usage: python release.py <version_number>")
        sys.exit(1)

    new_version = sys.argv[1]

    print(f"Starting release process for version {new_version}...")
    update_version_files(new_version)
    update_changelog(new_version)
    make_pypi_readme()

    try:
        run(
            "git add pyproject.toml Cargo.toml CHANGELOG.md navpipe/__init__.py docs/README_PyPI.md"
        )
        run(f'git commit -m "chore: release {new_version}"')
        run(f'git tag -a {new_version} -m "Release version {new_version}"')
        run("git push origin main")
        run(f"git push origin {new_version}")
        print(f"Successfully released {new_version}")
    except Exception as e:
        print(f"Git Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
