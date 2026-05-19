from dataclasses import replace

import pytest

from tui_adv.game.items import DEFAULT_ITEMS, use_item
from tui_adv.game.state import GameState, PlayerState


def test_default_consumable_items_load_use_effects():
    water = DEFAULT_ITEMS["bottled_water"]
    snack = DEFAULT_ITEMS["snack"]
    power_bank = DEFAULT_ITEMS["power_bank"]

    assert water.usable is True
    assert water.use_effects == {"thirst": -35}
    assert "생수" in water.use_log
    assert snack.use_effects == {"hunger": -25}
    assert power_bank.use_effects == {"battery": 35}


def test_use_consumable_item_applies_effect_consumes_inventory_and_advances_turn():
    state = replace(
        GameState.new(seed=1),
        player=PlayerState(hunger=20, thirst=70),
        inventory=["bottled_water"],
        log=["목이 말라서 정수기 소리가 크게 들린다."],
    )

    used = use_item(state, "bottled_water")

    assert used.turn == 1
    assert used.player.thirst == 37
    assert used.player.hunger == 21
    assert used.inventory == []
    assert "생수" in used.log[-1]
    assert state.inventory == ["bottled_water"]
    assert state.player.thirst == 70


def test_use_item_rejects_missing_or_unusable_inventory_without_mutation():
    state = replace(GameState.new(seed=1), inventory=["crumpled_printout"])

    with pytest.raises(ValueError, match="사용할 수 없는 아이템: crumpled_printout"):
        use_item(state, "crumpled_printout")
    with pytest.raises(ValueError, match="소지품에 없는 아이템: bottled_water"):
        use_item(state, "bottled_water")

    assert state.inventory == ["crumpled_printout"]
    assert state.turn == 0
