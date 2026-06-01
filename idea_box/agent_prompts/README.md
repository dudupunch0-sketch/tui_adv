---
type: agent_prompt_index
created: 2026-06-01
source: assistant
---

# agent_prompts

이 폴더는 긴 copy-paste 프롬프트를 Git-tracked 파일로 보관하기 위한 공간이다. 새 Hermes/agent 세션에는 긴 본문을 복사하지 말고, 아래 짧은 지시만 주면 된다.

```text
cd /home/dudupunch0/tui_adv
이 repo의 idea_box/agent_prompts/<prompt-file>.md를 읽고 그대로 수행해. repo canonical docs와 이 prompt가 충돌하면 repo canonical docs를 우선하고, 충돌 사실을 보고해.
```

## 현재 prompt 파일

| 파일 | 용도 | 변경 범위 |
|---|---|---|
| `wuxia_seo_harin_rescue_readiness.md` | 다음 구현 slice인 `wuxia_seo_harin_rescue`의 구현 직전 readiness 검수 / 최종 handoff 확인 | docs-only, 필요할 때만 최소 문서 수정 |
| `wuxia_seo_harin_rescue_implementation.md` | `wuxia_seo_harin_rescue` preview runtime slice 구현 | preview source/generated/test만, default office bundle 보호 |
| `seo_harin_future_design.md` | 서하린 companion/emotional-axis 및 후일담 future design 정리 | read-only 또는 docs-only, runtime 구현 금지 |

## 운영 규칙

- 이 폴더의 prompt는 실행 편의를 위한 handoff다. 최종 source of truth는 `docs/dev/Development_Plan.md`, `docs/dev/Notion_Design_Coverage.md`, content/design docs, runtime source다.
- prompt가 오래되어 canonical docs와 충돌하면 prompt를 따르지 말고 canonical docs 기준으로 판단한 뒤, prompt 갱신이 필요하다고 보고한다.
- `wuxia_seo_harin_rescue_implementation.md`만 runtime 구현을 허용한다. 나머지 prompt는 runtime YAML/Rust/Web/generated bundle을 직접 수정하지 않는다.
- Git으로 공유하려면 이 폴더 변경을 commit/push해야 한다. 같은 로컬 worktree의 다른 세션은 commit 전에도 파일을 읽을 수 있지만, 다른 checkout/서버는 push된 branch 또는 main에 반영된 뒤에만 볼 수 있다.
