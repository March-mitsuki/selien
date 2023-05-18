use crate::types::lang::SupportedLang;

use super::lang::{go, typescript};
use super::types::{Imports, TabSize, AST};

impl AST {
    pub fn generator(
        &self,
        lang: &SupportedLang,
        imports: &mut Imports,
        tabsize: &TabSize,
    ) -> String {
        match lang {
            SupportedLang::Go => go::generate_go(self, imports, tabsize.go),
            SupportedLang::TypeScript => {
                typescript::generate_typescript(self, imports, tabsize.typescript)
            }
        }
    }
}
