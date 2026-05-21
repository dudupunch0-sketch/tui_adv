import type { PublicSecret } from '../game/types';

const PRIVATE_SECRET_FIELDS = ['final_hint', 'actual_ip_address', 'office_location', 'treasure_location'];

export function assertPublicSecretsSafe(secrets: readonly unknown[]): void {
  for (const rawSecret of secrets) {
    const secret = rawSecret as Record<string, unknown>;
    const secretId = String(secret.id ?? '<missing>');
    for (const fieldName of PRIVATE_SECRET_FIELDS) {
      if (fieldName in secret) {
        throw new Error(`public secret ${secretId} has private-only field: ${fieldName}`);
      }
    }
  }
}

export function publicSecretSummary(secret: PublicSecret): string {
  return [
    `현실 연결 힌트: ${secret.title}`,
    ...(secret.public_hint_steps ?? []).map((step, index) => `${index + 1}. ${step}`),
    secret.puzzle_prompt ? `퍼즐: ${secret.puzzle_prompt}` : '',
    secret.reward_text ?? '',
  ]
    .filter(Boolean)
    .join('\n');
}
