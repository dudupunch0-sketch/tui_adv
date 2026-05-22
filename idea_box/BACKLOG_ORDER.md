# idea_box backlog order

이 문서는 `idea_box`의 미반영 아이디어를 어떤 순서로 처리해야 하는지 표시한다.

핵심 규칙:

1. `status: done`이 아닌 idea entry는 아직 반영되지 않은 backlog다.
2. backlog 처리 순서는 파일이 Git에 처음 추가된 commit 순서가 기본이다. 원격 push 시각 자체는 일반 Git history만으로 안정적으로 복원할 수 없으므로, 이 repo에서는 “Git에 올라간 순서”를 “파일이 처음 추가된 commit의 시간/순서”로 정의한다.
3. 현재 구조화된 처리 대상은 `idea_box/inbox/*.md`의 status-bearing idea entry다.
4. `idea_box` 루트의 원문/가이드 문서는 source 또는 운영 문서일 수 있다. `source_ref`가 있는 inbox entry가 있으면 그 inbox entry의 status와 order를 따른다.
5. 읽기만 했다고 `done`으로 바꾸지 않는다. 실제 docs/code/data에 반영했거나, 명시적으로 폐기/병합 처리하고 이유를 적었을 때만 `done`이다.
6. 새 idea entry를 만들 때 같은 날짜 파일명이 여러 개라 순서가 애매하면 `backlog_order`, `git_added_at`, `git_added_commit`을 frontmatter에 추가한다.
7. Git 최초 추가 시점을 알 수 없는 파일은 처리 전에 순서를 명시한다. 필요하면 파일명 prefix나 `backlog_order`를 추가한다.

## 현재 open backlog

Git 최초 추가 순서 기준이다. 아래 순서대로 처리한다.

| order | idea entry | git first added | commit | 상태 | 다음 처리 |
|---|---|---|---|---|---|
| 001 | `idea_box/inbox/2026-05-21-tui-storybook-glyphfx-concept-v2.md` | 2026-05-21T14:50:28+00:00 | `57a381bd` | open | 먼저 반영 |
| 002 | `idea_box/inbox/2026-05-22-dream-ending-branching.md` | 2026-05-22T03:01:30+00:00 | `f6fc8173` | open | 001 완료 후 반영 |
| 003 | `idea_box/inbox/2026-05-22-real-escape-ending-branching.md` | 2026-05-22T04:55:10+00:00 | `488110eb` | open | 002 완료 후 반영 |

## 이미 done인 구조화 entry

아래 entry들은 현재 backlog 처리 대상에서 제외한다.

| idea entry | git first added | commit | status |
|---|---|---|---|
| `idea_box/inbox/2026-05-21-character-stats-and-generator.md` | 2026-05-22T01:48:39+00:00 | `d32fc359` | done |
| `idea_box/inbox/2026-05-22-semiconductor-sw-storypacks.md` | 2026-05-22T03:25:19+00:00 | `2eb5fb91` | done |
| `idea_box/inbox/2026-05-22-general-corporate-storypacks.md` | 2026-05-22T04:21:39+00:00 | `2de58ef6` | done |

## 순서 재계산 방법

특정 파일의 Git 최초 추가 시점은 다음으로 확인한다.

```bash
git log --follow --diff-filter=A --format='%aI %h %H' -- idea_box/inbox/<file>.md
```

모든 inbox markdown의 현재 status와 최초 추가 순서를 다시 보고 싶으면 다음 기준으로 확인한다.

```bash
python3 - <<'PY'
import pathlib, re, subprocess
root = pathlib.Path.cwd()
rows = []
for p in sorted((root / 'idea_box' / 'inbox').glob('*.md')):
    text = p.read_text(encoding='utf-8')
    m = re.match(r'^---\n(.*?)\n---\n', text, re.S)
    status = None
    order = None
    if m:
        for line in m.group(1).splitlines():
            if line.startswith('status:'):
                status = line.split(':', 1)[1].strip()
            if line.startswith('backlog_order:'):
                order = line.split(':', 1)[1].strip()
    rel = p.relative_to(root).as_posix()
    log = subprocess.check_output([
        'git', 'log', '--follow', '--diff-filter=A', '--format=%aI %h', '--', rel
    ], text=True).strip().splitlines()
    first = log[-1] if log else 'UNTRACKED'
    rows.append((order or '---', first, status or 'NO_STATUS', rel))
for row in sorted(rows):
    print(' | '.join(row))
PY
```

## 다음 LLM/agent에게 주는 해석

`idea_box`를 처리하라는 지시를 받으면 임의로 흥미로운 항목을 고르지 않는다.

1. 이 파일에서 가장 낮은 `order`의 `open` 항목을 고른다.
2. 해당 idea entry와 `related_docs` / `source_ref`를 읽는다.
3. 관련 docs/code/data에 반영한다.
4. 반영 결과와 이유를 idea entry의 처리 기록에 적는다.
5. `status: done`, `used_by`, `done_at`을 갱신한다.
6. 이 문서의 현재 open backlog 표를 갱신한다.
