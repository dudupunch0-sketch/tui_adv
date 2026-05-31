# escape from the office

TUI 기반 랜덤 인카운터 선택지 생존 게임 엔진/콘텐츠 프로젝트.

현재 기본 storypack은 회사 사무실 아포칼립스인 `escape from the office`다. 플레이어는 갑자기 사람이 모두 사라지고 공간 또는 차원 단위로 격리된 대기업 연구개발동에서 시작한다. 다만 장기 개발 방향은 회사 전용 게임이 아니라, storypack/world를 바꿔 회사 자각몽 `yageunmong_pack`이나 무협 `wuxia_jianghu_pack` 같은 다른 세계관/전제를 같은 Rust GameCore, Web Storybook, SuperLightTUI renderer 계약으로 플레이할 수 있는 구조다. `yageunmong_pack`은 기본 office runtime을 대체하지 않는 office-family 후보이고, 첫 비-office 기준팩은 현대 회사원이 본인 몸과 출근복장 그대로 무협 세계에 전이되는 **이구학지 — 천기록**이다.

## 현재 단계

현재 구현은 Python/Textual TUI, TypeScript mirror core 기반 브라우저 fake-TUI, 그리고 Rust content runner가 함께 있는 전환기 상태다. 장기 방향은 이 셋을 그대로 키우는 것이 아니라 **Rust GameCore 공통 + Web Storybook/GlyphFX primary UX + SuperLightTUI terminal renderer/fallback**으로 재정렬하는 것이다.

중요한 기준:

- Web Storybook + GlyphFX가 플레이어용 메인 UX 후보다. 이미지/장면 컷, 대화 내역, 읽기 중심 선택지, Canvas/GlyphFX는 이 경로에서 먼저 구현한다.
- Web Storybook의 현재 시각 기준은 `docs/design/Mobile_Pixel_Storybook_UI.md`다. 웹에서 실행되더라도 generic web dashboard가 아니라 모바일 세로형 픽셀 게임북 board로 보이게 한다.
- Rust terminal 경로는 버리지 않는다. `escape-terminal`의 content 경로는 SuperLightTUI snapshot/play renderer와 `--app` full-screen SuperLightTUI app loop를 제공하며, visual card/GlyphFX/input 안내 polish와 tick/raw-draw GlyphFX baseline이 추가되어 **terminal-native horror edition** 기준을 강화했다. terminal은 fallback이지만 debug dump가 아니다.
- Python/Textual과 기존 Web fake-TUI dashboard는 당분간 legacy/parity oracle로 유지하되, 새 시각/상호작용 투자는 `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`의 방향을 따른다.

게임 구조와 안전한 현실 연결 원칙을 문서화했고, 순수 게임 상태 모델, 자원 임계치/실패 판정, 1차 사무실 위치 모델, 인접 위치 이동과 위험도 누적, 인카운터/선택지 조건·비용·결과 적용, 선택 불가 선택지의 이유 표시, 능력치 기반 선택지, 2d6 성공/실패 분기, 현재 상태 기반 인카운터 선택, 공간 왜곡 탈출/실패 엔딩 판정, YAML 공개 콘텐츠 로더/검증, YAML 기반 런타임 기본 위치/인카운터/엔딩, 로컬 비공개 현실 힌트 로더, 복합기/커피머신/화이트보드 더미 숫자 합계 퍼즐, 현실 연결 히든 엔딩 보상 출력, CLI 한 턴 실행, CLI 다중 턴 스크립트 실행, Textual 레이아웃 smoke, Textual 저장/불러오기 연결, TUI 저장/종료 단축키, TUI 저장 파일 목록·시작 슬롯 선택·삭제 패널, 도움말/이동 단축키/상세 도움말·인벤토리·로그 패널, 압박 경고 패널, Textual 그리드 패널과 터미널 테마 CSS, 소모품 아이템 사용, 물품창고 보급품, 엘리베이터/옥상 경로, 옥상 외부 신호 탈출 엔딩, 저정신력 선택지 왜곡, 고갈증 정수기 환각, 엘리베이터-보안실 우회 분기, 임계 자원 1회성 경고 로그, 보안실-서버실 격리 권한 정복 루트, 지하주차장 키태그/차단기 탈출 루트, 로비 방문증/회전문 탈출 루트, 대표실 결재 콘솔 정복 루트, 진실 루트의 재난 원인 문서, 생존자 설득/시스템 제압 설계, 세 번째 현실 연결 힌트 체인, 로컬 secret 템플릿과 현실 연결 안전 점검 문서, YAML→브라우저 JSON export, Vite 기반 fake-TUI 브라우저 셸, localStorage 저장, 복합기 현실 연결 pretext/Canvas 장면, 전 루트 웹 parity 테스트, 브라우저 아이템 사용·업적·능력치 판정·압박 UI, 인벤토리·업적·컨트롤·압박 패널, Rust content runner의 잠긴 선택지/위험도 parity까지 추가했다.

