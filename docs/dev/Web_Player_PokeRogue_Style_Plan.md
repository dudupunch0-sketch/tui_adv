# `tui_adv` Web Player 개발 문서

> 저장소 경로: `docs/dev/Web_Player_PokeRogue_Style_Plan.md`
> 원본 아이디어: `idea_box/web_play_like_pokerogue.md`
> 작성 기준: 2026-05-24, `main` 최신 확인 기준: `7c38217 web_play_like_pokerogue.md 만들기`
> 목적: `escape from the office`를 포켓로그처럼 “주소만 열면 바로 플레이되는 웹 게임”으로 완성·배포하기 위한 개발자용 구현 문서

---

## 1. 결론

`tui_adv`는 이제 “웹 전환을 처음 시작해야 하는 프로젝트”가 아니다. 최신 저장소 기준으로 이미 다음 기반이 있다.

- Rust workspace: `escape-core`, `escape-terminal`, `escape-wasm`
- Web player: `web/` 안의 Vite + TypeScript 앱
- Rust/WASM primary path: `crates/escape-wasm` → `web/src/core/wasm-pkg/` → `web/dist/assets/wasm-pkg/`
- Player build alias: `cd web && npm run build:player`
- Local player preview alias: `cd web && npm run preview:player`
- Web Storybook visual QA: `web/scripts/storybook-reference-qa.mjs`, `npm run qa:storybook:visual`
- 핵심 설계 방향: `Rust GameCore`가 gameplay truth를 소유하고, Web Storybook/GlyphFX는 `ScenePage`를 표시하는 primary UX가 된다.

따라서 앞으로의 핵심 작업은 **게임 로직을 새로 웹으로 옮기는 것**이 아니라 다음을 완성하는 것이다.

1. 정적 웹 배포 파이프라인을 만든다.
2. GitHub Pages 또는 Cloudflare Pages 같은 정적 호스팅에서 WASM asset이 안정적으로 로드되게 한다.
3. 포켓로그처럼 “설치·터미널·로컬 개발환경 없이 URL 접속만으로 플레이”되는 player entry를 고정한다.
4. localStorage 저장, seed, 새 게임, 이어하기, 오류 복구, 모바일 UX를 production 수준으로 다듬는다.
5. Rust GameCore와 Web renderer의 책임 분리를 계속 유지한다.

---

## 2. 목표 제품 컨셉

### 2.1 한 문장 컨셉

**회사에 갇힌 플레이어가 모바일 픽셀 게임북 화면에서 선택지를 고르며 생존·탈출·정복·진실·히든 현실 연결 루트를 탐험하는 브라우저 로그라이크 스토리 게임.**

### 2.2 포켓로그식 웹 플레이 경험에서 가져올 점

포켓로그의 핵심은 “게임 클라이언트가 웹앱으로 배포되어 누구나 브라우저에서 바로 실행한다”는 점이다. `tui_adv`도 같은 방향을 취하되, 장르는 다르므로 화면과 구조는 다음처럼 변형한다.

| 포켓로그식 요소 | `tui_adv`식 해석 |
|---|---|
| URL만 열면 바로 게임 시작 | `web/dist/` 정적 배포, WASM 포함 |
| 계정 없이도 플레이 가능 | MVP는 localStorage 기반 게스트 저장 |
| 반복 플레이 | seed 기반 run, 루트/엔딩/업적 수집 |
| 브라우저 UI | Web Storybook + GlyphFX 모바일 세로형 board |
| 게임 로직 클라이언트 실행 | Rust GameCore를 WASM으로 브라우저에서 실행 |
| 빠른 재시작 | 새 게임, seed 입력, 이어하기 버튼 |

### 2.3 차별화되는 게임 감각

이 프로젝트는 Phaser 액션 게임이 아니라 **TUI 감성의 텍스트/선택지/호러 gamebook**이다. 따라서 “웹 게임처럼 보이게” 하기보다 “터미널 감성과 문서/사내 시스템 공포를 웹에서 읽기 좋게 구현”하는 것이 맞다.

추천 톤:

