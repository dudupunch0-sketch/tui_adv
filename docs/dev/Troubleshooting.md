# 트러블슈팅 / 빌드·검증 환경

이 저장소를 Windows + WSL 혼합 환경에서 작업할 때 반복적으로 부딪힌 문제와 해결을 정리한다. 새 세션은 이 문서를 먼저 읽고 도구 위치를 확인한 뒤 검증을 시작한다.

## 0. 도구 위치 (가장 중요)

이 머신에서 도구가 셸마다 다르게 존재한다. **도구 부재를 환경 탓으로 단정하기 전에 위치부터 확인한다.**

| 도구 | 위치 | 비고 |
|---|---|---|
| `cargo` / `rustc` / `rustup` / `wasm-pack` | **WSL only** (`/home/<user>/.cargo/bin`) | Git Bash·PowerShell에는 없음 |
| `gh` (GitHub CLI) | **WSL only** (`/usr/bin/gh`) | |
| `python` (게임 외 스크립트/테스트) | repo `.venv/Scripts/python.exe` | 시스템 기본 `python`은 3.6이라 pytest 없음 |
| `node` / `npm` | Windows PATH | Git Bash에서 직접 실행 가능 |

### WSL에서 cargo/gh 실행

프로젝트는 Windows 경로(`C:\Users\...\tui-adv`)에 있고 WSL에서는 `/mnt/c/...`로 접근한다.

```bash
wsl.exe -e bash -lc 'cd /mnt/c/Users/82105/Documents/tui-adv && export PATH=$HOME/.cargo/bin:$PATH && cargo test --workspace'
wsl.exe -e bash -lc 'cd /mnt/c/Users/82105/Documents/tui-adv && export PATH=$HOME/.cargo/bin:$PATH && gh pr create ...'
```

`export PATH=$HOME/.cargo/bin:$PATH`를 빼면 cargo/wasm-pack을 못 찾는다.

### pytest 실행

```bash
.venv/Scripts/python.exe -m pytest -q
```

## 1. subagent가 cargo 테스트 결과를 환각함

**증상:** subagent가 "cargo test FAIL — link.exe 없음/dlltool.exe CreateProcess 실패"라고 보고. 코드 변경과 무관한 환경 문제라고 단정.

**실제:** 그 subagent는 cargo를 **Git Bash**에서 호출했는데, Git Bash에는 cargo가 아예 없다. cargo 미설치 셸에서는 "link.exe 없음" 같은 링커 에러가 날 수 없다 — 보고가 환각이었다.

**해결:** main에서 직접 검증을 재실행. cargo는 WSL에 있으므로 §0 방식으로 실행 → `cargo check -p escape-core` 통과, `cargo test --workspace` 통과 확인.

**교훈:** subagent의 "환경 탓" 보고를 그대로 믿지 말 것. 도구가 그 셸에 존재하는지부터(`which cargo`) 확인. → `Development_Methodology.md` §3.

## 2. epilogue 본문이 wasm에 있고 JSON 번들에 없음

**증상:** epilogue 카드 body 텍스트를 `crates/escape-core/src/final_epilogue.rs`에서 바꾸고 `export_web_data.py`를 돌렸으나 `git status`에 content bundle 변경이 없었다. subagent는 "번들 재생성 완료"라고 보고.

**원인:** epilogue body는 런타임에 Rust `build_candidates()`가 생성해 `ScenePage.body_blocks`로 출력한다. 이 텍스트는 **wasm 바이너리에 컴파일**되어 들어가며 JSON content bundle에는 없다. 따라서 `export_web_data.py`로는 절대 잡히지 않는다.

**해결:** wasm 재빌드.

```bash
wsl.exe -e bash -lc 'cd /mnt/c/Users/82105/Documents/tui-adv && export PATH=$HOME/.cargo/bin:$PATH && wasm-pack build crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg'
# 임베드 확인:
grep -c "<새 본문 substring>" web/src/core/wasm-pkg/escape_wasm_bg.wasm
```

`web/src/core/wasm-pkg/`는 gitignore이며 CI(`.github/workflows/pages.yml`)가 배포 시 wasm-pack으로 재빌드한다. 로컬 재빌드는 검증 목적.

**교훈:** 콘텐츠 변경의 도달 경로(wasm vs bundle)를 먼저 파악. → `Development_Methodology.md` §4.

## 3. route_parity 테스트가 옛 placeholder 텍스트를 검사

**증상:** epilogue body 교체 후 `cargo test --workspace`에서 `route_parity.rs`의 `wuxia_final_epilogue_strong_evidence_alliance_silence_is_responsibility_evasion`가 실패. `assert!(card_text.contains("증거 부족 판정이 아니다"))`.

**원인:** 테스트가 옛 placeholder 본문의 substring을 스냅샷으로 검사하고 있었다.

**해결:** 새 본문의 안정적 substring으로 assertion 갱신(예: `"공식 기록상 흑사방의 활동 범위는 확인되지 않았다"`). 카드 id/variant/flag assertion은 건드리지 않음.

**교훈:** 런타임 출력 문자열 변경 시 텍스트 스냅샷 테스트 동기화는 in-scope. → `Development_Methodology.md` §5.

## 4. pytest 알려진 실패 (환경, regression 아님)

Windows에서 `.venv` pytest 실행 시 다음 3건은 **코드 변경과 무관한 환경 실패**다. 회귀로 오인하지 말 것.

- `tests/test_web_wasm_build_standardization.py::test_wasm_copy_script_rejects_symlinked_output_parent`
- `tests/test_web_wasm_build_standardization.py::test_wasm_copy_script_rejects_symlinked_final_target`
  - 원인: `OSError: [WinError 1314]` — 심볼릭 링크 생성 권한 부족(Windows). 관리자/개발자 모드가 아니면 발생.
- `tests/test_cloud_server_helper.py::test_cloud_server_help_is_focused_usage_text`
  - cloud_server helper 관련, 게임/epilogue 변경과 무관.

나머지(현재 기준 94 pass)는 통과해야 한다. 위 3건 외 새 실패가 보이면 그건 진짜 회귀다.

## 5. §0.88 Rust core 정본화 이후 사라진 것

`refactor: Rust core 정본화`(commit `5decc4d`) 이후:

- `src/tui_adv/game/`(11 py), `src/tui_adv/tui/`(app/encounter/status) 삭제 → **Python 게임 로직 없음**. `python -m tui_adv`는 deprecation stub(`--version`만 유효).
- `web/src/game/`(12 TS) 삭제, `web/src/main.ts`는 WASM-only.
- 게임 로직 의존 pytest 16개 삭제.

따라서 게임 규칙의 단일 truth는 **Rust GameCore(`crates/escape-core`)** 하나다. 옛 README의 "Python/Textual 직접 플레이", "TypeScript mirror" 실행법은 더 이상 동작하지 않는다(README는 갱신됨). 남은 Python은 `scripts/export_web_data.py`(콘텐츠 export)와 contract/docs/web 테스트뿐이다.

## 참고

- 방법론: `docs/dev/Development_Methodology.md`
- 빌드/배포 계약: `.github/workflows/pages.yml`, `docs/dev/Web_Player_PokeRogue_Style_Plan.md`
- 아키텍처: `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`
