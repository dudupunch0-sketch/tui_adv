import { describe, expect, it } from 'vitest';

import secrets from '../data/generated/secrets.example.json';
import { assertPublicSecretsSafe, publicSecretSummary } from './publicSecretGuard';


describe('browser public secret guard', () => {
  it('accepts generated public secret examples and formats only public hints', () => {
    assertPublicSecretsSafe(secrets);

    const summary = publicSecretSummary(secrets[0]);

    expect(summary).toContain('첫 번째 현실 연결 힌트');
    expect(summary).toContain('마지막 문장은 로컬 비공개 파일이 있을 때만 완성된다.');
    expect(summary).not.toContain('final_hint');
    expect(summary).not.toContain('secrets.local');
  });

  it('rejects private-only fields in browser-visible secret data', () => {
    expect(() =>
      assertPublicSecretsSafe([
        {
          id: 'unsafe',
          title: 'unsafe',
          public_hint_steps: [],
          final_hint: 'do not ship',
        },
      ]),
    ).toThrow('public secret unsafe has private-only field: final_hint');
  });
});