- 모바일 세로형 픽셀 보드
- 사원증 HUD
- 체력/정신력/배터리/허기/갈증 상태칸
- 위험도 rail
- 오래된 결재서류나 사내공지 같은 본문 배경
- `✥` bullet이 붙은 큰 문장형 선택지
- 복합기, CCTV, 메신저, 회의실 패널 기반 GlyphFX
- “웹 대시보드”가 아니라 “회사 괴담 게임북”

---

## 3. 현재 저장소 상태 요약

### 3.1 런타임 구조

```text
Rust GameCore
├─ crates/escape-core
│  ├─ content bundle loading
│  ├─ GameState / PlayerState
│  ├─ action availability
│  ├─ choice/move/use action application
│  ├─ resource/item/clue/flag/ending/achievement rules
│  └─ ScenePage / ActionResult / EffectCue
│
├─ crates/escape-wasm
│  └─ JSON-string WASM boundary
│     ├─ new_game_json(seed, content_bundle_json)
│     ├─ scene_page_json(state_json, content_bundle_json)
│     └─ apply_action_json(state_json, content_bundle_json, action_id)
│
├─ web/
│  ├─ Vite + TypeScript app
│  ├─ Web Storybook renderer
│  ├─ GlyphFX / visual catalog
│  ├─ generated content bundle import
│  ├─ localStorage save surface
│  └─ visual QA runner
│
└─ crates/escape-terminal
   └─ SuperLightTUI terminal fallback / horror edition
```

### 3.2 이미 존재하는 주요 명령

현재 웹 실행 경로는 다음 명령을 중심으로 보면 된다.

```bash
python scripts/export_web_data.py \
  --check \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json

cd web
npm install
npm test
npm run build
npm run dev -- --host 127.0.0.1 --port 8765
```

Rust/WASM-primary player 경로:

```bash
cd web
npm run wasm:build
npm run build:wasm
npm run preview:wasm
npm run build:player
npm run preview:player
```

최신 main 기준 package script의 의미:

| Script | 의미 |
|---|---|
| `dev` | Vite dev server. WASM package가 없으면 legacy fallback 가능 |
| `dev:wasm` | WASM package 생성 후 Vite dev server |
| `build` | TypeScript check + Vite build |
| `wasm:build` | `crates/escape-wasm`을 `wasm-pack --target web`으로 빌드 |
| `wasm:copy` | generated WASM package를 `web/dist/assets/wasm-pkg/`로 복사 |
| `build:wasm` | WASM build + Vite build + WASM copy |
| `build:player` | player 배포용 alias. 현재는 `build:wasm`과 동일 |
| `preview:wasm` | WASM build 후 Vite preview |
| `preview:player` | player preview alias |
| `qa:storybook:visual` | Playwright Chromium 기반 Web Storybook 구조/레이아웃 QA |

---

## 4. 목표 아키텍처

### 4.1 핵심 원칙

```text
Core owns truth. Renderer owns mood.
```

- 게임 규칙, 선택 가능 여부, 결과, 엔딩, 업적, 자원 변화는 Rust GameCore가 소유한다.
- Web renderer는 `ScenePage`를 받아 DOM/CSS/Canvas/GlyphFX로 표시한다.
- Web renderer는 “이 선택지가 가능한가?”를 다시 계산하지 않는다.
- TypeScript mirror core는 fallback/parity oracle로만 유지하고, 새 gameplay rule을 추가하지 않는다.
- production player는 가능한 한 Rust/WASM path를 기본이자 필수 path로 둔다.

### 4.2 데이터 흐름

```text
src/tui_adv/data/*.yaml
        │
        ▼
scripts/export_web_data.py
        │
        ├─ crates/escape-core/fixtures/content/content.bundle.json
        └─ web/src/data/generated/content.bundle.json
                    │
                    ▼
web/src/core/wasmRuntime.ts
        │
        ▼
escape-wasm JSON boundary
        │
        ▼
escape-core GameState / ScenePage / ActionResult
        │
        ▼
web/src/ui/storybook/render.ts
        │
        ▼
DOM + CSS + Canvas/GlyphFX + localStorage
```

