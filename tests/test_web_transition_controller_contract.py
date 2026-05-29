from pathlib import Path


def test_transition_controller_is_wired_into_web_storybook_renderer_locally():
    main = Path("web/src/main.ts").read_text(encoding="utf-8")
    css = Path("web/src/styles/storybook.css").read_text(encoding="utf-8")

    assert "createStorybookTransitionController" in main
    assert "transitionController.transitionTo" in main
    assert "previousPage" in main
    assert "nextPage" in main
    assert "resolveMotionMode" in main
    assert "transitionController.cancel" in main

    for fragment in [
        "storybook-transition-shell",
        "storybook-transition-exit",
        "storybook-transition-enter",
        "data-transition-name",
        "data-transition-phase",
        "data-transition-danger",
    ]:
        assert fragment in css


def test_transition_controller_keeps_audio_and_schema_out_of_pr_b_scope():
    controller = Path("web/src/ui/motion/transitionController.ts").read_text(encoding="utf-8")
    main = Path("web/src/main.ts").read_text(encoding="utf-8")

    assert "ScenePage" in controller
    assert "AudioContext" not in controller
    assert "audio" not in controller.lower()
    assert "interface ScenePage" not in main
    assert "status_summary:" not in main
