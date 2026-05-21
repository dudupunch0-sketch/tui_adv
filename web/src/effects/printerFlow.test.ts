import { describe, expect, it } from 'vitest';

import { buildPrinterFlowScene, PRETEXT_MODULE_URL, renderPrinterFallbackText } from './printerFlow';


describe('pretext printer anomaly effect contract', () => {
  it('uses the pinned pretext ESM URL', () => {
    expect(PRETEXT_MODULE_URL).toBe('https://esm.sh/@chenglou/pretext@0.0.6');
  });

  it('builds a real corpus around the copier obstacle without private final hints', () => {
    const scene = buildPrinterFlowScene();

    expect(scene.obstacle.kind).toBe('copier');
    expect(scene.corpus).toContain('복합기가 혼자 출력한다');
    expect(scene.corpus).toContain('마지막 문장은 로컬 비공개 파일이 있을 때만 완성된다.');
    expect(scene.corpus).not.toContain('final_hint');
    expect(scene.corpus).not.toContain('secrets.local');
  });

  it('has a readable fallback for browsers that cannot import pretext', () => {
    const fallback = renderPrinterFallbackText(buildPrinterFlowScene());

    expect(fallback).toContain('[ANOMALY CANVAS FALLBACK]');
    expect(fallback).toContain('복합기에 붙은 IP 주소 표의 숫자들을 모두 더한다.');
  });
});