## 실행/테스트

개발 중에는 설치 없이 다음처럼 실행할 수 있다.

기존 Python/Textual 직접 플레이:

```bash
PYTHONPATH=src python -m tui_adv --play --seed 123
```

Rust content-backed 직접 플레이:

```bash
cargo run -p escape-terminal -- --scene content --content-bundle crates/escape-core/fixtures/content/content.bundle.json --seed 123 --play
cargo run -p escape-terminal -- --scene content --content-bundle crates/escape-core/fixtures/content/content.bundle.json --seed 123 --app
```

주의: `--play`는 scriptable/stdin-friendly 직접 플레이이고, `--app`은 full-screen SuperLightTUI app loop다. visual card는 `ScenePage.visual`의 id/layout/alt를 terminal card로 표시하고, GlyphFX fallback은 intensity meter와 stable terms/fallback text를 보존한다. `--app-smoke --tick`은 같은 app-frame renderer의 tick/raw-draw GlyphFX를 headless로 검증한다. 개인 서버/WSL에서 `cargo`가 없고 `/home` 용량이 부족하면 Rust/Cargo 경로를 `/tmp`로 돌려 구성한다.

```bash
export RUSTUP_HOME="/tmp/$USER-rustup"
export CARGO_HOME="/tmp/$USER-cargo"
export CARGO_TARGET_DIR="/tmp/$USER-tui-adv-target"
export PATH="$CARGO_HOME/bin:$PATH"
mkdir -p "$RUSTUP_HOME" "$CARGO_HOME" "$CARGO_TARGET_DIR"
rustup toolchain install stable --profile minimal --component rustfmt --component clippy
```

`cloud_server_only.sh`는 이 repo를 작업하던 제한된 개발 서버에서 Rust 캐시/빌드 산출물을 `/tmp`로 돌리기 위한 helper다. 일반 개인 서버나 WSL 실행법의 기본값이 아니다.

```bash
./cloud_server_only.sh install
./escape-terminal-cloud-server-only --seed 123
```

Rust headless smoke:

```bash
cargo run -p escape-terminal -- --scene content --content-bundle crates/escape-core/fixtures/content/content.bundle.json --seed 123 --smoke --action choice:check_message --action move:dev_office
cargo run -p escape-terminal -- --scene content --content-bundle crates/escape-core/fixtures/content/content.bundle.json --seed 123 --tui-smoke --action choice:check_message
cargo run -p escape-terminal -- --scene content --content-bundle crates/escape-core/fixtures/content/content.bundle.json --seed 123 --tui-smoke --action choice:check_message --action move:dev_office --action move:printer_area
cargo run -p escape-terminal -- --scene content --content-bundle crates/escape-core/fixtures/content/content.bundle.json --seed 123 --app-smoke --tick 7 --action choice:check_message --action move:dev_office --action move:printer_area
```

스크립트/스모크 실행:

```bash
PYTHONPATH=src python -m tui_adv --new --seed 123
PYTHONPATH=src python -m tui_adv --new --seed 123 --choice 1
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:server_room_front --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --location printer_area --action choice:2 --action move:pantry --action choice:3
PYTHONPATH=src python -m tui_adv --new --seed 123 --location printer_area --action choice:3 --action move:pantry --action choice:3
PYTHONPATH=src python -m tui_adv --new --seed 123 --location printer_area --action choice:1 --action move:dev_office --action move:meeting_room --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --location emergency_stairs --flag escape_puzzle_ready --choice 1
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:supply_closet --action choice:2 --action use:power_bank
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:elevator_hall --action choice:1 --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --location elevator_hall --action choice:2 --action choice:1 --action move:hallway --action move:server_room_front --action choice:4 --action choice:3
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:parking_lot --action choice:1 --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:lobby --action choice:1 --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:lobby --action choice:2 --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --location pantry --resource thirst=70 --action choice:1
PYTHONPATH=src python -m tui_adv --tui-smoke --seed 123 --resource sanity=30
PYTHONPATH=src python -m tui_adv --tui-smoke --seed 123 --save saves/autosave.json
PYTHONPATH=src python -m tui_adv --tui-smoke --seed 123 --save saves/autosave.save --delete-save-slot 1
PYTHONPATH=src python -m tui_adv --tui-smoke --seed 123
PYTHONPATH=src python -m tui_adv --tui --seed 123 --save saves/autosave.json  # Textual 설치 환경에서 실행, s 저장/q 종료
PYTHONPATH=src python -m tui_adv --tui --load saves/autosave.json --save saves/autosave.json
PYTHONPATH=src python -m tui_adv --version
```

테스트:

```bash
python -m pytest tests -q
PYTHONPATH=src python scripts/qa_smoke.py
PYTHONPATH=src python scripts/textual_qa_smoke.py  # Textual 설치 환경에서 실행
```

브라우저 Web Storybook / renderer-neutral content bundle:

```bash
python scripts/export_web_data.py --write --bundle crates/escape-core/fixtures/content/content.bundle.json --bundle web/src/data/generated/content.bundle.json
python scripts/export_web_data.py --check --bundle crates/escape-core/fixtures/content/content.bundle.json --bundle web/src/data/generated/content.bundle.json
cd web
npm install
npm test
npm run build
npm run dev -- --host 127.0.0.1 --port 8765
```

무협 storypack preview bundle은 기본 office bundle과 분리된 별도 artifact다.

```bash
python scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check

cargo run -p escape-terminal -- \
  --scene content \
  --storypack-preview wuxia_jianghu_pack \
  --seed 123 \
  --tui-smoke

cargo run -p escape-terminal -- \
  --scene content \
  --storypack-preview wuxia_jianghu_pack \
  --seed 123 \
  --tui-smoke \
  --action choice:follow_roadside_dust \
  --action move:jianghu_market_street \
  --action choice:run_toward_open_street
```

Rust/WASM-primary Web preview/build는 generated wasm package를 먼저 만든다.

```bash
cd web
npm run wasm:build
npm run build:wasm
npm run preview:wasm
npm run build:player
npm run preview:player
```

`npm run wasm:build`는 `wasm-pack build ../crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg`를 실행한다. 생성되는 `web/src/core/wasm-pkg/`는 로컬 build artifact라서 Git에 커밋하지 않는다. `npm run build:wasm`은 Vite build 뒤 `npm run wasm:copy`를 실행해 이 generated package를 `web/dist/assets/wasm-pkg/`로 복사하므로 player artifact의 dynamic WASM import 경로가 함께 배포된다.

