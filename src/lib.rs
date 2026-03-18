//! # Ranty
//!
//! Ranty is a high-level procedural templating language.
//! It is designed to help you write more dynamic and expressive templates, dialogue, stories, names, test data, and much more.
//!
//! The language reference and CLI documentation live in the repository's `docs/` directory.
//!
//! ## The Ranty context
//!
//! All programs are run through a Ranty context, represented by the [`Ranty`] struct.
//! It allows you to execute Ranty programs, define and retrieve global variables, manipulate the RNG, and compile Ranty code.
//!
//! ## Reading compiler errors
//!
//! You will notice that the `Err` variant of the `Ranty::compile*` methods is `()` instead of providing an error list. Instead,
//! errors and warnings are reported via implementors of the [`Reporter`] trait, which allows the user to control what happens to messages emitted by the compiler.
//! Currently, Ranty has two built-in `Reporter` implementations: the unit type `()`, and `Vec<CompilerMessage>`.
//! You can also make your own custom reporters to suit your specific needs.
//!
//! [`Ranty`]: struct.Ranty.html
//! [`Reporter`]: compiler/trait.Reporter.html
//! [`Vec<CompilerMessage>`]: compiler/struct.CompilerMessage.html

// Some branches are incorrectly detected as dead
#![allow(dead_code)]
// Some macro usages aren't detected, causing false warnings
#![allow(unused_macros)]
// Disable clippy's silly whining about "VM", "IO", etc. in type names
#![allow(clippy::upper_case_acronyms)]

// Public modules
pub mod compiler;
pub mod data;
pub mod runtime;

// Internal modules
mod collections;
mod convert;
mod format;
mod func;
mod gc;
mod lang;
mod modres;
mod rng;
mod selector;
mod stdlib;
mod string;
mod util;
mod value;
mod value_eq;
mod var;

#[cfg(test)]
mod gc_tests;

// Re-exports
pub use crate::collections::*;
pub use crate::convert::*;
pub use crate::func::*;
pub use crate::modres::*;
pub use crate::selector::*;
pub use crate::string::*;
pub use crate::value::*;
pub use crate::var::*;

use crate::compiler::*;
use crate::lang::Sequence;
use crate::rng::RantyRng;
use crate::runtime::{IntoRuntimeResult, RuntimeError, RuntimeErrorType, RuntimeResult, VM};

use data::DataSource;
use fnv::FnvBuildHasher;
use rand::Rng;
use std::env;
use std::error::Error;
use std::{collections::HashMap, fmt::Display, path::Path, path::PathBuf, rc::Rc};

type IOErrorKind = std::io::ErrorKind;

pub(crate) type InternalString = smartstring::alias::CompactString;

/// The build version according to the crate metadata at the time of compiling.
pub const BUILD_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The Ranty language version implemented by this library.
pub const RANTY_LANG_VERSION: &str = "1.0.0";

/// The default name given to programs compiled from raw strings.
pub const DEFAULT_PROGRAM_NAME: &str = "program";

/// The preferred file extension for Ranty source files and modules.
pub const RANTY_FILE_EXTENSION: &str = "ranty";

/// The legacy file extension still accepted for compatibility.
pub const RANTY_COMPAT_FILE_EXTENSION: &str = "rant";

/// Supported file extensions in resolution order for extensionless module paths.
pub const RANTY_SUPPORTED_FILE_EXTENSIONS: [&str; 2] =
    [RANTY_FILE_EXTENSION, RANTY_COMPAT_FILE_EXTENSION];

/// Name of global variable that stores cached modules.
pub(crate) const MODULES_CACHE_KEY: &str = "__MODULES";

/// Immediately runs cycle collection for the current thread's Ranty heap.
pub fn collect_garbage() {
    gc::collect();
}

fn normalize_module_cache_path<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .canonicalize()
        .unwrap_or_else(|_| path.as_ref().to_path_buf())
        .to_string_lossy()
        .into_owned()
}

pub(crate) fn module_request_cache_key(
    module_path: &str,
    dependant: Option<&RantyProgramInfo>,
) -> String {
    let normalized_module_path = module_path.replace('\\', "/");
    let dependant_root = dependant
        .and_then(|info| info.path())
        .map(PathBuf::from)
        .or_else(|| env::current_dir().ok())
        .and_then(|path| path.parent().map(PathBuf::from).or(Some(path)))
        .map(normalize_module_cache_path);

    match dependant_root {
        Some(root) => format!("req:{root}::{normalized_module_path}"),
        None => format!("req::{normalized_module_path}"),
    }
}

