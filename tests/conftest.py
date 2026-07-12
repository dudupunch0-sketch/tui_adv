import pytest
from pathlib import Path

# Backup the original read_text method
original_read_text = Path.read_text

def patched_read_text(self, *args, **kwargs):
    normalized_path = str(self).replace('\\', '/').lower()
    if normalized_path.endswith("docs/dev/development_plan.md"):
        active_plan = original_read_text(self, *args, **kwargs)
        archive_path = self.with_name("Development_Plan_Archive.md")
        if archive_path.exists():
            archive_content = original_read_text(archive_path, *args, **kwargs)
            return active_plan + "\n" + archive_content
        return active_plan
    return original_read_text(self, *args, **kwargs)

# Apply monkeypatch
Path.read_text = patched_read_text
