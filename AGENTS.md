# AGENTS.md

이 파일은 이 저장소에서 작업하는 AI agent를 위한 프로젝트 운영 지침이다.

## 프로젝트 성격

- 한국어 storypack/world 기반 선택지 생존 게임 프로젝트다. 현재 메인/default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다. 전제는 현대 회사원이 본인 몸과 출근복장 그대로 무협 세계에 전이되는 것이다.
- 회사-아포칼립스(`escape from the office`)는 기존 기준팩/legacy content로 남아 있지만, 새 UI/UX와 Web player 기본 경로는 이구학지를 우선한다. 엔진/renderer/문서 설계는 특정 세계관에 고정하지 않고, office surface와 무협 surface 양쪽에서 설명 가능한 형태를 유지한다.
- 시각 정체성은 TUI/fake-terminal 분위기를 유지하되, Web Storybook은 shell/HUD, GlyphFX는 story/UI 효과를 담당한다. 전투 보드의 primary renderer는 Three.js 고정 카메라 3D hex auto-battler다.
- 스토리/UI 특수 효과는 Web Storybook의 Canvas/GlyphFX가 소유하고, 전투 보드 VFX는 Three.js가 소유할 수 있다. 기존 browser fake-TUI는 legacy/parity fallback으로만 취급한다.
- 실제 사용자의 메모/사적 노트는 공개 산출물로 옮기지 않는다.

## 로컬 작업 위치와 실행 환경

- 이 프로젝트의 장기 작업 기준은 **WSL 내부 ext4 경로의 clone**이다. 권장 기본 경로는 `~/work/tui-adv`다.
- Windows 경로(`C:\Users\82105\Documents\tui-adv`)에 있는 repo는 전환 전 백업/참조용으로만 취급하고, 새 작업의 주 repo로 쓰지 않는다.
- Claude와 Codex는 같은 WSL 내부 clone을 기준으로 각자 worktree/branch를 만들어 작업한다. Claude는 주로 가이드/계획을 작성하고, Codex는 그 가이드를 읽고 구현한다.
- `git`, `cargo`, `wasm-pack`, `npm`, `pytest`, `gh` 등 개발 명령은 WSL에서 실행한다.
- Windows는 Codex Desktop, 브라우저 preview, localhost 확인, 파일 확인 같은 UI/검토 용도로만 사용한다.
- 기존 Windows repo에 남은 변경사항이 있으면 WSL 내부 clone으로 전환하기 전에 `git status`로 확인하고, 필요한 변경만 명시적으로 옮긴다.
- `/mnt/c/...` 아래 Windows 파일시스템 repo에서 장기 빌드/테스트를 반복하지 않는다. CRLF/LF, 권한, 파일 감시, `node_modules`, Rust `target` 같은 차이 때문에 성능 저하와 불필요한 diff가 생기기 쉽다.

## Subagent 모델 tier (작업 위임 규칙)

메인 agent는 전체 오케스트레이션, 설계 판단, 범위·우선순위·충돌 해결과 최종 직접 검증을 맡는다.
subagent 생성과 model/effort 선택에는 매번 사용자 승인을 다시 받지 않아도 되며, 비용과 실패 비용을
함께 고려해 다음 tier를 기본값으로 쓴다. (2026-08-12 사용자 지시)

- **GPT-5.6 Luna Low/Medium = 가벼운 작업과 범위가 명확한 실행.** 분류, 추출, 빠른 조회,
  테스트벤치 실행, 단순 문서 동기화, 이미 결정된 작은 구현.
- **GPT-5.6 Terra Medium = 일반 구현.** 여러 파일에 걸치지만 계약과 acceptance가 명확한 코드 작업.
- **GPT-5.6 Sol Medium = 맥락·설계 판단.** 큰 문서 정독, 아키텍처, 정합성 감사, 충돌 해결,
  복잡한 리뷰. High 이상은 실패 비용이 큰 핵심 결정에만 제한한다.

적용:

- subagent를 띄울 때 작업 성격에 맞는 model/effort를 명시한다.
- read-only 조사/탐색은 Luna 또는 Explorer로 돌리되, 넓은 맥락 판단이 필요하면 Sol Medium을 쓴다.
- 한 슬라이스 완료마다 아주 짧게 보고한다. plan/orchestration ↔ 구현(subagent) 단계를 구분한다.

### 검증 신뢰 원칙 (필수)

- **subagent의 PASS/FAIL 보고를 그대로 믿지 않는다.** 영향이 큰 검증(테스트/빌드/산출물 반영)은 main에서 직접 재실행한다. (사례: subagent가 cargo 없는 셸에서 "link.exe 실패"를 환각 보고함.)
- "실패가 환경 탓"이라는 보고는 도구 존재 여부부터 확인(`which`/`Get-Command`). 이 머신에서 `cargo`/`wasm-pack`/`gh`는 **WSL에만** 있고, pytest는 repo `.venv`를 쓴다.
- 콘텐츠 변경의 도달 경로를 먼저 파악한다. 런타임 Rust 생성 텍스트(epilogue body 등)는 **wasm 바이너리**에 들어가므로 `export_web_data.py`가 아니라 wasm 재빌드가 필요하다.
- 상세 절차는 `docs/dev/Development_Methodology.md`, 환경/알려진 실패는 `docs/dev/Troubleshooting.md`를 따른다.
- 한 슬라이스를 실제로 굴릴 때의 규율(플랜에 적을 정지 조건, 기대값 수정 규칙, 직접 재검증 항목, 위임 지시서 체크리스트)과 이 저장소에서 반복해서 밟은 함정은 `docs/dev/Implementation_Slice_Discipline.md`에 정리돼 있다. 구현을 위임하기 전에 읽는다.

## 협업 루프: 플랜/리뷰 ↔ 구현 (역할 이관 가능해야 함)

기본 분업은 **Fable(Claude) = 플랜 작성·구현 리뷰·시각 폴리시 직접 작업, codex/gemini = 플랜 구현**이다. 단, Fable의 토큰이 부족하면 codex가 이 섹션을 따라 플랜/리뷰 역할까지 이어받을 수 있어야 한다. 아래 컨벤션은 역할과 무관하게 지킨다.

### 플랜 문서 컨벤션

- 플랜 파일: repo 루트 `fable_<주제>_step1_<YYMMDDHHMM>.md` (선례: `fable_gameloop3_step1_2607161330.md` 등). 구현 보고서: `fable_<주제>_step2_report.md`.
- 플랜 필수 구성: Baseline 커밋 명시 → Scope(P-트랙) → **Hard invariants**(이전 슬라이스에서 상속: additive-optional 직렬화, renderer boundary, 액션 prefix/저장 키 동결, 신규 의존성 금지, route graph 불변, reduced-motion resting state) → 검증 명령 → WP 목록(순서 고정, WP당 커밋 1개) → 명시적 범위 밖 → 최종 체크리스트.
- 구현자는 WP를 순서대로 하나씩: 검증 → 커밋 → 다음. 위험해 보이는 WP는 스킵하고 사유를 커밋+보고서에 남긴다.

### 리뷰 사이클

- 구현이 머지되면 플랜 작성자가 diff를 리뷰한다. 리뷰는 보고서를 믿지 않고 코드/실화면으로 검증한다 (선례: 존재하지 않는 능력치 id, 죽은 코드, CSS 0줄, 셀렉터 오매치 등은 전부 리뷰에서만 잡혔다).
- 발견 결함은 심각도 표로 정리 → 수정은 sonnet subagent에 구체적 지시서로 위임 → 수정 diff를 다시 검수 → `fix:` PR. 수정 지시서에는 파일 경로·정확한 원인·기대 결과·검증 명령을 명시한다.
- 시각 폴리시는 플랜 범위에서 명시적으로 제외하고 별도 step으로 진행한다. 그 대신 신규 마크업의 클래스/데이터 속성을 플랜에서 계약으로 고정한다.

### PR 운영 규칙