현재 브라우저 앱은 Web player start screen을 먼저 표시한 뒤 Web Storybook + GlyphFX renderer를 기본 플레이 화면으로 사용한다. 시작 화면은 localStorage 기반 이어하기/새 게임/seed 표시/save timestamp/reset confirmation과 별도 `wuxia_jianghu_pack` storypack preview launcher를 제공한다. 기본 office run은 `escape-office.rust.save.v1` 계열 key를 유지하고, storypack preview run은 그 key를 쓰지 않는다. `web/src/core/wasmRuntime.ts`가 generated content bundle(`web/src/data/generated/content.bundle.json`)을 `escape-wasm` JSON-string boundary에 전달해 Rust GameCore의 `ScenePage`/`ActionResult`를 소비한다. `web/src/core/contentBundles.ts`는 기본 office bundle과 Web generated storypack preview bundle을 분리한다. Rust/WASM-primary preview는 `npm run build:wasm` 또는 `npm run preview:wasm` 경로로 확인한다. generated wasm package가 없거나 `wasm-pack`/Rust toolchain이 없는 개발 환경에서는 legacy TypeScript mirror가 fallback/parity oracle로 동작한다. legacy TypeScript mirror와 Python/Textual은 freeze 상태이며, 새 게임 규칙은 Rust GameCore에만 추가한다. 공개 secret JSON과 content bundle에는 실제 사무실 최종 위치나 `final_hint`를 넣지 않는다.

배포 표면은 현재 Web-only로 결정했다. `npm run build:player`는 Rust/WASM-primary Web 정적 산출물(`web/dist/`)을 만들고, `npm run preview:player`는 같은 경로를 로컬 preview한다. Tauri/Electron은 desktop wrapper의 고유 가치가 생길 때까지 deferred 상태이며, 결정 기록은 `docs/dev/Web_Distribution_Decision.md`에 둔다.

패키지 설치 후에는 `tui-adv` console script를 사용할 수 있다.

## 핵심 설정

- 기본 storypack 제목: `escape from the office`
- 프로젝트 방향: storypack/world 기반 TUI 선택지 생존 게임 + Web Storybook/GlyphFX primary UX + SuperLightTUI terminal renderer
- 기본 톤: 블랙코미디 회사 괴담 + 코스믹 호러
- office-family 후보팩: `yageunmong_pack` / **야근몽**. 전제는 “회사에서 잠깐 잠든 주인공이 자각몽 상태의 회사 악몽에서 업무 완료가 아니라 깨어나기를 목표로 한다”이며, 기본 office runtime을 대체하지 않는다.
- 첫 비-office 기준팩: `wuxia_jianghu_pack` / **이구학지 — 천기록**. 전제는 “현대 회사원이 본인 몸과 출근복장 그대로 무협 세계 시장에 전이되고, 천기록/천외편린 성장 구조를 경험한다”
- 1차 재난 타입: 불명 재난
- 상황: 사람 실종, 연구개발동 규모의 공간/차원 격리, 제한된 외부 인터넷, 간헐적 사내망 연락
- 핵심 자원: 체력, 정신력, 배터리, 허기, 갈증
- 판정 능력치: 논리, 공감, 의지, 침착, 인터페이스, 신체
- 주요 루트: 탈출, 정복, 진실 발견, 히든 현실 연결
- 렌더러 방향: Rust GameCore가 게임 규칙의 truth를 소유하고, Web Storybook/GlyphFX가 primary UX, SuperLightTUI 기반 Rust terminal이 terminal-native fallback/horror edition을 담당한다. 기존 TypeScript mirror core와 fake-TUI 패널은 전환기 parity/legacy 구현이다.

## 문서

