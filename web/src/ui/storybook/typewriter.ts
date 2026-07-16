const TICK_MS = 16;
const TARGET_TOTAL_MS = 3600;
const MIN_CHARS_PER_TICK = 1;

export interface TypewriterHandle {
  finish(): void;
  cancel(): void;
}

const NOOP_HANDLE: TypewriterHandle = { finish() {}, cancel() {} };

interface TypingNode {
  node: Text;
  fullText: string;
}

/**
 * 본문 텍스트를 타자기식으로 점진 출력한다. 화면 탭(클릭)으로 즉시 완성.
 * 판정 배너(.check-resolution)는 자체 연출이 있으므로 제외한다.
 * enabled=false(reduced motion 등)면 아무것도 하지 않고 선택지만 즉시 노출한다.
 */
export function startTypewriter(shell: HTMLElement, options: { enabled: boolean }): TypewriterHandle {
  const choices = Array.from(shell.querySelectorAll<HTMLElement>('.storybook-choices'));
  const revealChoices = (animated: boolean) => {
    for (const choiceNav of choices) {
      choiceNav.setAttribute('data-reveal', animated ? 'done' : 'settled');
    }
  };

  const body = shell.querySelector<HTMLElement>('.storybook-body')
    ?? shell.querySelector<HTMLElement>('.story-flow--ordered');
  if (!options.enabled || !body) {
    revealChoices(false);
    return NOOP_HANDLE;
  }

  const nodes = collectTextNodes(body);
  const totalChars = nodes.reduce((sum, entry) => sum + entry.fullText.length, 0);
  if (!totalChars) {
    revealChoices(false);
    return NOOP_HANDLE;
  }

  for (const choiceNav of choices) choiceNav.setAttribute('data-reveal', 'pending');
  for (const entry of nodes) entry.node.textContent = '';

  const caret = document.createElement('span');
  caret.className = 'type-caret';
  caret.setAttribute('aria-hidden', 'true');

  const charsPerTick = Math.max(MIN_CHARS_PER_TICK, Math.ceil(totalChars / (TARGET_TOTAL_MS / TICK_MS)));
  let nodeIndex = 0;
  let charIndex = 0;
  let timer = 0;
  let finished = false;

  const viewport = shell.querySelector<HTMLElement>('.game-viewport');

  const detach = () => {
    window.clearInterval(timer);
    caret.remove();
    viewport?.removeEventListener('pointerdown', onSkip, true);
  };

  const finish = () => {
    if (finished) return;
    finished = true;
    detach();
    for (const entry of nodes) entry.node.textContent = entry.fullText;
    revealChoices(true);
  };

  const cancel = () => {
    if (finished) return;
    finished = true;
    detach();
  };

  function onSkip(event: Event) {
    // 진행 중일 때의 탭은 "본문 완성"으로만 소비하고 아래 요소로 전달하지 않는다.
    event.preventDefault();
    event.stopPropagation();
    finish();
  }

  viewport?.addEventListener('pointerdown', onSkip, true);

  timer = window.setInterval(() => {
    let budget = charsPerTick;
    while (budget > 0 && nodeIndex < nodes.length) {
      const entry = nodes[nodeIndex];
      const remaining = entry.fullText.length - charIndex;
      const take = Math.min(budget, remaining);
      charIndex += take;
      budget -= take;
      entry.node.textContent = entry.fullText.slice(0, charIndex);
      entry.node.parentElement?.appendChild(caret);
      if (charIndex >= entry.fullText.length) {
        nodeIndex += 1;
        charIndex = 0;
      }
    }
    if (nodeIndex >= nodes.length) finish();
  }, TICK_MS);

  return { finish, cancel };
}

function collectTextNodes(body: HTMLElement): TypingNode[] {
  const walker = document.createTreeWalker(body, NodeFilter.SHOW_TEXT, {
    acceptNode(node) {
      const text = node.textContent ?? '';
      if (!text.trim()) return NodeFilter.FILTER_REJECT;
      const parent = node.parentElement;
      if (!parent) return NodeFilter.FILTER_REJECT;
      if (parent.closest('.check-resolution, .storybook-choices, .story-illustration')) {
        return NodeFilter.FILTER_REJECT;
      }
      return NodeFilter.FILTER_ACCEPT;
    },
  });
  const nodes: TypingNode[] = [];
  let current = walker.nextNode();
  while (current) {
    nodes.push({ node: current as Text, fullText: current.textContent ?? '' });
    current = walker.nextNode();
  }
  return nodes;
}