### 4.3 Web runtime 책임

Web runtime은 다음만 담당한다.

- WASM module 로드
- generated content bundle 문자열 전달
- `stateJson` 보관
- `ScenePage` 요청
- 사용자가 누른 `actionId`를 그대로 core에 전달
- localStorage 저장/삭제
- 렌더링 갱신
- WASM 로드 실패 또는 action 오류를 사용자-facing warning으로 표시

Web runtime이 하면 안 되는 일:

- 선택지 조건 재계산
- resource delta 재계산
- 엔딩 판정
- 업적 해금 판정
- private secret 해석
- Rust core와 다른 별도 route truth 생성

---

## 5. 구현 방향

## 5.1 MVP: 정적 Web Player

MVP 목표는 다음 URL이 동작하는 것이다.

```text
https://<owner>.github.io/tui_adv/
```

또는 커스텀 도메인 사용 시:

```text
https://<domain>/
```

MVP에서 필요한 기능:

- 첫 접속 시 새 게임 시작
- localStorage 자동 저장
- 새 게임 버튼
- 이어하기 버튼 또는 자동 이어하기
- 번호키 입력
- 클릭/터치 입력
- 모바일 portrait board 유지
- Rust/WASM GameCore 로드
- WASM 로드 실패 시 명확한 오류 표시
- private field 누출 방지
- QA에서 `assets/wasm-pkg/escape_wasm.js`, `escape_wasm_bg.wasm` 로드 확인

MVP에서 하지 않을 것:

- 계정 시스템
- 서버 저장
- 글로벌 랭킹
- 결제
- 멀티플레이
- Electron/Tauri desktop wrapper
- golden screenshot baseline commit
- 실제 회사/실제 위치/실제 내부망 정보 포함

---

## 5.2 Production player에서 WASM을 사실상 필수로 만들기

현재 Web 앱은 WASM 로드에 실패하면 legacy TypeScript mirror로 임시 실행할 수 있다. 개발 중에는 좋지만, production player에서는 다음 문제가 생길 수 있다.

- Rust GameCore와 legacy mirror가 어긋나도 사용자가 모를 수 있다.
- 배포 문제를 fallback이 숨긴다.
- QA가 WASM resource load를 놓치면 “겉으로는 되는 것처럼 보이는” 상태가 된다.

권장 정책:

| 환경 | WASM 실패 처리 |
|---|---|
| local dev | legacy fallback 허용, warning 표시 |
| PR preview | fallback 허용 가능하나 visual QA는 `--require-wasm` 별도 실행 |
| production player | fallback 대신 “WASM 로드 실패” 오류 패널 표시 권장 |

구현안:

```ts
const REQUIRE_WASM = import.meta.env.VITE_REQUIRE_WASM === 'true';

async function bootstrapWasmRuntime(): Promise<void> {
  try {
    wasmRuntime = await createEscapeWasmRuntime({
      initialStateJson: window.localStorage.getItem(RUST_SAVE_KEY) ?? undefined,
      seed: 123,
    });
    lastError = null;
    render();
  } catch (error) {
    if (REQUIRE_WASM) {
      lastError = `Rust GameCore WASM을 불러오지 못했습니다. 새로고침하거나 배포 asset 경로를 확인하세요: ${errorMessage(error)}`;
      renderFatalPlayerError(lastError);
      return;
    }

    lastError = `Rust GameCore WASM을 불러오지 못해 legacy mirror로 임시 실행 중입니다: ${errorMessage(error)}`;
    render();
  }
}
```

production build에서는 다음 환경변수를 준다.

```bash
VITE_REQUIRE_WASM=true npm run build:player
```

---

## 5.3 WASM asset path 하드닝

현재 구조는 Vite build 후 JS bundle이 `assets/` 아래에 위치하고, `wasm-pkg`도 `assets/wasm-pkg/`로 복사되는 구조다.

권장 개선:

```ts
const DEFAULT_WASM_MODULE_PATH = new URL(
  './wasm-pkg/escape_wasm.js',
  import.meta.url,
).toString();
```

