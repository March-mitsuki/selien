pub mod go {
    use std::path::{Path, PathBuf, StripPrefixError};

    use crate::types::config::Config;

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
    ///     modName: selien
    ///     output: packages/server/api_spec
    /// ```
    /// we will get file path like `selien-spec/foo/bar`, (last bar is file name)
    ///
    /// so, we replace `selien-spec` with `selien/api_spec`
    pub fn replace_selien_root(p: &Path, s: &Config) -> Result<PathBuf, StripPrefixError> {
        let o = s
            .output
            .go
            .as_ref()
            .expect("Maybe you want to generate Go code but forget to set it in config file.");
        let go_root = PathBuf::from(&o.mod_name);

        let striped = p.strip_prefix(&s.spec.root)?;
        let pkg_name = get_root_pkg_name(s);
        Ok(go_root.join(pkg_name).join(striped))
    }
}
