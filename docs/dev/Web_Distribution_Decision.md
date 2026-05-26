# Web/Tauri/Electron 배포 표면 결정

## 결정

현재 배포 표면: Web-only.

Tauri/Electron: deferred.

Rust/WASM-primary Web build를 플레이어용 기본 배포물로 둔다. 지금 단계에서는 Tauri/Electron 데스크톱 앱을 열지 않는다.

```text
Rust GameCore
  -> escape-wasm JSON boundary
  -> Web Storybook + GlyphFX renderer
  -> static web artifact: web/dist/
```

## 실행 표면

| 표면 | 상태 | 기준 |
|---|---|---|
| Web static build | primary | `npm run build:player`가 `npm run build:wasm`을 실행해 `web/dist/`를 만든다. |
| Web local preview | primary smoke | `npm run preview:player`가 Rust/WASM-primary build 후 Vite preview를 실행한다. |
| SuperLightTUI terminal | fallback / horror edition | 별도 terminal-native renderer로 유지한다. 데스크톱 패키징 대체물이 아니다. |
| Tauri | deferred | 파일 시스템/업데이트/네이티브 창 요구가 생기기 전까지 추가하지 않는다. |
| Electron | deferred | 배포 크기와 별도 desktop runtime 비용 때문에 현재 단계에서는 추가하지 않는다. |

## 이유

1. 현재 제품의 플레이어 UX 후보는 Web Storybook + GlyphFX다.
2. `escape-wasm`과 generated content bundle이 이미 Rust GameCore truth를 Web에 제공한다.
3. Tauri/Electron을 지금 추가하면 core/renderer 검증보다 installer, auto-update, OS permission, bundle size 관리가 먼저 커진다.
4. 개인 서버/WSL/Cloud Shell 환경에서는 Web static artifact가 가장 재현 가능한 배포 단위다.
5. Desktop wrapper는 Web primary UX가 충분히 안정된 뒤에도 별도 사용자 가치가 있어야 한다.

## 패키징 계약

- Web player build alias: `npm run build:player`
- Web player preview alias: `npm run preview:player`
- Rust/WASM primary build path는 계속 `npm run build:wasm`이다.
- 생성물 `web/src/core/wasm-pkg/`는 로컬 artifact이며 커밋하지 않는다. `npm run build:wasm`은 이 package를 정적 배포물의 `web/dist/assets/wasm-pkg/`로 복사한다.
- 배포 대상 정적 산출물은 `web/dist/`다.
- package manifest에 Tauri/Electron dependency나 script를 추가하지 않는다.

## Tauri/Electron을 다시 열 조건

다음 조건 중 하나 이상이 실제 요구가 될 때 별도 slice로 재검토한다.

- 오프라인 저장/파일 import-export가 브라우저 localStorage를 넘어선다.
- 로컬 secret 파일을 안전하게 선택/암호화/격리하는 native file dialog가 필요하다.
- OS-level 알림, 창 모드, 전체화면 kiosk, 자동 업데이트가 핵심 UX가 된다.
- Web 배포에서 브라우저 보안 정책 때문에 Web Storybook/GlyphFX primary UX가 막힌다.

그 전까지는 Web-only가 결정된 배포 표면이다. `escape-terminal`의 terminal full-screen app loop와 tick/raw-draw GlyphFX baseline도 완료되었으므로, 이 문서는 배포 표면 결정만 고정한다. 현재 구현 우선순위는 `docs/dev/Development_Plan.md`를 따른다.
