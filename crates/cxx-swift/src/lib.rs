mod abi;
mod ffi {
    pub(crate) mod llvm {
        pub(crate) mod adt {
            pub(crate) mod small_vector {
                pub(crate) mod swift {
                    pub(crate) mod ast {
                        pub(crate) mod identifier;
                    }
                }
            }
            pub(crate) mod small_vector_impl {
                pub(crate) mod swift {
                    pub(crate) mod ast {
                        pub(crate) mod identifier;
                    }
                }
            }
        }
    }
    pub(crate) mod swift {
        pub(crate) mod ast {
            pub(crate) mod ast_context;
            pub(crate) mod ast_walker;
            pub(crate) mod ast_walker_base;
            pub(crate) mod ast_walker_rust;
            pub(crate) mod decl;
            pub(crate) mod dependency_tracker;
            pub(crate) mod diagnostic_engine;
            pub(crate) mod expr;
            pub(crate) mod identifier;
            pub(crate) mod import_path {
                pub(crate) mod module;
            }
            pub(crate) mod module_decl;
            pub(crate) mod pattern;
            pub(crate) mod search_path_options;
            pub(crate) mod sil_options;
            pub(crate) mod stmt;
            pub(crate) mod type_repr;
        }
        pub(crate) mod basic {
            pub(crate) mod clang_importer_options;
            pub(crate) mod lang_options;
            pub(crate) mod source_loc;
            pub(crate) mod source_manager;
            pub(crate) mod type_checker_options;
        }
        pub(crate) mod clang_importer {
            pub(crate) mod clang_importer;
            pub(crate) mod effective_clang_context;
            pub(crate) mod serialized_swift_name;
            pub(crate) mod swift_lookup_table;
        }
        pub(crate) mod symbol_graph_gen {
            pub(crate) mod symbol_graph_options;
        }
    }
}
mod gen {
    pub(crate) mod llvm {
        pub(crate) mod adt {
            pub(crate) mod small_vector {
                pub(crate) mod swift {
                    pub(crate) mod ast {
                        pub(crate) mod identifier;
                    }
                }
            }
            pub(crate) mod small_vector_impl {
                pub(crate) mod swift {
                    pub(crate) mod ast {
                        pub(crate) mod identifier;
                    }
                }
            }
        }
    }
    pub(crate) mod swift {
        pub(crate) mod ast {
            pub(crate) mod ast_context;
            pub(crate) mod ast_walker;
            pub(crate) mod ast_walker_base;
            pub(crate) mod ast_walker_rust;
            pub(crate) mod decl;
            pub(crate) mod dependency_tracker;
            pub(crate) mod diagnostic_engine;
            pub(crate) mod expr;
            pub(crate) mod import_path {
                pub(crate) mod module;
            }
            pub(crate) mod module_decl;
            pub(crate) mod pattern;
            pub(crate) mod search_path_options;
            pub(crate) mod sil_options;
            pub(crate) mod stmt;
            pub(crate) mod type_repr;
        }
        pub(crate) mod basic {
            pub(crate) mod clang_importer_options;
            pub(crate) mod lang_options;
            pub(crate) mod llvm_initialize;
            pub(crate) mod source_loc;
            pub(crate) mod source_manager;
            pub(crate) mod type_checker_options;
        }
        pub(crate) mod clang_importer {
            pub(crate) mod clang_importer;
            pub(crate) mod effective_clang_context;
            pub(crate) mod serialized_swift_name;
            pub(crate) mod swift_lookup_table;
        }
        pub(crate) mod symbol_graph_gen {
            pub(crate) mod symbol_graph_options;
        }
    }
}
pub mod util;
pub mod swift {
    pub mod ast {
        pub mod ast_walker {
            pub mod vtable {
                pub use crate::ffi::swift::ast::ast_walker::vtable::{AstWalkerAbstract, AstWalkerConcrete};
            }
            pub use crate::ffi::swift::ast::{
                ast_walker::parent_ty::ParentTy,
                ast_walker_base::AstWalkerBase,
                ast_walker_rust::AstWalkerRust,
            };
        }
        pub mod decl {
            pub use crate::ffi::swift::ast::decl::Decl;
        }
        pub mod expr {
            pub use crate::ffi::swift::ast::expr::Expr;
        }
        pub mod identifier {
            pub use crate::ffi::swift::ast::identifier::Identifier;
        }
        pub mod import_path {
            pub mod module {
                pub use crate::ffi::swift::ast::import_path::module::builder::Builder;
            }
            pub use crate::ffi::swift::ast::import_path::module::Module;
        }
        pub mod module_decl {
            pub use crate::ffi::swift::ast::module_decl::ModuleDecl;
        }
        pub mod stmt {
            pub use crate::ffi::swift::ast::stmt::Stmt;
        }
    }
    pub mod basic {
        pub mod clang_importer_options {
            pub use crate::ffi::swift::basic::clang_importer_options::ClangImporterOptions;
        }
        pub mod lang_options {
            pub use crate::ffi::swift::basic::lang_options::LangOptions;
        }
        pub mod source_loc {
            pub use crate::ffi::swift::basic::source_loc::SourceLoc;
        }
        pub mod source_manager {
            pub use crate::ffi::swift::basic::source_manager::SourceManager;
        }
        pub mod type_checker_options {
            pub use crate::ffi::swift::basic::type_checker_options::TypeCheckerOptions;
        }
    }
    pub mod clang_importer {
        pub mod clang_importer {
            pub use crate::ffi::swift::clang_importer::clang_importer::ClangImporter;
        }
        pub mod effective_clang_context {
            pub use crate::ffi::swift::clang_importer::effective_clang_context::EffectiveClangContext;
        }
        pub mod swift_lookup_table {
            pub mod single_entry {
                pub mod owning_iterator {
                    pub use crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::owning_iterator::SwiftLookupTableSingleEntryOwningIterator;
                }
                pub use crate::ffi::swift::clang_importer::swift_lookup_table::single_entry::SwiftLookupTableSingleEntry;
            }
            pub use crate::ffi::swift::clang_importer::swift_lookup_table::SwiftLookupTable;
        }
        pub mod serialized_swift_name {
            pub mod owning_iterator {
                pub use crate::ffi::swift::clang_importer::serialized_swift_name::owning_iterator::SerializedSwiftNameOwningIterator;
            }
            pub use crate::ffi::swift::clang_importer::serialized_swift_name::SerializedSwiftName;
        }
    }
    pub mod symbol_graph_gen {
        pub mod symbol_graph_options {
            pub use crate::ffi::swift::symbol_graph_gen::symbol_graph_options::SymbolGraphOptions;
        }
        pub use crate::ffi::swift::symbol_graph_gen::symbol_graph_options::SymbolGraphOptions;
    }
    pub use crate::{
        ffi::swift::{
            ast::{
                ast_context::AstContext,
                ast_walker::AstWalker,
                decl::Decl,
                diagnostic_engine::DiagnosticEngine,
                expr::Expr,
                identifier::Identifier,
                module_decl::ModuleDecl,
                pattern::Pattern,
                search_path_options::SearchPathOptions,
                sil_options::SilOptions,
                stmt::Stmt,
                type_repr::TypeRepr,
            },
            basic::{
                clang_importer_options::ClangImporterOptions,
                lang_options::LangOptions,
                source_loc::SourceLoc,
                source_manager::SourceManager,
                type_checker_options::TypeCheckerOptions,
            },
            clang_importer::{
                clang_importer::ClangImporter,
                effective_clang_context::EffectiveClangContext,
                serialized_swift_name::{owning_iterator::SerializedSwiftNameOwningIterator, SerializedSwiftName},
                swift_lookup_table::{
                    single_entry::{
                        owning_iterator::SwiftLookupTableSingleEntryOwningIterator,
                        SwiftLookupTableSingleEntry,
                    },
                    SwiftLookupTable,
                },
            },
        },
        gen::swift::basic::llvm_initialize::initialize_llvm,
    };
}

#[cfg(not(feature = "debug"))]
#[doc(hidden)]
pub trait TracingDebug {}
#[cfg(not(feature = "debug"))]
impl<T> TracingDebug for T {
}

#[cfg(feature = "debug")]
#[doc(hidden)]
pub trait TracingDebug: core::fmt::Debug {}
#[cfg(feature = "debug")]
impl<T: core::fmt::Debug> TracingDebug for T {
}

use core::{mem::MaybeUninit, pin::Pin};

pub struct Initializer<This, Data> {
    pub(crate) call: unsafe fn(Pin<&mut MaybeUninit<This>>, Data),
    pub(crate) data: Data,
}

impl<This, Data> Initializer<This, Data> {
    #[inline]
    pub(crate) fn new(call: unsafe fn(Pin<&mut MaybeUninit<This>>, Data), data: Data) -> Self {
        Self { call, data }
    }
}

unsafe impl<This, Data> cxx_memory::New for Initializer<This, Data> {
    type Output = This;

    #[inline]
    unsafe fn new(self, this: Pin<&mut MaybeUninit<Self::Output>>) {
        (self.call)(this, self.data)
    }
}