이렇게 하면 dev/build 양쪽에서 “현재 JS module 기준 상대 경로”로 WASM JS glue 파일을 찾는다.

예상 해석:

| 환경 | module 위치 | WASM import 해석 |
|---|---|---|
| Vite dev | `/src/core/wasmRuntime.ts` | `/src/core/wasm-pkg/escape_wasm.js` |
| Vite build | `/assets/index-*.js` | `/assets/wasm-pkg/escape_wasm.js` |
| GitHub Pages project site | `/tui_adv/assets/index-*.js` | `/tui_adv/assets/wasm-pkg/escape_wasm.js` |

---

## 5.4 Vite base path 설정

GitHub Pages project site는 보통 루트가 아니라 repository name 아래에 배포된다.

```text
https://<owner>.github.io/tui_adv/
```

따라서 Vite `base`를 환경변수로 제어하는 것이 안전하다.

`web/vite.config.ts` 권장 변경:

```ts
import { defineConfig } from 'vite';

export default defineConfig({
  base: process.env.VITE_BASE_PATH ?? '/',
  build: {
    outDir: 'dist',
    sourcemap: true,
  },
  test: {
    environment: 'node',
  },
});
```

GitHub Pages project site build:

```bash
VITE_BASE_PATH=/tui_adv/ VITE_REQUIRE_WASM=true npm run build:player
```

커스텀 도메인 root build:

```bash
VITE_BASE_PATH=/ VITE_REQUIRE_WASM=true npm run build:player
```

---

## 6. GitHub Pages 배포 계획

### 6.1 왜 GitHub Pages인가

MVP에는 GitHub Pages가 충분하다.

- 정적 파일 배포에 적합하다.
- `web/dist/` artifact를 그대로 올릴 수 있다.
- public repo라면 접근 장벽이 낮다.
- 서버를 직접 운영하지 않아도 된다.
- 포켓로그식 “그냥 URL로 플레이”에 맞다.

단, GitHub Pages는 서버 로직을 돌리는 곳이 아니다. 나중에 계정, 랭킹, 서버 저장을 붙이려면 별도 backend가 필요하다.

### 6.2 권장 workflow

새 파일:

```text
.github/workflows/pages.yml
```

예시:

