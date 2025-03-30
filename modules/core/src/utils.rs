use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};

pub fn get_version() -> String {
    format!(
        "{}.{}.{}",
        pkg_version_major!(),
        pkg_version_minor!(),
        pkg_version_patch!()
    )
}