- 스택 PR은 **base 쪽을 먼저 머지하고, 위 PR의 base가 main으로 재타게팅된 것을 확인한 뒤** 머지한다 (2026-07-12 사고: 11초 차이로 게임 프레임 커밋이 main에 못 들어감).
- main이 스쿼시 머지라 브랜치가 CONFLICTING이 되면 원 브랜치를 고치지 말고 **최신 main 위에 해당 커밋만 체리픽한 새 브랜치로 재착지**한다.
- PR 본문은 문제→수정→검증 순서로, 검증 수치(테스트 수, QA 뷰포트)를 포함한다.

### 시각/게임필 방향 (확정 결정, 임의 변경 금지)

- **수묵/먹선은 폐기됐다 (2026-08 사용자 지시).** 이 줄은 오랫동안 "수묵 천기록 — SVG 먹 실루엣 + 한지 토큰만 사용"을 확정 방향으로 지시하고 있었으나 실제로는 이미 버려진 컨셉이었다. 폐기가 전파되지 않은 이유는 기록해 둔다: 아트 트랙이 2026-07-17 이후 멈췄고(`web/src/ui/storybook/ink|art`, `web/public/assets/art` 마지막 커밋 `5ab25cb`), **먹선 SVG가 폴백 경로라 절대 실패하지 않기 때문이다** — 등록 안 된 `visual_id`는 조용히 SVG로 떨어지고(번들에 `placeholder: true` 47개), 그럴듯한 뭔가가 나오니 버그 리포트가 생기지 않는다. 폴백은 조용하다.
- **토큰 규율은 유효하다**: 신규 색상 리터럴 금지, 색은 모듈당 팔레트 테이블 한 곳에만. 기존 CSS 커스텀 프로퍼티를 계속 쓰되 `--ink`/`--paper*` 같은 이름은 수묵 유산이며 이름이 방향을 지시하지 않는다.
- **전투 표면 시각 방향 (현재 canonical)**: 전투는 Three.js **고정 카메라 3D axial hex auto-battler**가 primary다. Rust core가 게임 truth를 소유하고 renderer는 presentation-only로 동작하며, 결정적 replay를 보장한다. 보드 캐릭터는 절대적으로 얼굴을 배제하지 않으며, **shared-rig modular semi-SD GLB 캐릭터**를 우선한다. 상태 얼굴은 존재할 수 있지만 비의미적(non-semantic) 장식이며, 감정과 서사는 authored 2D portrait가 담당한다. 진영은 색만이 아니라 shape-first 규칙으로 읽히게 한다. 기존 2D combat-primary 및 absolute faceless/code-generated-only 제약은 폐기·대체한다. 상세 계약은 docs/design/ThreeJS_Combat_Visual_Architecture.md를 정본으로 삼는다.
- **접근성 및 폴백**: reduced motion, forced colors, DOM fallback을 계속 지원한다. Three.js 전투 연출은 이 조건들에서도 핵심 정보와 조작을 잃지 않아야 한다.
- **PC-first 3D 성능/품질 기준**: 현재 3D 작업의 대표 타깃은 1080p/60fps의 중급형 Windows PC다. 최종 성능 예산을 먼저 고정하지 않고 시각 품질 탐색을 우선하며, 측정 결과를 바탕으로 예산을 확정한다. 모바일 최적화와 품질 tier는 후속 작업이다. 모바일 우선 성능 제한이나 영구적인 no-shadows/no-postprocessing 규칙을 두지 않는다. 다만 아키텍처 수준의 효율성, reduced motion·forced colors·DOM fallback은 계속 보존한다.
- **비공개 학습 레퍼런스 정책**: 이 비공개 프로젝트에서는 외부 레퍼런스 코드·에셋을 직접 연구하고 필요하면 적용할 수 있다. 사용한 출처와 적용 범위는 최소한의 provenance note로 남기며, 공개·배포 전에 라이선스/유사성/권리 리스크를 다시 감사한다.
- Three.js 전투의 예시 이미지·외부 저장소·기준 commit은 `docs/design/references/threejs_combat/README.md`에 고정한다. 캐릭터·VFX·화면 구성 WP는 정본 다음에 이 팩을 읽는다.
- **게임필 (2026-07-12 사용자 인터뷰 확정)**: 모바일 텍스트RPG 게임 프레임. 엄격한 3분할(상단바 장소·장/턴 / 스크롤 뷰포트 / 하단바 체력·정신력·천기 게이지) — 본문이 바 영역을 침범하면 회귀다. 카드형 선택지, 판정은 주사위+成/敗 도장 연출, 타자기식 본문 출력(탭으로 완성), 행동 결과는 선택한 화면에서 결과 비트로 먼저 표시 후 전환(시간 자동넘김 없음).
- 용어: 자원은 **체력/정신력**으로 통일 (몸/마음 표기는 폐기됨).
- 모든 애니메이션은 reduced-motion에서 최종 프레임이 올바른 정지 상태여야 한다 (fill-mode both, 지속 루프는 `prefers-reduced-motion: no-preference` 블록으로 분리).

