#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate derive_builder;

pub use failure::Error as FailErr;

mod error;
pub mod exec;
mod filters;
pub mod option;
mod types;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use galvanic_test::test_suite;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};
    use std::time::{Duration, SystemTime};
    use tempfile::{tempdir, TempDir};

    test_suite! {
        name tags;

        const TEST_TAGS_A: [&str; 3] = ["tag_a", "tag_b", "tag_c"];
        // has common with a
        const TEST_TAGS_B: [&str; 3] = ["tag_a", "tag_d", "tag_f"];
        // has none common with a
        const TEST_TAGS_C: [&str; 3] = ["tag_g", "tag_h", "tag_i"];
        use crate::types::*;
        use std::path::{PathBuf, Path};

        fn test_linkdata() -> LinkData {
                 LinkDataBuilder::default()
            .source(PathBuf::from("/tmp/testsource"))
            .destination(PathBuf::from("/tmp/testsource"))
            .method(LinkMethod::Link)
            .options(LinkOptions::default())
            .filters(LinkConditions::default())
            .build()
            .expect("failed to create_test_linkdata()")
        }

        fixture test_linkfile() -> Linkfile<LinkData> {
            setup(&mut self) {
                    LinkfileBuilder::default()
                    .tags(
                TEST_TAGS_B
                    .into_iter()
                    .map(|tag_str| tag_str.to_string())
                    .collect(),
            )
            .links(vec![test_linkdata()])
            .build()
            .expect("failed to create_test_linkfile()")
            }
        }

        test filter_tags_true(test_linkfile) {
            use crate::filters::filter_tags;
            assert!(filter_tags(&TEST_TAGS_A, &test_linkfile.val));
        }

        test filter_tags_false(test_linkfile) {
              use crate::filters::filter_tags;
              assert!(!filter_tags(&TEST_TAGS_C, &test_linkfile.val));
        }

        test linkfile_contains_tag_true(test_linkfile) {
            TEST_TAGS_B.iter()
                .for_each(|test_tag| assert!(test_linkfile.val.contains_tag(test_tag)))
        }

        test linkfile_contains_tag_false(test_linkfile) {
            TEST_TAGS_C.iter()
                .for_each(|test_tag| assert!(!test_linkfile.val.contains_tag(test_tag)))
        }
    }

    test_suite! {
        name link_creation;
        use tempfile::{tempdir, TempDir};
        use crate::types::*;
        use std::fs::File;
        use std::io::Write;

        fixture link_data() -> LinkData {
                members {
                     dir: Option<TempDir>
                }
                setup(&mut self) {
                     self.dir = Some(tempdir().expect("failed to create tempdir"));
                     let source_file_text = "test source file text";
                     let source_file_path = self.dir.as_ref().unwrap().path().join("source.txt");

                     let mut source_file = File::create(source_file_path.as_path())
                     .expect("failed to create file");
                     write!(source_file, "{}", source_file_text);

                     let dest_file_path = self.dir.as_ref().unwrap().path().join("dest.txt");

                     LinkDataBuilder::default()
                     .source(source_file_path.as_path())
                     .destination(dest_file_path.as_path())
                     .method(LinkMethod::Link)
                     .options(LinkOptions::default())
                     .filters(LinkConditions::default())
                     .build()
                     .expect("failed to build test LinkData")

                }
        }

        test create_link(link_data) {
            link_data.val.create_link().expect("failed to create links");
               assert!(link_data.val.destination.exists());
               assert_eq!(std::fs::read_to_string(link_data.val.source)
               .expect("failed to read source file"),
                          std::fs::read_to_string(link_data.val.destination)
                          .expect("failed to read destination file"));
        }

        test destination_dir_exists_true(link_data) {
            let dest_file = File::create(&link_data.val.destination);
            assert!(link_data.val.destination_dir_exists());
        }
    }
}