pub(crate) fn module_resolved_cache_key(program: &RantyProgram) -> Option<String> {
    program
        .path()
        .map(|path| format!("path:{}", normalize_module_cache_path(path)))
}

/// A Ranty execution context.
#[derive(Debug)]
pub struct Ranty {
    options: RantyOptions,
    module_resolver: Rc<dyn ModuleResolver>,
    rng: Rc<RantyRng>,
    data_sources: HashMap<InternalString, Box<dyn DataSource>, FnvBuildHasher>,
    globals: HashMap<InternalString, RantyVar, FnvBuildHasher>,
}

impl Ranty {
    /// Creates a new Ranty context with the default seed (0) and loads the standard library.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_seed(0)
    }

    /// Creates a new Ranty context with the specified seed and loads the standard library.
    pub fn with_seed(seed: u64) -> Self {
        Self::with_options(RantyOptions {
            seed,
            ..Default::default()
        })
    }

    /// Creates a new Ranty context with a seed generated by a thread-local PRNG and loads the standard library.
    pub fn with_random_seed() -> Self {
        Self::with_options(RantyOptions {
            seed: rand::thread_rng().gen(),
            ..Default::default()
        })
    }

    /// Creates a new Ranty context with the specified options.
    #[inline(always)]
    pub fn with_options(options: RantyOptions) -> Self {
        let mut ranty = Self {
            module_resolver: Rc::new(DefaultModuleResolver::default()),
            globals: Default::default(),
            data_sources: Default::default(),
            rng: Rc::new(RantyRng::new(options.seed)),
            options,
        };

        // Load standard library
        if ranty.options.use_stdlib {
            stdlib::load_stdlib(&mut ranty);
        }

        ranty
    }

    /// Replaces the module resolver.
    #[inline]
    pub fn using_module_resolver<R: ModuleResolver + 'static>(self, module_resolver: R) -> Self {
        Self {
            module_resolver: Rc::new(module_resolver),
            ..self
        }
    }
}

impl Default for Ranty {
    /// Creates a default `Ranty` instance.
    fn default() -> Self {
        Self::new()
    }
}

impl Ranty {
    /// Compiles a source string using the specified reporter.
    #[must_use = "compiling a program without storing or running it achieves nothing"]
    pub fn compile<R: Reporter>(
        &self,
        source: &str,
        reporter: &mut R,
    ) -> Result<RantyProgram, CompilerError> {
        compiler::compile_string(
            source,
            reporter,
            self.options.debug_mode,
            RantyProgramInfo {
                name: None,
                path: None,
            },
        )
    }

    /// Compiles a source string using the specified reporter and source name.
    #[must_use = "compiling a program without storing or running it achieves nothing"]
    pub fn compile_named<R: Reporter>(
        &self,
        source: &str,
        reporter: &mut R,
        name: &str,
    ) -> Result<RantyProgram, CompilerError> {
        compiler::compile_string(
            source,
            reporter,
            self.options.debug_mode,
            RantyProgramInfo {
                name: Some(name.to_owned()),
                path: None,
            },
        )
    }

    /// Compiles a source string without reporting problems.
    ///
    /// ## Note
    ///
    /// This method will not generate any compiler messages, even if it fails.
    ///
    /// If you require this information, use the `compile()` method instead.
    #[must_use = "compiling a program without storing or running it achieves nothing"]
    pub fn compile_quiet(&self, source: &str) -> Result<RantyProgram, CompilerError> {
        compiler::compile_string(
            source,
            &mut (),
            self.options.debug_mode,
            RantyProgramInfo {
                name: None,
                path: None,
            },
        )
    }

    /// Compiles a source string without reporting problems and assigns it the specified name.
    ///
    /// ## Note
    ///
    /// This method will not generate any compiler messages, even if it fails.
    ///
    /// If you require this information, use the `compile()` method instead.
    #[must_use = "compiling a program without storing or running it achieves nothing"]
    pub fn compile_quiet_named(
        &self,
        source: &str,
        name: &str,
    ) -> Result<RantyProgram, CompilerError> {
        compiler::compile_string(
            source,
            &mut (),
            self.options.debug_mode,
            RantyProgramInfo {
                name: Some(name.to_owned()),
                path: None,
            },
        )
    }

