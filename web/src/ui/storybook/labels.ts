const achievementLabels: Record<string, string> = {
  wuxia_first_arrival: '강호 출근',
  wuxia_first_fragment_seen: '천기록 첫 편린',
};

const inventoryItemLabels: Record<string, string> = {
  commuter_badge: '사원증',
  cheonggi_record_notebook: '천기록이 깃든 업무수첩',
  work_chore_token: '청류문 잡일패',
  rejected_aid_letter_fragment: '거절당한 도움 요청 서찰 조각',
};

export function achievementLabel(id: string): string {
  return achievementLabels[id] ?? fallbackLabel(id);
}

export function inventoryItemLabel(id: string): string {
  return inventoryItemLabels[id] ?? fallbackLabel(id);
}

export function hasAchievementLabel(id: string): boolean {
  return id in achievementLabels;
}

export function hasInventoryItemLabel(id: string): boolean {
  return id in inventoryItemLabels;
}

function fallbackLabel(id: string): string {
  return id.replaceAll('_', ' ');
}