```yaml
name: Deploy Web Player

on:
  push:
    branches: [main]
    paths:
      - "web/**"
      - "crates/**"
      - "src/tui_adv/data/**"
      - "scripts/export_web_data.py"
      - "Cargo.toml"
      - "Cargo.lock"
      - ".github/workflows/pages.yml"
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: "3.12"

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: "22"
          cache: npm
          cache-dependency-path: web/package-lock.json

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install wasm-pack
        run: cargo install wasm-pack --locked

      - name: Check generated public content bundle
        run: |
          python scripts/export_web_data.py \
            --check \
            --bundle crates/escape-core/fixtures/content/content.bundle.json \
            --bundle web/src/data/generated/content.bundle.json

      - name: Test Rust workspace
        run: cargo test --workspace

      - name: Install web dependencies
        working-directory: web
        run: npm ci

      - name: Run web tests
        working-directory: web
        run: npm test

      - name: Build web player
        working-directory: web
        env:
          VITE_BASE_PATH: /tui_adv/
          VITE_REQUIRE_WASM: "true"
        run: npm run build:player

      - name: Install Playwright Chromium
        working-directory: web
        env:
          PLAYWRIGHT_BROWSERS_PATH: /tmp/tui-adv/ms-playwright
        run: npx playwright install chromium

      - name: Preview player for visual QA
        working-directory: web
        run: |
          npx vite preview --host 127.0.0.1 --port 4173 --base /tui_adv/ > /tmp/tui-adv-vite-preview.log 2>&1 &
          for i in $(seq 1 30); do
            if curl -fsS http://127.0.0.1:4173/tui_adv/ >/dev/null; then
              exit 0
            fi
            sleep 1
          done
          cat /tmp/tui-adv-vite-preview.log
          exit 1

      - name: Run Storybook visual QA
        working-directory: web
        env:
          PLAYWRIGHT_BROWSERS_PATH: /tmp/tui-adv/ms-playwright
        run: |
          npm run qa:storybook:visual -- \
            --base-url http://127.0.0.1:4173/tui_adv/ \
            --out-dir /tmp/tui-adv/storybook-visual-qa \
            --require-wasm

      - name: Configure Pages
        uses: actions/configure-pages@v5

      - name: Upload Pages artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: web/dist

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}

    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

주의:

- repository URL이 `/tui_adv/`가 아니라 커스텀 도메인 root라면 `VITE_BASE_PATH`를 `/`로 바꾼다.
- workflow 최초 추가 후 GitHub repository Settings → Pages → Source를 GitHub Actions로 설정한다.
- Playwright QA는 시간이 걸릴 수 있으므로, 처음에는 PR check로만 두고 Pages deploy job에서는 optional로 둘 수도 있다.

---

## 7. Cloudflare Pages / Vercel / Netlify 대안

GitHub Pages로 충분하지만, 다음 요구가 생기면 다른 정적 호스팅이 더 편할 수 있다.

| 요구 | 추천 |
|---|---|
| 커스텀 header, cache control, preview URL 강화 | Cloudflare Pages |
| 프론트엔드 preview workflow | Vercel / Netlify |
| edge function 또는 serverless API | Cloudflare Pages + Workers, Vercel Functions, Netlify Functions |
| 향후 계정/랭킹/서버 저장 | 별도 backend 또는 BaaS 필요 |

MVP는 GitHub Pages로 시작하고, 다음 조건이 생기면 이전한다.

- WASM MIME/cache policy를 세밀하게 제어해야 한다.
- 매 PR마다 shareable preview URL이 필요하다.
- daily run, 랭킹, 계정 저장 같은 server API가 필요하다.

---

## 8. Player UX 설계

### 8.1 시작 화면

현재는 seed `123` 중심으로 바로 시작되는 흐름이다. 포켓로그식 공개 player로 가려면 시작 화면이 필요하다.

권장 시작 화면:

```text
ESCAPE FROM THE OFFICE

[이어하기]
[새 게임]
[시드 입력]
[오늘의 격리 run]
[설정]
```

MVP 우선순위:

1. 이어하기
2. 새 게임
3. seed 표시
4. seed 복사
5. seed 입력

후순위:

- 오늘의 seed
- 난이도 preset
- 접근성 설정
- reduce motion toggle
- 저장 export/import

### 8.2 저장 UX

현재 Web은 localStorage를 사용한다. production player에서는 저장 구조를 명확히 보여줘야 한다.

권장 localStorage key:

```text
escape-office.rust.save.v1
escape-office.settings.v1
escape-office.last-run-summary.v1
```

저장 UI:

- 자동 저장됨 표시
- 저장 시간 표시
- 새 게임 전 확인 modal
- 저장 초기화 버튼
- JSON export/import 버튼

SaveEnvelope 정책:

```json
{
  "schema_version": 1,
  "state": {
    "seed": 123,
    "turn": 1,
    "location_id": "dev_desk"
  }
}
```

규칙:

- Web-only rule state를 만들지 않는다.
- Rust core save JSON을 기준으로 한다.
- schema version mismatch는 traceback이 아니라 사용자 메시지로 처리한다.
- private secret은 save/export에 포함하지 않는다.

### 8.3 입력 UX

지원 입력:

- 마우스 클릭
- 터치 탭
- 숫자키 `1`~`9`
- 새 게임 단축키는 production에서는 신중하게 둔다.
- 모바일에서는 큰 row button을 유지한다.

선택지 표시는 renderer-local 번호를 붙이되, 실행은 항상 `SceneAction.id`로 한다.

```html
<button class="choice-row" data-action-id="choice:check_message" data-action-kind="choice">
  <span class="choice-bullet">✥</span>
  <span class="choice-index">1</span>
  <span class="choice-label">메시지를 확인한다</span>
