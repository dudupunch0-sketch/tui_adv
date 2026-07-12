from pathlib import Path

def combined_development_plan() -> str:
    return (
        Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
        + chr(10)
        + Path("docs/dev/Development_Plan_Archive.md").read_text(encoding="utf-8")
    )
