# Browser Fake-TUI + Pretext Slice — Development History

> 보관용 문서입니다. 이 문서는 이미 처리된 과거 계획을 정리한 개발 이력이며, 현재 실행 계획이나 제품 방향이 아닙니다.
>
> 정리일: 2026-05-22

## 상태

- 원래 계획: `.hermes/plans/2026-05-21_123855-fake-tui-pretext-browser-plan.md`
- 현재 상태: 과거 브라우저 fake-TUI 수직 슬라이스 계획의 보존본
- 현재 제품 방향: Rust GameCore + dual renderer
  - terminal: SuperLightTUI 기반 terminal-native GlyphFX 후보
  - web: Storybook + GlyphFX primary UX 후보
- 폐기된 방향: 브라우저 fake-TUI dashboard를 장기 제품 UI로 키우는 방향

## 왜 보관하는가

이 계획은 Python/Textual 기반 회사 아포칼립스 게임을 브라우저에서 검증하기 위한 첫 전환안이었다. 이후 실제 구현을 통해 다음 가치가 확인됐다.

- YAML 콘텐츠를 웹 런타임용 데이터로 내보낼 수 있다.
- public/private secret boundary를 웹 빌드에서도 지켜야 한다.
- 복합기/현실 연결 장면처럼 특정 사건에만 글자 흐름/왜곡 효과를 넣는 접근은 유효하다.
- save/load, route parity, 대표 루트 검증 같은 브라우저 검증 포인트가 필요하다.

다만 이후 방향 정리에서 “패널형 fake-TUI dashboard 자체”는 장기 제품 UI에서 제외됐다. 이 보존본은 구현 당시의 배움과 안전 기준만 남긴다.

## 삭제한 과거 계획 요소

아래 항목은 더 이상 활성 계획으로 보존하지 않는다.

- `web/` Vite fake-TUI dashboard를 primary product surface로 확장하는 지시
- DOM/CSS 패널 shell을 장기 메인 UI로 삼는 설계
- Canvas/pretext anomaly panel을 별도 2번 제품 방향으로 유지하는 비교 축
- 전체 콘텐츠를 TypeScript mirror core에 계속 포팅하는 방향
- 오래된 브랜치/PR/명령 전제
- “첫 브라우저 수직 슬라이스를 지금 구현하라”는 단계별 작업표

## 보존하는 역사적 결정

### 1. 콘텐츠 원본은 YAML

사람이 쓰는 원본 콘텐츠는 YAML로 유지하고, 런타임별 데이터는 생성물로 둔다는 결정은 이후 Rust GameCore 전환에도 유효하다.

```text
src/tui_adv/data/*.yaml
        |
        v
generated runtime content bundle
        |
        +--> legacy Python/Textual
        +--> legacy TypeScript/browser
        +--> future Rust GameCore / WASM
```

### 2. 현실 연결 secret boundary

공개 빌드에는 실제 회사 사무실 위치, 최종 보물/메모 위치, private-only 힌트를 넣지 않는다. 공개 데이터에는 공개 힌트, placeholder, reward metadata만 둔다.

보존할 원칙:

- `private/` 또는 `*.local.*` 데이터는 공개 번들에 포함하지 않는다.
- `final_hint` 류의 실제 위치 정보는 local-only로 유지한다.
- 현실 연결 힌트는 단계적으로 드러나되, 최종 좌표는 개발자가 별도 설치한 로컬 메모/보물과 연결한다.
- build/export 검증에서 private string leakage를 검사한다.

### 3. 효과는 게임 규칙이 아니라 presentation cue

복합기 출력물, 백룸식 공간 왜곡, 저정신력 글자 흔들림 같은 연출은 core rule과 분리한다. 현재 방향에서는 이 원칙을 `EffectCue`로 일반화한다.

예:

```json
{
  "kind": "glyph_anomaly",
  "source": "copier_output",
  "stable_terms": ["비상계단", "토너", "접힌 방향"],
  "distortion": "reflow_then_stabilize"
}
```

Renderer는 cue를 받아 자기 환경에 맞게 표현한다.

- terminal renderer: cell grid, 색상 떨림, glyph corruption, terminal reflow
- web renderer: Storybook page, DOM/Canvas GlyphFX, 모바일 세로 화면 중심

### 4. 검증에서 배운 것

보존할 검증 관점:

- 콘텐츠 export/check
- route parity smoke
- save/load schema roundtrip
- public secret leakage search
- reduced-motion/fallback path
- browser build 또는 terminal snapshot smoke

## 현재 방향과의 관계

이 문서는 “브라우저 fake-TUI dashboard를 계속 만든다”는 의미가 아니다. 현재 방향에서는 다음처럼 흡수된다.

- fake-TUI dashboard UI: 장기 제품 방향에서 제외
- Canvas/pretext 효과 아이디어: Web Storybook GlyphFX와 terminal-native GlyphFX의 재료로 흡수
- TypeScript mirror core: Rust GameCore 전환 전까지 legacy/parity oracle
- YAML export와 secret safety: 새 content bundle/export 설계로 계승

## 후속 문서에서 다룰 대상

활성 설계는 별도 문서에서 다룬다.

- `docs/dev/Rust_Core_Dual_Renderer_Architecture.md` 예정
- `docs/dev/Development_Plan.md` 갱신 예정
- `docs/design/TUI_Storybook_GlyphFX_Concept.md` 갱신 예정

이 history 문서는 의사결정 배경을 설명하기 위한 보관본으로만 사용한다.