</button>
```

### 8.4 모바일 화면 기준

유지해야 할 viewport set:

```text
390 x 844
414 x 896
800 x 1440
810 x 1644
wide desktop
```

wide desktop에서도 dashboard로 바꾸지 않는다. 가운데 portrait board를 유지한다.

---

## 9. Visual/GlyphFX 방향

### 9.1 목표

GlyphFX는 “멋있는 효과”가 아니라 “공포와 단서 전달”을 위한 presentation layer다.

규칙:

- stable terms는 항상 읽을 수 있어야 한다.
- no-canvas/reduced-motion 환경에서도 fallback text를 보여준다.
- core는 `EffectCue`만 제공한다.
- renderer가 title/body를 보고 임의로 효과를 추측하지 않는다.

### 9.2 Visual catalog

`ScenePage.visual.id`는 file path가 아니라 semantic id다.

예:

| visual id | Web 해석 | Terminal 해석 |
|---|---|---|
| `opening_messenger` | 메신저/document panel | small message frame |
| `printer_anomaly` | Canvas/GlyphFX 복합기 출력 | ASCII 복합기 card |
| `office_corridor_static` | 정적 복도 vignette | Unicode corridor card |
| `location:<id>` | 위치 fallback visual | 위치 fallback card |
| `ending:<id>` | 엔딩 visual panel | 엔딩 terminal card |

Unknown visual id 정책:

- 빈 화면 금지
- action drop 금지
- `visual.alt` 또는 page title 기반 safe placeholder 표시

### 9.3 레퍼런스 이미지 정책

`idea_box/플레이화면*.bmp`는 UI grammar reference로만 사용한다.

- 그대로 production asset으로 import하지 않는다.
- 출처/라이선스 확인 전 배포 asset으로 쓰지 않는다.
- 필요한 경우 내부 스타일 문법만 CSS/DOM으로 재구현한다.

---

## 10. 콘텐츠와 보안

### 10.1 공개 bundle 원칙

배포되는 파일에는 다음이 절대 들어가면 안 된다.

```text
final_hint
actual_ip_address
office_location
treasure_location
실제 회사명
실제 내부망 주소
실제 좌석/층/방 번호
실제 사람 이름
private final hint text
```

검증 명령:

```bash
python scripts/export_web_data.py \
  --check \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json

python -m pytest tests/test_content_data.py tests/test_secrets.py tests/test_web_data_export.py -q
```

### 10.2 현실 연결 히든 루트

현실 연결 요소는 게임의 독특한 매력이지만, 공개 웹 배포에서는 안전한 placeholder만 허용한다.

권장 정책:

- 공개 web player: public-safe puzzle, placeholder IP, reward text까지만 표시
- 로컬 private build: `private/secrets.local.yaml` 기반 final hint 가능
- production web: local private secret loader 비활성 또는 build-time guard 필수

---

## 11. QA / 검증 매트릭스

### 11.1 로컬 검증

```bash
# repo root
python scripts/export_web_data.py \
  --check \
  --bundle crates/escape-core/fixtures/content/content.bundle.json \
  --bundle web/src/data/generated/content.bundle.json

python -m pytest tests -q
cargo fmt --check
cargo test --workspace

cd web
npm ci
npm test
npm run build:player
```

### 11.2 WASM player preview 검증

터미널 A:

```bash
cd web
npm run build:player
npx vite preview --host 127.0.0.1 --port 4173 --base /tui_adv/
```

터미널 B:

```bash
cd web
export PLAYWRIGHT_BROWSERS_PATH=/tmp/tui-adv/ms-playwright
npm run qa:storybook:visual -- \
  --base-url http://127.0.0.1:4173/tui_adv/ \
  --out-dir /tmp/tui-adv/storybook-visual-qa \
  --require-wasm
