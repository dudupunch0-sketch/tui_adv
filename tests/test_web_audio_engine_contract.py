from pathlib import Path


def test_web_audio_engine_skeleton_is_renderer_local_and_asset_free():
    engine_path = Path("web/src/ui/audio/audioEngine.ts")
    assert engine_path.exists()

    engine = engine_path.read_text(encoding="utf-8")
    main = Path("web/src/main.ts").read_text(encoding="utf-8")
    core_types = Path("web/src/core/types.ts").read_text(encoding="utf-8")

    assert "createStorybookAudioEngine" in main
    assert "unlockFromUserGesture" in main
    assert "audioCueForSceneTransition" in main
    assert "syncAmbience" in main

    assert "GeneratedAudioBackend" in engine
    assert "AudioContext" in engine
    assert "createOscillator" in engine
    assert "muted" in engine
    assert "ScenePage" in engine

    for forbidden_schema_field in (
        "audio_cues",
        "soundtrack",
        "music",
        "ambience_cues",
    ):
        assert forbidden_schema_field not in core_types

    binary_assets = []
    for pattern in ("*.mp3", "*.wav", "*.ogg", "*.flac", "*.aac"):
        binary_assets.extend(Path("web/src").rglob(pattern))
    assert binary_assets == []