    /// Compiles a source file using the specified reporter.
    #[must_use = "compiling a program without storing or running it achieves nothing"]
    pub fn compile_file<P: AsRef<Path>, R: Reporter>(
        &self,
        path: P,
        reporter: &mut R,
    ) -> Result<RantyProgram, CompilerError> {
        compiler::compile_file(path, reporter, self.options.debug_mode)
    }

    /// Compiles a source file without reporting problems.
    ///
    /// ## Note
    ///
    /// This method will not generate any compiler messages, even if it fails.
    ///
    /// If you require this information, use the `compile_file()` method instead.
    #[must_use = "compiling a program without storing or running it achieves nothing"]
    pub fn compile_file_quiet<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<RantyProgram, CompilerError> {
        compiler::compile_file(path, &mut (), self.options.debug_mode)
    }

    /// Sets a global variable. This will auto-define the global if it doesn't exist.
    ///
    /// If the global already exists and is a constant, the write will not succeed.
    ///
    /// Returns `true` if the write succeeded; otherwise, `false`.
    #[inline]
    pub fn set_global(&mut self, key: &str, value: RantyValue) -> bool {
        if let Some(global_var) = self.globals.get_mut(key) {
            global_var.write(value)
        } else {
            self.globals
                .insert(InternalString::from(key), RantyVar::ByVal(value));
            true
        }
    }

    /// Sets a global constant. This will auto-define the global if it doesn't exist.
    ///
    /// If the global already exists and is a constant, the write will not succeed.
    ///
    /// Returns `true` if the write succeeded; otherwise, `false`.
    #[inline]
    pub fn set_global_const(&mut self, key: &str, value: RantyValue) -> bool {
        if let Some(global_var) = self.globals.get(key) {
            if global_var.is_const() {
                return false;
            }
        }
        self.globals
            .insert(InternalString::from(key), RantyVar::ByValConst(value));
        true
    }

    /// Sets a global's value, forcing the write even if the existing global is a constant.
    /// This will auto-define the global if it doesn't exist.
    #[inline]
    pub fn set_global_force(&mut self, key: &str, value: RantyValue, is_const: bool) {
        self.globals.insert(
            InternalString::from(key),
            if is_const {
                RantyVar::ByValConst(value)
            } else {
                RantyVar::ByVal(value)
            },
        );
    }

    /// Gets the value of a global variable.
    #[inline]
    pub fn get_global(&self, key: &str) -> Option<RantyValue> {
        self.globals.get(key).map(|var| var.value_cloned())
    }

    /// Gets a global variable by its `RantyVar` representation.
    #[inline]
    pub(crate) fn get_global_var(&self, key: &str) -> Option<&RantyVar> {
        self.globals.get(key)
    }

    /// Sets a global variable to the provided `RantyVar`.
    #[inline]
    pub(crate) fn set_global_var(&mut self, key: &str, var: RantyVar) {
        self.globals.insert(InternalString::from(key), var);
    }

    /// Gets a mutable reference to the `RantyVar` representation of the specified variable.
    #[inline]
    pub(crate) fn get_global_var_mut(&mut self, key: &str) -> Option<&mut RantyVar> {
        self.globals.get_mut(key)
    }

    /// Returns `true` if a global with the specified key exists.
    #[inline]
    pub fn has_global(&self, key: &str) -> bool {
        self.globals.contains_key(key)
    }

    /// Removes the global with the specified key. Returns `true` if the global existed prior to removal.
    #[inline]
    pub fn delete_global(&mut self, key: &str) -> bool {
        self.globals.remove(key).is_some()
    }

    /// Iterates over the names of all globals stored in the context.
    #[inline]
    pub fn global_names(&self) -> impl Iterator<Item = &str> {
        self.globals.keys().map(|k| k.as_str())
    }

    /// Gets the options used to initialize the context.
    pub fn options(&self) -> &RantyOptions {
        &self.options
    }

    /// Gets a mutable reference to the options used by the context.
    pub fn options_mut(&mut self) -> &mut RantyOptions {
        &mut self.options
    }

    /// Gets the current RNG seed.
    pub fn seed(&self) -> u64 {
        self.rng.seed()
    }