```

### 11.3 Acceptance checklist

배포 전 반드시 확인한다.

- [ ] player URL이 브라우저에서 바로 열린다.
- [ ] terminal, Python, Rust toolchain 없이 플레이 가능하다.
- [ ] `assets/wasm-pkg/escape_wasm.js`가 200으로 로드된다.
- [ ] `assets/wasm-pkg/escape_wasm_bg.wasm`이 200으로 로드된다.
- [ ] `.storybook-runtime-warning`이 production player에 보이지 않는다.
- [ ] 첫 선택지를 클릭하면 page fingerprint가 바뀐다.
- [ ] 숫자키 `1`로 선택지가 실행된다.
- [ ] localStorage 저장 후 새로고침해도 이어진다.
- [ ] 새 게임으로 저장을 초기화할 수 있다.
- [ ] 390px mobile width에서 horizontal overflow가 없다.
- [ ] desktop에서도 centered portrait board가 유지된다.
- [ ] 공개 bundle에 private-only field가 없다.
- [ ] unknown visual id가 action을 막지 않는다.
- [ ] reduced-motion/no-canvas에서도 stable clue text가 읽힌다.

---

## 12. 추천 PR 단위

### PR 1 — Web player deployment readiness

Status: implemented.

목표: GitHub Pages에 올릴 수 있는 production player build를 고정한다.

변경 후보:

- `web/vite.config.ts`
  - `base: process.env.VITE_BASE_PATH ?? '/'`
- `web/src/core/wasmRuntime.ts`
  - `new URL('./wasm-pkg/escape_wasm.js', import.meta.url)` 방식으로 path 명확화
- `web/src/main.ts`
  - `VITE_REQUIRE_WASM` production policy 추가
- `.github/workflows/pages.yml`
  - `npm run build:player`
  - visual QA `--require-wasm`
  - Pages artifact upload/deploy
- `docs/dev/Web_Player_PokeRogue_Style_Plan.md`
  - 이 문서 추가

검증:

```bash
cargo test --workspace
python -m pytest tests -q
cd web
npm ci
npm test
npm run build:player
npm run qa:storybook:visual -- --base-url http://127.0.0.1:4173/ --out-dir /tmp/tui-adv/storybook-visual-qa --require-wasm
```

### PR 2 — Player start/save UX

목표: 공개 플레이어로서 최소한의 시작/이어하기 UX를 제공한다.

변경 후보:

- start screen 추가
- continue/new game/seed input
- save timestamp
- save reset confirmation
- save import/export JSON
- localStorage schema version message

비목표:

- 서버 저장
- 계정
- 랭킹

### PR 3 — Production visual polish

목표: Web Storybook을 “대시보드”가 아니라 “게임 화면”으로 더 고정한다.

변경 후보:

- HUD polish
- bottom dock 기능 연결
- visual catalog 확장
- GlyphFX reduced-motion fallback 강화
- ending 화면 polish
- route summary screen

비목표:

- 새 gameplay rule
- ScenePage schema 확장

ScenePage schema 확장이 필요하면 먼저 `docs/dev/Data_Schema.md`를 수정하는 별도 design slice로 처리한다.

### PR 4 — Offline/PWA optional

목표: 한 번 연 뒤 오프라인에서도 시작 화면과 기존 save를 볼 수 있게 한다.

변경 후보:

- `manifest.webmanifest`
- service worker
- app icon
- install prompt 안내
- cache versioning
- WASM/font/static asset precache

주의:

- service worker는 cache invalidation 버그를 만들기 쉽다.
- MVP 배포가 안정된 뒤 진행한다.

### PR 5 — Optional backend

목표: 포켓로그식 retention 기능을 추가한다.

후보:

- daily seed
- run summary share
- anonymous leaderboard
- cloud save
- issue/feedback button

주의:

- 서버가 붙는 순간 개인정보, abuse, rate limit, 비용, 보안 정책이 생긴다.
- MVP에서는 backend 없이 간다.

---

## 13. 운영 정책

### 13.1 배포 채널

권장 채널:

| 채널 | URL | 용도 |
|---|---|---|
| local dev | `http://127.0.0.1:8765` | 개발 |
| local preview | `http://127.0.0.1:4173` | production build smoke |
| GitHub Pages | `https://<owner>.github.io/tui_adv/` | public player |
| custom domain | `https://<domain>/` | 공개 공유용 |

### 13.2 버전 표기

게임 화면 footer 또는 설정에 다음을 보여준다.

```text
build: <short sha>
core: schema_version 1
content: manifest schema_version 1
save: v1
```

Vite build 시 환경변수로 주입 가능하다.

