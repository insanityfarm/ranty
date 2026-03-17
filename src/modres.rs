use super::*;
use std::{env, ffi::OsStr, fmt::Debug, io::ErrorKind, path::PathBuf};

/// Result type used by the module loader.
pub type ModuleResolveResult = Result<RantyProgram, ModuleResolveError>;

/// Represents the features required for a module resolver.
///
/// A module resolver only resolves the `RantyProgram` that the final module object is loaded from.
/// This is designed as such to ensure that module loading is limited to the maximum call stack size of the requesting program.
pub trait ModuleResolver: Debug {
    fn try_resolve(
        &self,
        context: &mut Ranty,
        module_path: &str,
        dependant: Option<&RantyProgramInfo>,
    ) -> ModuleResolveResult;
}

/// The default filesystem-based module resolver.
///
/// ### Resolution strategy
/// This resolver uses the following strategy to locate module files:
/// 1. If triggered by a program, the program's containing directory is searched first.
/// 1. The directory specified in `local_modules_path` is searched next. If not specified, uses the host application's current working directory.
/// 1. If `enable_global_modules` is set to `true`, the global modules path is searched.
#[derive(Debug)]
pub struct DefaultModuleResolver {
    /// Enables loading modules from RANTY_MODULES_PATH.
    pub enable_global_modules: bool,
    /// Specifies a preferred module loading path with higher precedence than the global module path.
    /// If not specified, looks in the current working directory.
    pub local_modules_path: Option<String>,
}

impl DefaultModuleResolver {
    /// The name of the environment variable that used to provide the global modules path.
    pub const ENV_MODULES_PATH_KEY: &'static str = "RANTY_MODULES_PATH";
}

impl Default for DefaultModuleResolver {
    fn default() -> Self {
        Self {
            enable_global_modules: true,
            local_modules_path: None,
        }
    }
}

impl ModuleResolver for DefaultModuleResolver {
    fn try_resolve(
        &self,
        context: &mut Ranty,
        module_path: &str,
        dependant: Option<&RantyProgramInfo>,
    ) -> ModuleResolveResult {
        // Try to find module path that exists
        if let Some(full_module_path) = self.find_module_path(module_path, dependant) {
            let mut errors = vec![];
            let compile_result = context.compile_file(full_module_path, &mut errors);
            match compile_result {
                Ok(module) => Ok(module),
                Err(err) => Err(ModuleResolveError {
                    name: module_path.to_owned(),
                    reason: match err {
                        CompilerError::SyntaxError => {
                            ModuleResolveErrorReason::CompileFailed(errors)
                        }
                        CompilerError::IOError(ioerr) => match ioerr {
                            IOErrorKind::NotFound => ModuleResolveErrorReason::NotFound,
                            _ => ModuleResolveErrorReason::FileIOError(ioerr),
                        },
                    },
                }),
            }
        } else {
            Err(ModuleResolveError {
                name: module_path.to_owned(),
                reason: ModuleResolveErrorReason::NotFound,
            })
        }
    }
}

impl DefaultModuleResolver {
    fn module_candidates(module_path: &str) -> Vec<PathBuf> {
        let module_path =
            PathBuf::from(module_path.replace("/", &String::from(std::path::MAIN_SEPARATOR)));

        if module_path.extension().and_then(OsStr::to_str).is_some() {
            vec![module_path]
        } else {
            RANTY_SUPPORTED_FILE_EXTENSIONS
                .iter()
                .map(|extension| module_path.with_extension(extension))
                .collect()
        }
    }

    #[inline]
    fn find_module_path(
        &self,
        module_path: &str,
        dependant: Option<&RantyProgramInfo>,
    ) -> Option<PathBuf> {
        let module_candidates = Self::module_candidates(module_path);

        macro_rules! search_for_module {
            ($path:expr) => {
                let path = $path;
                for module_candidate in &module_candidates {
                    // Construct full path to module
                    if let Ok(full_module_path) = path.join(module_candidate).canonicalize() {
                        // Verify file is still in modules directory and it exists
                        if full_module_path.starts_with(&path) && full_module_path.exists() {
                            return Some(full_module_path);
                        }
                    }
                }
            };
        }

        // Search path of dependant running program
        if let Some(dependant_path) = dependant.map(|d| d.path.as_deref()) {
            if let Some(program_path) = dependant_path
                .map(PathBuf::from)
                .as_deref()
                .and_then(|p| p.parent())
            {
                search_for_module!(program_path);
            }
        }

        // Search local modules path
        if let Some(local_modules_path) = self
            .local_modules_path
            .as_ref()
            .map(PathBuf::from)
            .or_else(|| env::current_dir().ok())
            .and_then(|p| p.canonicalize().ok())
        {
            search_for_module!(local_modules_path);
        }

        // Check global modules, if enabled
        if self.enable_global_modules {
            if let Some(global_modules_path) = env::var_os(Self::ENV_MODULES_PATH_KEY)
                .map(PathBuf::from)
                .and_then(|p| p.canonicalize().ok())
            {
                search_for_module!(global_modules_path);
            }
        }

        None
    }
}

/// Stub module resolver that completely disables modules.
///
/// All calls to `try_resolve` on this resolver will return a "not found" error.
#[derive(Debug)]
pub struct NoModuleResolver;

impl ModuleResolver for NoModuleResolver {
    fn try_resolve(
        &self,
        _context: &mut Ranty,
        module_path: &str,
        _dependant: Option<&RantyProgramInfo>,
    ) -> ModuleResolveResult {
        Err(ModuleResolveError {
            name: module_path.to_owned(),
            reason: ModuleResolveErrorReason::NotFound,
        })
    }
}

/// Represents an error that occurred when attempting to load a Ranty module.
#[derive(Debug)]
pub struct ModuleResolveError {
    pub name: String,
    pub reason: ModuleResolveErrorReason,
}

impl Error for ModuleResolveError {}

impl ModuleResolveError {
    /// Gets the name of the module that failed to load.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the reason for the module load failure.
    #[inline]
    pub fn reason(&self) -> &ModuleResolveErrorReason {
        &self.reason
    }
}

/// Represents the reason for which a Ranty module failed to load.
#[derive(Debug)]
pub enum ModuleResolveErrorReason {
    /// The module was not found.
    NotFound,
    /// The module could not be compiled.
    CompileFailed(Vec<CompilerMessage>),
    /// The module could not load due to a file I/O error.
    FileIOError(ErrorKind),
}

impl Display for ModuleResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reason() {
            ModuleResolveErrorReason::NotFound => write!(f, "module '{}' not found", self.name()),
            ModuleResolveErrorReason::CompileFailed(msgs) => write!(
                f,
                "module '{}' failed to compile: {}",
                self.name(),
                msgs.iter().fold(String::new(), |mut acc, msg| {
                    acc.push_str(&format!("[{}] {}\n", msg.severity(), msg.message()));
                    acc
                })
            ),
            ModuleResolveErrorReason::FileIOError(ioerr) => {
                write!(f, "file I/O error ({:?})", ioerr)
            }
        }
    }
}

impl IntoRuntimeResult<RantyProgram> for ModuleResolveResult {
    fn into_runtime_result(self) -> RuntimeResult<RantyProgram> {
        self.map_err(|err| RuntimeError {
            error_type: RuntimeErrorType::ModuleError(err),
            description: None,
            stack_trace: None,
        })
    }
}
