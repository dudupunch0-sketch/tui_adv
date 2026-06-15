# 개발 방법론 (재사용 가능)

이 문서는 이 저장소에서 반복 적용 가능한 개발 운영 방법론을 정리한다. 특정 슬라이스의 구현 기록이 아니라, **다음 세션/다른 agent가 그대로 따라 할 수 있는 절차**를 목적으로 한다. 모델 tier 위임 규칙은 `AGENTS.md`의 "Subagent 모델 tier"가 canonical이며, 이 문서는 그 위에서 실제 작업 루프를 어떻게 도는지를 다룬다.

## 1. 작업 루프 (plan → implement → verify → report)

한 슬라이스 단위로 다음을 반복한다.

1. **Plan (Opus, main).** canonical 문서(`docs/dev/Development_Plan.md`)에서 다음 슬라이스를 확정한다. 범위/제약/non-goals를 명시한다. 큰 문서 정독·대량 코드 탐색은 subagent(Explore/haiku)에 위임하고 결론만 받아 메인 컨텍스트를 가볍게 유지한다.
2. **Implement (Sonnet subagent).** 계획된 슬라이스의 실제 코드/콘텐츠 구현. subagent 프롬프트는 **자기완결**이어야 한다 — 파일 경로, 정확한 old/new 문자열, 제약, 검증 명령, 보고 형식까지 포함한다. subagent는 이 대화의 맥락을 모른다.
3. **Verify.** 아래 §3의 "주장 신뢰 금지" 원칙에 따라 핵심 검증은 main에서 직접 재실행한다.
4. **Report.** 슬라이스마다 아주 짧게 보고한다. plan 단계와 implement 단계를 명확히 구분해 표기한다.

## 2. 완성 판정은 canonical 추적 문서 기준

"모두 구현했다"를 주장하기 전에 **반드시 인벤토리**한다. 기억이나 인상이 아니라 파일 기준으로 판단한다.

- `docs/dev/Checklist.md` 미체크 박스(`[ ]`/`[~]`/`[!]`) 수를 grep으로 센다.
- `docs/dev/Development_Plan.md` 상단 "현재 최우선 남은 작업" / "다음 액션"과 최신 섹션을 확인한다.
- 발견된 "보류/deferred"가 **누락**인지 **의도적 scope 결정**인지 구분한다. 사용자가 명시적으로 보류한 항목은 미완료가 아니라 *범위 밖*이다.

`in-scope 완성`과 `전체 목표 완성`을 절대 혼동하지 않는다. 경량 설계 철학 안에서 끝난 것과, 사용자가 보류한 무거운 신규 엔진 레이어는 다른 범주다.

## 3. 검증 신뢰 원칙 — "주장 신뢰 금지, 직접 재실행"

> 이번 세션에서 subagent가 "cargo test가 link.exe 부재로 실패"라고 보고했으나, **해당 셸에는 cargo 자체가 없었다**(환각). 실제 cargo는 WSL에만 있었다. 또 subagent의 "content bundle 재생성" 보고도 부정확했다(git에 번들 변경이 없었음 — 해당 텍스트는 번들이 아니라 wasm 런타임 출력이라 애초에 번들에 안 들어감).

따라서:

- **subagent의 PASS/FAIL 보고를 그대로 믿지 않는다.** 영향이 큰 검증(테스트, 빌드, 산출물 반영 여부)은 main에서 직접 재실행한다.
- **"실패 원인이 환경 문제"라는 보고는 특히 의심한다.** 도구 자체가 그 셸에 존재하는지부터 확인한다(`which`, `Get-Command`).
- **산출물이 실제로 바뀌었는지 git status / grep으로 확인한다.** "재생성했다"는 말이 아니라 diff/임베드 여부로 검증한다.
- 완료를 주장하기 전 테스트를 돌리고, 실패하면 출력을 그대로 보고한다(`CLAUDE.md` working norm).

## 4. 산출물 경로 모델을 먼저 파악한다

콘텐츠 변경이 **어느 경로를 통해 플레이어에게 도달하는지**를 구현 전에 확인한다.

- 런타임에 Rust가 생성하는 텍스트(예: epilogue `body_blocks`)는 **wasm 바이너리**에 컴파일되어 들어간다. JSON content bundle이 아니다 → 반영하려면 **wasm 재빌드**(`npm run wasm:build`)가 필요하고, `scripts/export_web_data.py`로는 잡히지 않는다.
- YAML encounter/choice/outcome 데이터는 content bundle을 통해 도달한다 → `export_web_data.py`로 재생성한다.
- `web/src/core/wasm-pkg/`는 gitignore이며 CI(`.github/workflows/pages.yml`)가 배포 시 재빌드한다. 로컬 재빌드는 검증용이다.

경로를 모르면 "반영했다"가 거짓이 될 수 있다.

## 5. 테스트 스냅샷 동기화

런타임 출력 문자열을 바꾸면, 그 문자열을 assert하는 테스트가 깨질 수 있다. 이는 in-scope 수정이다.

- 예: epilogue body 교체 시 `crates/escape-core/tests/route_parity.rs`가 옛 placeholder substring을 검사해 실패 → 새 본문의 안정적 substring으로 갱신.
- 단, **구조/플래그/카드 id를 검사하는 assertion은 건드리지 않는다.** 텍스트 스냅샷만 동기화한다.

## 6. 보안/설계 제약은 구현 전에 명시

이구학지 기준 영구 제약(구현 대상에서 영구 제외):

- `told_seoharin_truth`(서하린 진실 전달), `final_cheongirok_identity_revealed`(천기록 기록자 정체 reveal) — 코드베이스에 넣지 않는다.
- 숫자 combat resolver / HP 숫자전 — Notion이 사도전 등에서 금지.
- private/`*.local.*` 콘텐츠, 실제 위치/사내 정보 — 공개 산출물에 넣지 않는다.

기록자 **중간 암시**(실시간 필사 감지/시선/존재감)는 정체 reveal이 아니므로 허용된다. subagent에 위임할 때 이 제약을 프롬프트에 그대로 전달한다.

## 7. 충돌 해결 — 자동 리마인더 vs 직접 지시

자동 hook/리마인더가 일괄 목표("모두 구현")를 재주입하더라도, 사용자가 **이 세션에서 직접 내린 명시적 결정**(예: "보류 유지 — 멈춤 확정")이 우선한다. 되돌리기 어려운 대규모 작업(신규 persistent 엔진/아키텍처)은 직접 지시 없이 시작하지 않는다. 같은 질문을 반복하지 않고, 한 번의 결정으로 충돌을 종결한다.

## 참고

- 모델 tier 위임: `AGENTS.md` "Subagent 모델 tier"
- 빌드/도구 위치, 알려진 실패: `docs/dev/Troubleshooting.md`
- 다음 작업 우선순위: `docs/dev/Development_Plan.md`
