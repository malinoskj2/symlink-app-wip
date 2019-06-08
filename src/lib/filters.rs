use super::types::{LinkData, Linkfile};

/// If the user specifies tags via the -t CLI arg then only file/links
/// that match at least one of the specified tags should be executed
/// this function compares the list tags provided via CLI to the tags
/// found in a config file and returns true if there was at least one match
pub fn filter_tags(tag_list: &[impl AsRef<str>], link_map: &Linkfile<LinkData>) -> bool {
    tag_list
        .iter()
        .any(|tag| link_map.contains_tag(tag.as_ref()))
}
