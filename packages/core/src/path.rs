use std::env;
use std::path::{Component, Path, PathBuf};

use log::warn;

/// Process path from input with normalize.
/// If input is relative path, then join it with current working directory.
pub fn process_path<P: AsRef<Path>>(input: P) -> PathBuf {
    let mut path = input.as_ref().to_path_buf();

    if path.is_relative() {
        let cwd = env::current_dir().expect("Can not get current working dir.");
        path = cwd.join(path);
    }

    path = normalize_path(&path);

    path
}

pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

/// Convert absolute selien $ref path to relative path.
///
/// `e.g.` /rest/user -> rest/user
pub fn to_relative(p: &Path) -> PathBuf {
    if p.is_relative() {
        return p.to_owned();
    }

    if !p.starts_with("/") {
        warn!("$ref absolute path should start with /: {}", p.display());
        return p.to_owned();
    }

    p.strip_prefix("/").unwrap().to_path_buf()
}

/// add dot to relative path.
///
/// `e.g.` home/username/selien -> ./home/username/selien
pub fn add_dot(p: &Path) -> PathBuf {
    let mut path = PathBuf::from(".");
    path.push(p);
    path
}

pub fn diff_paths<P, B>(path: P, base: B) -> Option<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let base = base.as_ref();

    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();
        let mut comps: Vec<Component> = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}