- `AGENTS.md`: agent 작업 지침과 `idea_box` 확인 우선순위
- `docs/00_Index.md`: 전체 문서 구조
- `docs/01_Game_Overview.md`: 게임 개요
- `docs/story/Story.md`: 스토리와 오프닝
- `docs/story/Disaster_Cause.md`: 재난 원인과 진실 루트 공개-safe 설계
- `docs/story/Disaster_Type_Extension.md`: 재난 타입 확장 규칙과 후보 백로그
- `docs/story/Dream_Ending_Branching.md`: 꿈 엔딩/깨어난 뒤 재시험 문법 설계 후보
- `docs/story/Real_Escape_Ending_Branching.md`: 현실 탈출 후 결과 정산/후일담 설계 후보
- `docs/story/Reality_Link.md`: 현실 연결 원칙
- `docs/design/Player_State.md`: 플레이어 상태 규칙
- `docs/design/Character_Stats_and_Generator.md`: 6스탯 등장인물/LLM 생성기 설계 후보
- `docs/design/Storypack_World_Model.md`: storypack/world 기반 일반화 기준, 야근몽 office-family 후보, 무협 기준팩 방향
- `docs/design/Game_Loop.md`: 턴/선택/인카운터 루프
- `docs/design/Combat_System_Auto_Brawl.md`: 자동 난투 + 상황 개입 전투 설계 후보
- `docs/design/Map.md`: 1차 맵 설계
- `docs/design/UI_Rules.md`: 사내 시스템형 TUI, 글리치, 선택지 오염 규칙
- `docs/design/TUI_Storybook_GlyphFX_Concept.md`: Web primary UX로 채택한 TUI풍 스토리북 + GlyphFX 방향
- `docs/design/Mobile_Pixel_Storybook_UI.md`: Web Storybook의 모바일 세로형 픽셀 게임북 board 시각 contract
- `docs/dev/Development_Plan.md`: canonical main plan. 현재 방향, 다음 작업, 우선순위의 source of truth
- `docs/dev/Checklist.md`: 단계별 완료 여부 추적용 체크리스트
- `docs/dev/Storypack_Runtime_Preview_Mode.md`: 무협 runtime prototype을 기본 office bundle과 섞지 않는 preview mode 결정
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`: `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment` preview source YAML
- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`: Rust/GameCore용 무협 preview fixture bundle
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`: Web/WASM용 무협 preview generated bundle
- `web/src/core/contentBundles.ts`: Web default office bundle과 storypack preview bundle registry
- `docs/content/Location_List.md`: 1차 위치 목록
- `docs/content/Item_List.md`: 1차 아이템 목록
- `docs/content/Encounter_List.md`: 1차 인카운터 목록
- `docs/content/Ending_List.md`: 1차 엔딩 목록
- `docs/content/Survivor_System_Routes.md`: 생존자 설득과 시스템 제압 루트 설계
- `docs/content/Secret_List.md`: 공개 가능한 히든 루트/비밀 목록
- `docs/content/Horror_Ideas.md`: 호러 연출 아이디어 저장소
- `docs/content/storypacks/yageunmong_pack.md`: 야근몽 office-dream storypack 후보
- `docs/content/storypacks/wuxia_jianghu_pack.md`: 이구학지 — 천기록 첫 비-office 무협 storypack 후보
- `docs/content/encounter_db/yageunmong_pack.md`: 야근몽 encounter situation cards
- `docs/content/encounter_db/wuxia_jianghu_pack.md`: 이구학지 — 천기록 encounter situation cards
- `docs/content/storypack_db/README.md`: machine-readable storypack/card 후보 DB와 검증 범위
- `docs/content/storypack_db/storypacks.json`: machine-readable storypack 후보 record DB
- `docs/content/storypack_db/encounter_situations.json`: machine-readable encounter situation card DB
- `docs/dev/Architecture.md`: 코드 구조와 모듈 경계
- `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`: Rust GameCore + Web Storybook + SuperLightTUI terminal 활성 방향
- `docs/dev/Data_Schema.md`: YAML/JSON 데이터 스키마
- `docs/dev/TUI_Layout.md`: TUI 화면 설계
- `docs/dev/Save_Slot_UX.md`: 저장 슬롯 이름 변경 UX 후보
- `docs/dev/Balance_QA_Packaging.md`: 밸런싱, QA smoke, 패키징/릴리즈 기준
- `docs/dev/Web_Distribution_Decision.md`: Web-only 배포 표면과 Tauri/Electron defer 결정
- `docs/dev/Web_Player_PokeRogue_Style_Plan.md`: 포켓로그식 URL 즉시 플레이 Web player 배포/UX 구현 계획
- `docs/dev/Final_QA_Log.md`: 실제 Textual/터미널 크기/10회 새 게임 QA 기록
- `docs/implementation-map/index.html`: 현재 구현을 한 번에 보는 interactive HTML 구현 지도
- `docs/implementation-map/README.md`: 구현 지도 업데이트 방법
- `web/`: Vite 기반 브라우저 fake-TUI 수직 슬라이스
- `scripts/export_web_data.py`: Python YAML 콘텐츠를 브라우저 JSON으로 export/check하는 스크립트
- `docs/dev/Reality_Secret_Safety_Checklist.md`: 로컬 현실 힌트 안전 점검
- `docs/templates/local-secrets.template.yaml`: `private/secrets.local.yaml` 작성용 공개 안전 템플릿
- `docs/archive/idea_0515.md`: 2026-05-15 원본 아이디어 노트
- `idea_box/README.md`: 별도 세션에서 떠오른 아이디어를 보관하고 처리하는 규칙

## idea_box

사용자가 별도 세션에서 떠오르는 아이디어를 저장하기 위해 `idea_box/`를 사용한다.

중요한 원칙: 모든 작업 시작 전에 `idea_box`를 확인하지 않는다. 남아 있는 plan, todo list, 또는 명시된 사용자 지시가 있으면 그것을 먼저 처리한다. 현재 세션에서 더 이상 진행할 plan/todo가 없을 때만 `idea_box`를 확인해 다음 설계/개발 후보를 찾는다.

아이디어-설계 흐름은 Notion-first다. 원본 reference는 Notion 문서이며, repo-local `idea_box/inbox/*.md`는 Notion page id/title/url과 처리 순서, 관련 설계 문서 포인터를 담는다. 표준 흐름은 Notion 정리 → repo 설계 아이디어 문서 변환 → `docs/dev/Development_Plan.md` main plan 격상 → 설계 진행 → Notion reference 대조 → `done` 처리다.

자세한 운영 규칙은 `idea_box/README.md`와 `AGENTS.md`를 따른다.

## 현실 연결 안전 원칙

이 게임은 현실 사무실의 메모/보물 위치를 히든 루트로 연결할 수 있다.
단, 공개 저장소에는 실제 최종 위치를 넣지 않는다.

- 공개 문서에는 구역/사물/행동 수준의 중간 힌트까지만 둔다.
- 실제 최종 위치는 `private/` 또는 `*.local.*` 파일에만 둔다.
- `private/`와 local secret 파일은 `.gitignore`로 커밋을 차단한다.
- 개인 책상, 잠긴 공간, 위험 설비, 회사 기밀과 관련된 위치는 사용하지 않는다.

## 다음 작업 기준

다음 작업의 source of truth는 `docs/dev/Development_Plan.md`다. README에는 실행법과 문서 입구만 유지하고, 긴 next-task 목록을 복제하지 않는다.

다른 LLM/agent에게 이어서 작업을 맡길 때는 다음처럼 지시한다.

```text
docs/dev/Development_Plan.md를 메인 플랜으로 보고, 그 안의 “현재 최우선 남은 작업”과 “다음 액션”부터 진행해라.
```

역할 구분:

- `docs/dev/Development_Plan.md`: 현재 방향, 우선순위, 다음 작업 순서.
- `docs/dev/Checklist.md`: 완료 여부 체크만 추적.
- `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`, `docs/dev/Data_Schema.md`: 구현 계약 참조.
- `idea_box/`: active plan/todo가 없거나 사용자가 명시적으로 요청했을 때 보는 backlog.
- `.hermes/plans/`: 일회성 세션 artifact이며 canonical 계획이 아니다.
