use super::types::{LinkData, Linkfile};

pub fn filter_tags(tag_list: &[impl AsRef<str>], link_map: &Linkfile<LinkData>) -> bool {
    tag_list
        .iter()
        .any(|tag| link_map.contains_tag(tag.as_ref()))
}