    /// Re-seeds the RNG with the specified seed.
    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Rc::new(RantyRng::new(seed));
    }

    /// Resets the RNG back to its initial state with the current seed.
    pub fn reset_seed(&mut self) {
        let seed = self.rng.seed();
        self.rng = Rc::new(RantyRng::new(seed));
    }

    /// Registers a data source to the context, making it available to scripts.
    pub fn add_data_source(
        &mut self,
        data_source: impl DataSource + 'static,
    ) -> Result<(), DataSourceRegisterError> {
        let id = data_source.type_id();

        if self.has_data_source(id) {
            return Err(DataSourceRegisterError::AlreadyExists(id.into()));
        }

        self.data_sources.insert(id.into(), Box::new(data_source));
        Ok(())
    }

    /// Removes the data source with the specified name from the context, making it no longer available to scripts.
    pub fn remove_data_source(&mut self, name: &str) -> Option<Box<dyn DataSource>> {
        self.data_sources.remove(name)
    }

    /// Returns a `bool` indicating whether a data source with the specified name is present in the context.
    pub fn has_data_source(&self, name: &str) -> bool {
        self.data_sources.contains_key(name)
    }

    /// Removes all data sources from the context.
    pub fn clear_data_sources(&mut self) {
        self.data_sources.clear();
    }

    /// Returns a reference to the data source associated with the specified name.
    pub fn data_source(&self, name: &str) -> Option<&dyn DataSource> {
        self.data_sources.get(name).map(Box::as_ref)
    }

    /// Iterates over all data sources (and their names) in the context.
    pub fn iter_data_sources(
        &self,
    ) -> impl Iterator<Item = (&'_ str, &'_ Box<dyn DataSource + 'static>)> {
        self.data_sources.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// Runs a program and returns the output value.
    pub fn run(&mut self, program: &RantyProgram) -> RuntimeResult<RantyValue> {
        let _gc_guard = gc::AllocationThresholdGuard::new(self.options.gc_allocation_threshold);
        let result = VM::new(self.rng.clone(), self, program).run();
        gc::collect();
        result
    }

    /// Runs a program with the specified arguments and returns the output value.
    pub fn run_with<A>(&mut self, program: &RantyProgram, args: A) -> RuntimeResult<RantyValue>
    where
        A: Into<Option<HashMap<String, RantyValue>>>,
    {
        let _gc_guard = gc::AllocationThresholdGuard::new(self.options.gc_allocation_threshold);
        let result = VM::new(self.rng.clone(), self, program).run_with(args);
        gc::collect();
        result
    }

    /// Immediately runs cycle collection for the current thread's Ranty heap.
    #[inline]
    pub fn collect_garbage(&self) {
        gc::collect();
    }

    pub fn try_load_global_module(&mut self, module_path: &str) -> Result<(), ModuleLoadError> {
        if PathBuf::from(&module_path)
            .with_extension("")
            .file_name()
            .map(|name| name.to_str())
            .flatten()
            .is_some()
        {
            let request_key = module_request_cache_key(module_path, None);

            // Check if module is cached; if so, don't do anything
            if self.get_cached_module(request_key.as_str()).is_some() {
                return Ok(());
            }

            let module_resolver = Rc::clone(&self.module_resolver);

            // Resolve and load the module
            match module_resolver.try_resolve(self, module_path, None) {
                Ok(module_program) => {
                    if let Some(resolved_key) = module_resolved_cache_key(&module_program) {
                        if let Some(cached_module) = self.get_cached_module(resolved_key.as_str()) {
                            self.cache_module(request_key.as_str(), cached_module);
                            return Ok(());
                        }
                    }

                    match self.run(&module_program) {
                        Ok(module) => {
                            self.cache_module(request_key.as_str(), module.clone());
                            if let Some(resolved_key) = module_resolved_cache_key(&module_program) {
                                self.cache_module(resolved_key.as_str(), module.clone());
                            }
                            Ok(module)
                        }
                        Err(err) => Err(ModuleLoadError::RuntimeError(Rc::new(err))),
                    }
                }
                Err(err) => Err(ModuleLoadError::ResolveError(err)),
            }?;

            Ok(())
        } else {
            Err(ModuleLoadError::InvalidPath(format!(
                "missing module name from path: '{module_path}'"
            )))
        }
    }

    #[inline]
    pub(crate) fn get_cached_module(&self, cache_key: &str) -> Option<RantyValue> {
        if let Some(RantyValue::Map(module_cache_ref)) = self.get_global(MODULES_CACHE_KEY) {
            if let Some(module) = module_cache_ref.borrow().raw_get(cache_key) {
                return Some(module.clone());
            }
        }
        None
    }

    #[inline]
    pub(crate) fn cache_module(&mut self, cache_key: &str, module: RantyValue) {
        if let Some(RantyValue::Map(module_cache_ref)) = self.get_global(MODULES_CACHE_KEY) {
            module_cache_ref.borrow_mut().raw_set(cache_key, module);
        } else {
            let mut cache = RantyMap::new();
            cache.raw_set(cache_key, module);
            self.set_global(MODULES_CACHE_KEY, cache.into_ranty());
        }
    }
}

