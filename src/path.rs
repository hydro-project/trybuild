use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! path {
    ($($tt:tt)+) => {
        $crate::tokenize_path!([] [] $($tt)+)
    };
}

// Private implementation detail.
#[macro_export]
macro_rules! tokenize_path {
    ([$(($($component:tt)+))*] [$($cur:tt)+] /) => {
        $crate::directory::Directory::new($crate::tokenize_path!([$(($($component)+))*] [$($cur)+]))
    };

    ([$(($($component:tt)+))*] [$($cur:tt)+] / $($rest:tt)+) => {
        $crate::tokenize_path!([$(($($component)+))* ($($cur)+)] [] $($rest)+)
    };

    ([$(($($component:tt)+))*] [$($cur:tt)*] $first:tt $($rest:tt)*) => {
        $crate::tokenize_path!([$(($($component)+))*] [$($cur)* $first] $($rest)*)
    };

    ([$(($($component:tt)+))*] [$($cur:tt)+]) => {
        $crate::tokenize_path!([$(($($component)+))* ($($cur)+)])
    };

    ([$(($($component:tt)+))*]) => {{
        let mut path = std::path::PathBuf::new();
        $(
            path.push(&($($component)+));
        )*
        path
    }};
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub(crate) struct CanonicalPath(PathBuf);

impl CanonicalPath {
    pub(crate) fn new(path: &Path) -> Self {
        if let Ok(canonical) = path.canonicalize() {
            CanonicalPath(canonical)
        } else {
            CanonicalPath(path.to_owned())
        }
    }
}

#[test]
fn test_path_macro() {
    struct Project {
        dir: PathBuf,
    }

    let project = Project {
        dir: PathBuf::from("../target/tests"),
    };

    let cargo_dir = path!(project.dir / ".cargo" / "config.toml");
    assert_eq!(cargo_dir, Path::new("../target/tests/.cargo/config.toml"));
}
