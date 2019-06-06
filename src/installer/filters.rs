use super::types::{ConfigMap, ConfigLink};

pub fn filter_tags(tag_list: &[impl AsRef<str>], link_map: &ConfigMap<ConfigLink>) -> bool {
    tag_list.iter().
        any(|tag| link_map.contains_tag(tag.as_ref()))
}
