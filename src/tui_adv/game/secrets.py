from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

import yaml

from tui_adv.game.content import DATA_DIR


@dataclass(frozen=True, slots=True)
class PublicSecret:
    id: str
    title: str
    unlock_flags: tuple[str, ...]
    public_hint_steps: tuple[str, ...]
    final_hint_policy: str
    reward_text: str
    puzzle_prompt: str = ""
    placeholder_ip_address: str | None = None


@dataclass(frozen=True, slots=True)
class LocalSecret:
    id: str
    final_hint: str
    safety_checked: bool
    notes: str = ""
    actual_ip_address: str | None = None


@dataclass(frozen=True, slots=True)
class PhysicalHintReveal:
    id: str
    title: str
    public_hint_steps: tuple[str, ...]
    reward_text: str
    final_hint: str | None = None
    puzzle_prompt: str = ""
    puzzle_ip_address: str | None = None
    puzzle_answer: int | None = None

    @property
    def final_hint_available(self) -> bool:
        return self.final_hint is not None


def reveal_physical_hint(
    secret_id: str,
    *,
    public_path: Path | Any | None = None,
    local_path: Path | Any | None = None,
) -> PhysicalHintReveal:
    """Merge safe public hint steps with an optional local-only final hint."""

    public_secrets = load_public_secrets(public_path)
    if secret_id not in public_secrets:
        raise ValueError(f"unknown public secret: {secret_id}")
    public_secret = public_secrets[secret_id]
    local_secret = load_local_secrets(local_path).get(secret_id)
    final_hint = None
    ip_address = public_secret.placeholder_ip_address
    if local_secret is not None and local_secret.safety_checked:
        final_hint = local_secret.final_hint
        if local_secret.actual_ip_address:
            ip_address = local_secret.actual_ip_address
    return PhysicalHintReveal(
        id=public_secret.id,
        title=public_secret.title,
        public_hint_steps=public_secret.public_hint_steps,
        reward_text=public_secret.reward_text,
        final_hint=final_hint,
        puzzle_prompt=public_secret.puzzle_prompt,
        puzzle_ip_address=ip_address,
        puzzle_answer=sum_ip_address_digits(ip_address) if ip_address else None,
    )


def load_public_secrets(path: Path | Any | None = None) -> dict[str, PublicSecret]:
    secret_path = DATA_DIR.joinpath("secrets.example.yaml") if path is None else path
    data = _read_yaml(secret_path)
    secrets: dict[str, PublicSecret] = {}
    for entry in data.get("secrets", []):
        if "final_hint" in entry:
            raise ValueError(f"public secret has final_hint: {entry.get('id')}")
        secret = PublicSecret(
            id=entry["id"],
            title=entry["title"],
            unlock_flags=tuple(entry.get("unlock_flags", ())),
            public_hint_steps=tuple(entry.get("public_hint_steps", ())),
            final_hint_policy=entry.get("final_hint_policy", "private_only"),
            reward_text=entry.get("reward_text", ""),
            puzzle_prompt=entry.get("puzzle_prompt", ""),
            placeholder_ip_address=entry.get("placeholder_ip_address"),
        )
        if secret.id in secrets:
            raise ValueError(f"duplicate public secret id: {secret.id}")
        secrets[secret.id] = secret
    return secrets


def load_local_secrets(path: Path | Any | None = None) -> dict[str, LocalSecret]:
    secret_path = Path("private/secrets.local.yaml") if path is None else path
    if not secret_path.exists():
        return {}
    data = _read_yaml(secret_path)
    secrets: dict[str, LocalSecret] = {}
    for entry in data.get("secrets", []):
        secret = LocalSecret(
            id=entry["id"],
            final_hint=entry.get("final_hint", ""),
            safety_checked=bool(entry.get("safety_checked", False)),
            notes=entry.get("notes", ""),
            actual_ip_address=entry.get("actual_ip_address"),
        )
        if secret.id in secrets:
            raise ValueError(f"duplicate local secret id: {secret.id}")
        secrets[secret.id] = secret
    return secrets


def sum_ip_address_digits(ip_address: str) -> int:
    """Return the sum of every digit written in an IP address label."""

    return sum(int(character) for character in ip_address if character.isdigit())


def _read_yaml(path: Path | Any) -> dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    loaded = yaml.safe_load(text)
    if loaded is None:
        return {}
    if not isinstance(loaded, dict):
        raise ValueError(f"YAML root must be a mapping: {path}")
    return loaded