### 실화면 QA (웹 변경 시 필수)

- 공식 게이트: `cd web && npm run qa:storybook:visual -- --base-url <dev-url> --out-dir <scratch>` — 5개 뷰포트 전 항목 통과. 라이브 게임 플로우 검증에는 wasm 재빌드가 선행되어야 한다 (`wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg`, WSL).
- artManifest에 WebP 일러스트 자산을 추가·변경할 때는 `cd web && npm run qa:art-assets`를 실행해 파일 존재, WebP 형식, 150KB 이하, 5:3 비율을 검증한다.
- 연출(타자기/비트/플로팅)은 QA 스크립트가 reduced-motion이라 꺼진다 — 별도 Playwright 컨텍스트(`reducedMotion: 'no-preference'`)로 동작 증빙을 남긴다.
- 픽스처 시각 검증 요령: vite dev의 TS 서빙으로 `page.evaluate(() => import('/src/ui/storybook/render.ts'))` 후 임의 ScenePage를 렌더하면 실제 CSS로 스크린샷을 뜰 수 있다.

## 계획 문서 우선순위

- `docs/dev/Development_Plan.md`가 이 저장소의 canonical main plan이다. 현재 방향, 다음 작업, 우선순위, phase 순서는 이 파일을 기준으로 판단한다.
- 사용자가 “다음 작업”, “계속해”, “계획대로 해”처럼 말하면 먼저 `docs/dev/Development_Plan.md`의 상단 우선순위와 “현재 최우선 남은 작업” / “다음 액션”을 확인한다.
- `docs/dev/Checklist.md`는 완료 여부 추적용이며, 독립적인 다음 계획 source가 아니다.
- 아키텍처/스키마 문서는 구현 계약 참조이고, README는 실행법과 문서 입구다.
- `.hermes/plans/`는 일회성 세션 artifact이며 canonical 계획으로 쓰지 않는다.

## idea_box 운영 규칙

이 프로젝트에는 `idea_box/`가 있다. 사용자가 별도 세션에서 떠오르는 아이디어를 저장하는 공간이다.

### 확인 우선순위

- 남아 있는 plan, todo list, 또는 명시된 사용자 지시가 있으면 그것을 먼저 따른다.
- 모든 작업 시작 전에 `idea_box`를 확인하지 않는다. 이는 현재 작업의 우선순위를 흐릴 수 있다.
- 현재 세션에서 처리할 남은 plan/todo가 없을 때만 `idea_box/README.md`, `idea_box/BACKLOG_ORDER.md`, `idea_box/inbox/`의 열린 아이디어를 확인해 다음 설계/개발 항목을 찾는다.
- 사용자가 직접 `idea_box` 확인을 요청한 경우에는 즉시 확인한다.

### Local design-source 및 Notion mirror 파이프라인

현재 디자인 레코드의 정본은 Git repo 내부의 `docs/content/design_source/`다. 별도 DB/저장소가 아니며, 스토리·사건·선택지·후일담·보상·관계·기획 provenance를 설계/변경/감사할 때 참조하는 기획 SSoT다. 범위는 current design records로 한정하며 runtime graph/generated/game-code contract의 정본이 아니다. 일반 엔진/UI/빌드/전투 코드 작업에서는 전체 폴더를 매번 읽지 않고, 콘텐츠 ID·보상·분기 계약 구현 때만 manifest와 관련 소수 레코드를 선택 참조한다. Notion은 읽기·검수 미러다.

