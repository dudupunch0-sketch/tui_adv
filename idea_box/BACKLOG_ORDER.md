# idea_box backlog order

이 문서는 `idea_box`의 미반영 아이디어를 어떤 순서로 처리해야 하는지 표시한다.

핵심 규칙:

1. `status: done`이 아닌 idea entry는 아직 반영되지 않은 backlog다.
2. backlog 처리 순서는 파일이 Git에 처음 추가된 commit 순서가 기본이다. 원격 push 시각 자체는 일반 Git history만으로 안정적으로 복원할 수 없으므로, 이 repo에서는 “Git에 올라간 순서”를 “파일이 처음 추가된 commit의 시간/순서”로 정의한다.
3. 현재 구조화된 처리 대상은 `idea_box/inbox/*.md`의 status-bearing idea entry다.
4. Notion-origin entry의 원본 reference는 Notion이다. inbox entry는 Notion page id/title/url, 요약, 처리 순서, 관련 설계 문서 포인터를 담는 추적 문서다.
5. `idea_box` 루트의 원문/가이드 문서는 source 또는 운영 문서일 수 있다. `source_ref`가 있는 inbox entry가 있으면 그 inbox entry의 status와 order를 따른다.
6. 읽기만 했다고 `done`으로 바꾸지 않는다. 실제 설계/문서/구현에 반영하고 Notion reference 대조까지 마쳤거나, 명시적으로 폐기/병합 처리하고 이유를 적었을 때만 `done`이다.
7. 설계 흐름은 Notion 정리 → repo 설계 아이디어 문서 변환 → `docs/dev/Development_Plan.md` main plan 격상 → 설계 진행 → Notion reference 대조 → done 처리 순서다.
8. 새 idea entry를 만들 때 같은 날짜 파일명이 여러 개라 순서가 애매하면 `backlog_order`, `git_added_at`, `git_added_commit`을 frontmatter에 추가한다.
9. Git 최초 추가 시점을 알 수 없는 파일은 처리 전에 순서를 명시한다. 필요하면 파일명 prefix나 `backlog_order`를 추가한다.

## 현재 open backlog

Git 최초 추가 순서 기준이다. 아래 순서대로 처리한다.

현재 구조화된 open `idea_box/inbox/*.md` entry는 없다.

| order | idea entry | git first added | commit | 상태 | 다음 처리 |
|---|---|---|---|---|---|

## 이미 done인 구조화 entry

아래 entry들은 현재 backlog 처리 대상에서 제외한다.

| idea entry | git first added | commit | status |
|---|---|---|---|
| `idea_box/done/2026-05-21-tui-storybook-glyphfx-concept-v2.md` | 2026-05-21T14:50:28+00:00 | `57a381bd` | done |
| `idea_box/done/2026-05-21-character-stats-and-generator.md` | 2026-05-22T01:48:39+00:00 | `d32fc359` | done |
| `idea_box/done/2026-05-22-dream-ending-branching.md` | 2026-05-22T03:01:30+00:00 | `f6fc8173` | done |
| `idea_box/done/2026-05-22-semiconductor-sw-storypacks.md` | 2026-05-22T03:25:19+00:00 | `2eb5fb91` | done |
| `idea_box/done/2026-05-22-general-corporate-storypacks.md` | 2026-05-22T04:21:39+00:00 | `2de58ef6` | done |
| `idea_box/done/2026-05-22-real-escape-ending-branching.md` | 2026-05-22T04:55:10+00:00 | `488110eb` | done |
| `idea_box/done/2026-05-29-notion-storypack-system.md` | UNTRACKED | - | done |
| `idea_box/done/2026-05-29-notion-wuxia-igu-hakji-cheonggi-record.md` | UNTRACKED | - | done |
| `idea_box/done/2026-05-29-notion-office-yageunmong.md` | UNTRACKED | - | done |

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
2. 해당 idea entry의 Notion page id/title/url, `related_docs`, `source_ref`를 확인한다.
3. Notion-origin entry라면 Notion 원본 reference를 다시 읽는다.
4. Notion 내용을 repo 설계 아이디어 문서로 변환하거나, 이미 변환된 `related_docs`를 검토한다.
5. 다음에 실제 설계할 항목이면 `docs/dev/Development_Plan.md`의 active main plan / “현재 최우선 남은 작업”으로 격상시킨다.
6. 설계 후 Notion reference와 결과 설계가 같은 방향인지 대조하고 처리 기록에 적는다.
7. `status: done`, `reference_check`, `used_by`, `done_at`은 Notion 대조 또는 명시적 폐기/병합까지 끝난 뒤에만 갱신한다.
8. 이 문서의 현재 open backlog 표를 갱신한다.