```bash
VITE_BUILD_SHA=$(git rev-parse --short HEAD) npm run build:player
```

### 13.3 오류 메시지 정책

사용자에게 traceback을 보여주지 않는다.

예:

```text
게임 코어를 불러오지 못했습니다.
새로고침 후에도 계속되면 배포된 WASM 파일 경로를 확인해주세요.
```

개발자용 detail은 console에 남긴다.

---

## 14. 주요 리스크와 대응

| 리스크 | 증상 | 대응 |
|---|---|---|
| WASM path 깨짐 | production에서 legacy fallback warning | `new URL(..., import.meta.url)`, `--require-wasm` QA |
| Vite base path 오류 | GitHub Pages에서 JS/CSS 404 | `VITE_BASE_PATH=/tui_adv/` |
| silent legacy divergence | Web과 terminal 결과 다름 | production `VITE_REQUIRE_WASM=true` |
| private data leak | 실제 힌트/위치가 public bundle에 포함 | exporter guard + tests + grep |
| localStorage schema mismatch | 업데이트 후 save load 실패 | SaveEnvelope versioning + migration/error panel |
| mobile overflow | 작은 화면에서 가로 스크롤 | Playwright viewport QA |
| Korean font 깨짐 | tofu box 표시 | bundled Korean font 유지 |
| service worker stale cache | 업데이트해도 옛 코드 실행 | PWA는 후순위, cache version 엄격 관리 |
| asset copyright | reference image 무단 사용 | idea_box 이미지는 grammar reference로만 사용 |

---

## 15. 개발자가 바로 할 일

현재 최신 main 기준으로는 다음 순서가 가장 실용적이다.

1. `web/vite.config.ts`에 `VITE_BASE_PATH` 기반 `base` 설정을 추가한다.
2. `web/src/core/wasmRuntime.ts`의 WASM module path를 `new URL(..., import.meta.url)`로 명확히 한다.
3. `VITE_REQUIRE_WASM=true`일 때 production fallback을 막는다.
4. `.github/workflows/pages.yml`을 추가한다.
5. GitHub repository Settings → Pages → Source를 GitHub Actions로 설정한다.
6. `main` push 후 Pages URL에서 다음을 확인한다.
   - 화면이 열린다.
   - WASM files가 로드된다.
   - runtime warning이 없다.
   - 클릭/번호키 선택이 동작한다.
   - 새로고침 후 save가 유지된다.
7. 이후 start/save UX PR로 넘어간다.

---

## 16. 최종 목표 상태

최종적으로 이 프로젝트는 다음 상태가 되어야 한다.

```text
사용자:
  브라우저로 URL 접속
  → 모바일 게임북 화면 표시
  → 새 게임 또는 이어하기
  → 선택지 클릭/번호키 입력
  → localStorage 자동 저장
  → seed/run/ending 공유

개발자:
  YAML content 작성
  → export/check
  → Rust GameCore test
  → Web Storybook render test
  → WASM player build
  → visual QA
  → GitHub Pages deploy

아키텍처:
  Rust GameCore = gameplay truth
  Web Storybook/GlyphFX = primary UX
  SuperLightTUI = terminal fallback/horror edition
  TypeScript mirror = legacy fallback/parity oracle only
```

이 방향이면 `tui_adv`는 포켓로그처럼 “누구나 웹에서 바로 플레이 가능한 게임”이 되면서도, 기존 프로젝트의 강점인 Rust GameCore, TUI 감성, 회사 괴담/코스믹 호러, 현실 연결 히든 루트 구조를 잃지 않는다.

---

## 17. 참고 링크

- GitHub repository: `https://github.com/dudupunch0-sketch/tui_adv`
- Latest checked commit: `https://github.com/dudupunch0-sketch/tui_adv/commit/7c38217c15cd12dedbc5b6b6c5c4a8555ce882a7`
- GitHub Pages docs: `https://docs.github.com/en/pages`
- Vite static deployment docs: `https://vite.dev/guide/static-deploy.html`
- wasm-pack build docs: `https://rustwasm.github.io/docs/wasm-pack/commands/build.html`
