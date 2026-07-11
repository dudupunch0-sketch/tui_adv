const achievementLabels: Record<string, string> = {
  wuxia_first_arrival: '강호 출근',
  wuxia_first_fragment_seen: '천기록 첫 편린',
  first_signal_received: '첫 신호 확인',
  reality_link_discovered: '현실과 접속한 사람',
  reality_link_second_marker: '두 번째 현실 표식',
  reality_link_third_marker: '세 번째 현실 표식',
  broadcast_channel_captured: '사내 방송 장악자',
  truth_protocol_understood: '격리 프로토콜 독해',
  network_admin_claimed: '사내망 관리자',
  rooftop_signal_sent: '외부 신호 송신',
  parking_lot_escape_driver: '지하주차장 탈출자',
  lobby_exit_commuter: '로비 게이트 통과자',
  executive_approval_holder: '대표 승인권자',
};

const inventoryItemLabels: Record<string, string> = {
  commuter_badge: '사원증',
  cheonggi_record_notebook: '천기록이 깃든 업무수첩',
  work_chore_token: '청류문 잡일패',
  rejected_aid_letter_fragment: '거절당한 도움 요청 서찰 조각',
  bottled_water: '생수',
  coffee: '커피',
  snack: '과자',
  cup_noodle: '컵라면',
  first_aid_kit: '구급상자',
  power_bank: '보조배터리',
  flashlight: '손전등',
  employee_badge: '사원증',
  security_override_badge: '보안실 우회권한',
  crumpled_printout: '구겨진 출력물',
  ex_employee_memo: '퇴사자의 메모',
  parking_key_fob: '지하주차장 키태그',
  visitor_badge: '임시 방문증',
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
