# Reality Secret Safety Checklist

Use this before copying `docs/templates/local-secrets.template.yaml` to `private/secrets.local.yaml` and replacing placeholders with real-world clue text.

Rules:

1. Keep `safety_checked: false` until every item below is true.
2. Store real final clue text only in `private/secrets.local.yaml` or another ignored `*.local.*` file.
3. Do not put employee names, real room numbers, internal addresses, customer data, access instructions, or locked-area details in committed files.
4. Prefer public/shared, low-risk office objects that the developer deliberately placed.
5. If a clue uses a workplace label such as an IP address, keep the real value only in local data and leave committed examples on documentation/test placeholder ranges such as `192.0.2.10`.
6. Run the secret tests after editing the template or loader:

```bash
python3 -m pytest tests/test_secrets.py -q
```

Release check:

- `git status --ignored --short private/` should show local files as ignored, not staged.
- `git check-ignore private/secrets.local.yaml` should print the path.
- Public docs may describe the safety process, but not the final physical location.
