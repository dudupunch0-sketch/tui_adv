use super::*;

pub(crate) const DEFAULT_STORYPACK_ID: &str = "wuxia_jianghu_pack";
pub(crate) const DEFAULT_STORYPACK_BUNDLE_REL: &str =
    "../escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json";

pub(crate) fn selected_content_bundle_path(options: &CliOptions) -> Result<PathBuf, String> {
    if let Some(bundle_path) = &options.content_bundle {
        return Ok(bundle_path.clone());
    }
    if let Some(storypack_id) = &options.storypack_preview {
        return storypack_preview_bundle_path(storypack_id);
    }
    Ok(default_storypack_bundle_path())
}
pub(crate) fn storypack_preview_bundle_path(storypack_id: &str) -> Result<PathBuf, String> {
    if storypack_id != DEFAULT_STORYPACK_ID {
        return Err(format!(
            "unsupported --storypack-preview '{storypack_id}'; available: {DEFAULT_STORYPACK_ID}"
        ));
    }
    Ok(default_storypack_bundle_path())
}
pub(crate) fn default_storypack_bundle_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_STORYPACK_BUNDLE_REL)
}
