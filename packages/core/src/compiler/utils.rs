pub mod go {
    use std::{
        collections::HashSet,
        path::{Path, PathBuf, StripPrefixError},
    };

    use crate::{path::diff_paths, types::config::Config};

    pub fn get_root_pkg_name(s: &Config) -> &str {
        s.output
            .go
            .as_ref()
            .expect("Maybe you want generate Go code but forget to define it in config.")
            .output
            .split('/')
            .last()
            .unwrap()
    }

    /// replace selien root with package name.
    ///
    /// **@return** Result<PathBuf, Error>
    ///
    /// Error will be return when relative path out of selien-root.
    ///
    /// `e.g.` If config is:
    ///
    /// ```yaml
    /// spec:
    ///   root: selien-spec
    /// output:
    ///   go:
    ///     mod_name: selien
    ///     root: packages/server
    ///     output: packages/server/api/selien_spec
    /// ```
    /// we will get file path like `selien-spec/foo/bar`, (last bar is file name)
    ///
    /// so, we replace `selien-spec` with `selien/api/selien_spec`
    pub fn replace_selien_root(p: &Path, s: &Config) -> Result<PathBuf, StripPrefixError> {
        let o = s
            .output
            .go
            .as_ref()
            .expect("Maybe you want to generate Go code but forget to set it in config file.");

        let import_root = PathBuf::from(&o.mod_name);

        let goroot_to_output =
            diff_paths(&o.output, &o.root).expect("Can not diff path from go-root to output");

        let striped = p.strip_prefix(&s.spec.root)?;
        Ok(import_root.join(goroot_to_output).join(striped))
    }

    pub fn remove_duplicate_import(s: &str) -> String {
        let lines: HashSet<&str> = s.lines().collect();
        lines.into_iter().collect::<Vec<&str>>().join("\n")
    }
}

pub mod ts {
    use crate::generator::types::{Import, Imports};
    use std::collections::{HashMap, HashSet};

    pub fn reduce_imports(ipts: &Imports) -> Imports {
        let mut map: HashMap<String, Import> = HashMap::new();

        for ipt in ipts {
            match ipt {
                Import::Dyn(di) => {
                    let entry = map.entry(di.from.clone()).or_insert_with(|| ipt.clone());
                    if let Import::Dyn(d) = entry {
                        d.name = format!("{}, {}", d.name, di.name);
                    }
                }
                Import::Ref(ri) => {
                    let entry = map.entry(ri.from.clone()).or_insert_with(|| ipt.clone());
                    if let Import::Ref(r) = entry {
                        r.name = format!("{}, {}", r.name, ri.name);
                    }
                }
            }
        }

        map.iter_mut().for_each(|(_, v)| {
            if let Import::Dyn(d) = v {
                d.name = remove_duplicate_import(&d.name);
            }
            if let Import::Ref(r) = v {
                r.name = remove_duplicate_import(&r.name);
            }
        });

        let result: Vec<Import> = map.into_values().collect();
        result
    }

    fn remove_duplicate_import(s: &str) -> String {
        let parts: HashSet<&str> = s.split(", ").collect();
        parts.into_iter().collect::<Vec<&str>>().join(", ")
    }
}