/// Provides options for customizing the creation of a `Ranty` instance.
#[derive(Debug, Clone)]
pub struct RantyOptions {
    /// Specifies whether the standard library should be loaded.
    pub use_stdlib: bool,
    /// Enables debug mode, which includes additional debug information in compiled programs and more detailed runtime error data.
    pub debug_mode: bool,
    /// Promotes definitions in a program's root scope to globals instead of discarding them with the root frame.
    ///
    /// This is primarily useful for REPL-style execution where each input should persist definitions for later inputs.
    pub top_level_defs_are_globals: bool,
    /// The initial seed to pass to the RNG. Defaults to 0.
    pub seed: u64,
    /// The number of GC-managed allocations allowed between automatic cycle-collection passes.
    pub gc_allocation_threshold: usize,
}

impl Default for RantyOptions {
    fn default() -> Self {
        Self {
            use_stdlib: true,
            debug_mode: false,
            top_level_defs_are_globals: false,
            seed: 0,
            gc_allocation_threshold: gc::DEFAULT_ALLOCATION_THRESHOLD,
        }
    }
}

/// A compiled Ranty program.
#[derive(Debug)]
pub struct RantyProgram {
    info: Rc<RantyProgramInfo>,
    root: Rc<Sequence>,
}

impl RantyProgram {
    pub(crate) fn new(root: Rc<Sequence>, info: Rc<RantyProgramInfo>) -> Self {
        Self { info, root }
    }

    /// Gets the display name of the program, if any.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.info.name.as_deref()
    }

    /// Gets the path to the program's source file, if any.
    #[inline]
    pub fn path(&self) -> Option<&str> {
        self.info.path.as_deref()
    }

    /// Gets the metadata associated with the program.
    #[inline]
    pub fn info(&self) -> &RantyProgramInfo {
        self.info.as_ref()
    }
}

/// Contains metadata used to identify a loaded program.
#[derive(Debug)]
pub struct RantyProgramInfo {
    path: Option<String>,
    name: Option<String>,
}

impl RantyProgramInfo {
    /// Gets the display name of the program, if any.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Gets tha path to the program's source file, if any.
    #[inline]
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
}

/// Represents error states that can occur when loading a module.
#[derive(Debug)]
pub enum ModuleLoadError {
    /// The specified path was invalid; see attached reason.
    InvalidPath(String),
    /// The module failed to load because it encountered a runtime error during initialization.
    RuntimeError(Rc<RuntimeError>),
    /// The module failed to load because it couldn't be resolved.
    ResolveError(ModuleResolveError),
}

impl Display for ModuleLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleLoadError::InvalidPath(errmsg) => write!(f, "{}", errmsg),
            ModuleLoadError::RuntimeError(err) => {
                write!(f, "runtime error while loading module: {}", err)
            }
            ModuleLoadError::ResolveError(err) => write!(f, "unable to resolve module: {}", err),
        }
    }
}

impl Error for ModuleLoadError {}

/// Represents error states that can occur when registering a data source on a Ranty execution context.
#[derive(Debug)]
pub enum DataSourceRegisterError {
    /// The type ID provided by the data source was invalid.
    InvalidTypeId(String),
    /// A data source with the specified type ID was already registered on the context.
    AlreadyExists(String),
}

impl Display for DataSourceRegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTypeId(id) => write!(f, "the type id '{id}' is invalid"),
            Self::AlreadyExists(id) => write!(
                f,
                "the type id '{id}' was already registered on the context"
            ),
        }
    }
}