표준 흐름은 다음 순서다.

1. 사용자가 local design source에 아이디어를 정리하거나 기존 레코드를 검토한다. Notion-origin 아이디어는 필요할 때만 보존된 provenance를 참조한다.
2. agent는 manifest/governance를 먼저 읽고 관련 local records를 선택 참조한다. Notion provenance가 필요한 경우에만 repo 안의 설계 아이디어 문서와 `idea_box/inbox/*.md`에 page id/title/url과 `related_docs`를 기록한다.
3. 다음에 실제로 설계할 항목은 설계 아이디어 문서 중 하나를 `docs/dev/Development_Plan.md`의 active main plan / “현재 최우선 남은 작업”으로 격상시킨 뒤 진행한다.
4. 설계가 끝나면 local design source와 결과 설계 문서를 비교해 방향, 핵심 제약, non-goals가 어긋나지 않았는지 확인한다. 필요하면 Notion mirror에도 검수 결과를 반영한다.
5. local design source 반영 또는 명시적 폐기/병합 기록까지 끝난 뒤에만 해당 idea entry를 `done` 처리한다. 단순 import, 단순 요약, 또는 설계 아이디어 문서 작성만으로는 `done`이 아니다.
6. local design source → review → 필요 시 runtime handoff/구현 → Notion mirror 순서로 수행한다. Runtime 직접 계약은 runtime schema/preview/generated source와 `Development_Plan.md`가 소유하며 design source를 자동 실행 데이터로 간주하지 않는다. Notion 역방향 변경을 current design records의 정본으로 사용하지 않는다. manifest/governance를 먼저 읽고 관련 레코드만 여는 progressive disclosure와 100KB 문서 제한을 따른다.

### 아이디어 처리

- 아이디어는 즉시 현재 작업에 끼워 넣는 요구사항은 아니지만, `status: done`이 아닌 entry는 반영되지 않은 backlog다.
- 남은 plan/todo가 없거나 사용자가 `idea_box` 처리를 요청하면 `idea_box/BACKLOG_ORDER.md`의 Git 최초 추가 순서대로 처리한다.
- Notion-origin entry는 필요할 때 보존된 provenance를 확인하고, 설계 완료 후에는 local design source 반영 또는 폐기/병합 결과를 처리 기록에 남긴다.
- 프로젝트의 톤, 우선순위, 현재 구현 단계에 맞지 않으면 구현하지 않고 폐기/병합 판단을 할 수 있지만, 그 이유를 처리 기록에 남겨야 한다.
- 아이디어를 실제 설계/문서/구현에 사용해 local design source에 반영했거나, 명시적으로 폐기/병합 처리했다면 `done` 처리한다.
- `done`은 단순히 읽었다는 뜻이 아니다. 어디에 반영했는지, 어떤 Notion reference와 대조했는지, 또는 왜 폐기/병합했는지 기록한다.
- 아이디어 파일은 삭제하지 않는다.

자세한 파일 형식과 처리 방식은 `idea_box/README.md`를 따른다.

## AI Agent용 문서 크기 제한 지침 (100KB 제한)

- "AI agent가 읽도록 하는 것이 목표인" 모든 md 문서는 **100kb** 크기를 절대로 넘지 않아야 한다.
- 만약 특정 문서의 크기가 100kb를 초과한다면, 과거 히스토리나 상세 데이터 등을 별도의 아카이브 문서로 분리하거나 요약본을 새로 만들어야 한다.
- Agent는 가급적 요약본이나 최신화된 메인 문서(100kb 이하)를 먼저 참조하고, 필요한 상세 내용이 있을 때만 원본/아카이브 문서를 부분적으로 조회한다.
- 100kb가 넘는 문서는 사람이 직접 관리하거나 거의 읽지 않는 백업/정리용 문서로만 분류한다.
