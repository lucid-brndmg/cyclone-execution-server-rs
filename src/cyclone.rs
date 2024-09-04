use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::{env, fs};
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::{InputStream, Parser};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::errors::ANTLRError;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Visitable};
use tracing::{debug, error};
use crate::parse::cyclonelexer::CycloneLexer;
use crate::parse::cycloneparser::{CycloneParser, CycloneParserContextType, OptionNameContext};
use crate::parse::cycloneparservisitor::CycloneParserVisitor;

// const CYCLONE_ENTRY_FILE: &str = "cyclone.jar";
const CYCLONE_ENTRY_EXTENSION: &str = "jar";
const CYCLONE_VERSION_KEYWORD: &str = "cyclone";

pub fn locate_cyclone_entry(specified_location: Option<&str>, is_allow_dialog: bool, executable_file: &str) -> Option<PathBuf> {
    let real_location = match specified_location {
        Some(location) => {
            let p = PathBuf::from(location);
            if p.is_dir() {
                match fs::read_dir(p) {
                    Ok(dir) => {
                        let mut r = None;
                        for entry in dir {
                            if let Ok(u) = entry {
                                if u.file_name() == executable_file {
                                    r = Some(u.path());
                                    break
                                }
                            }
                        }
                        r
                    }
                    Err(_) => None
                }
            } else if p.is_file() {
                Some(p)
            } else {
                None
            }
        }
        None => None
    };

    match real_location {
        Some(l) => Some(l),
        None => {
            if is_allow_dialog {
                FileDialog::new()
                    .add_filter(executable_file, &[CYCLONE_ENTRY_EXTENSION])
                    .set_file_name(executable_file)
                    .set_title("Select a Cyclone instance")
                    .pick_file()
            } else {
                None
            }
        }
    }
}

pub fn fetch_cyclone_version(location: &Path) -> Option<(String, String)> {
    match Command::new("java")
        .arg("-jar")
        .arg(location)
        .arg("--version")
        .output() {
        Ok(p) => {
            let out = String::from_utf8_lossy(&p.stdout);
            if !out.is_empty() && out.to_ascii_lowercase().contains(CYCLONE_VERSION_KEYWORD) {
                Some((out.to_string(), String::from_utf8_lossy(&p.stderr).to_string()))
            } else {
                None
            }
        }
        Err(e) => {
            error!("Error fetching Cyclone version: {}", e);
            None
        }
    }
}

pub fn prepare_cyclone_env(path_to_cyclone_executable: &Path) -> Result<(), env::JoinPathsError> {
    if let Some(path) = env::var_os("PATH") {

        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        match path_to_cyclone_executable.parent() {
            Some(p) => {
                let dir = PathBuf::from(p);
                paths.push(dir);
                let new_path = env::join_paths(paths)?;
                unsafe {env::set_var("PATH", &new_path);}
            }
            None => {}
        }

    }

    Ok(())
}

struct CycloneValidationVisitor <'opt, T: AsRef<str>> {
    invalid_options: &'opt [T],
    is_invalid: bool
}

impl<'node, T: AsRef<str>> ParseTreeVisitor<'node, CycloneParserContextType> for CycloneValidationVisitor<'node, T> {}

impl <'node, T: AsRef<str>> CycloneParserVisitor<'node> for CycloneValidationVisitor<'node, T> {
    fn visit_optionName(&mut self, ctx: &OptionNameContext<'node>) {
        let curr_opt = ctx.get_text();
        if !self.is_invalid {
            let opts = self.invalid_options;
            for opt in opts {
                if opt.as_ref() == curr_opt {
                    self.is_invalid = true;
                    debug!("Invalid option {}", curr_opt);
                    return;
                }
            }
        }

    }
}
struct CycloneErrorListener {
    is_invalid: Rc<RefCell<bool>>
}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for CycloneErrorListener {
    fn syntax_error(&self, _recognizer: &T, _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>, _line: isize, _column: isize, _msg: &str, _error: Option<&ANTLRError>) {
        debug!("Syntax error: {}", _msg);
        let mut invalid_flag = self.is_invalid.borrow_mut();
        *invalid_flag = true;
    }
}

#[derive(Debug)]
pub enum ValidationResult {
    Valid,
    InvalidOption,
    SyntaxError
}

pub fn is_valid_program<T: AsRef<str>>(program: &str, disabled_options: &[T]) -> ValidationResult {
    let program_trim = program.trim();
    if program_trim.is_empty() {
        debug!("Received empty Cyclone spec");
        return ValidationResult::SyntaxError
    }

    let flag_syntax_error = Rc::new(RefCell::new(false));

    let mut lexer = CycloneLexer::new(InputStream::new(program_trim));

    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(CycloneErrorListener {
        is_invalid: flag_syntax_error.clone()
    }));

    let is_lexing_error = flag_syntax_error.as_ref().clone().into_inner();
    if is_lexing_error {
        return ValidationResult::SyntaxError
    }

    let token_source = CommonTokenStream::new(lexer);
    let mut parser = CycloneParser::new(token_source);

    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(CycloneErrorListener {
        is_invalid: flag_syntax_error.clone()
    }));
    
    let result = parser.program();
    let is_syntax_error = flag_syntax_error.as_ref().clone().into_inner();
    if is_syntax_error {
        return ValidationResult::SyntaxError
    }

    match result {
        Err(_) => ValidationResult::SyntaxError,
        Ok(parsed) => {
            if parsed.exception.is_none() {
                if disabled_options.is_empty() {
                    ValidationResult::Valid
                } else {
                    let mut visitor = CycloneValidationVisitor {
                        invalid_options: disabled_options,
                        is_invalid: false
                    };
                    parsed.accept(&mut visitor);
                    if visitor.is_invalid {
                        ValidationResult::InvalidOption
                    } else { ValidationResult::Valid }
                }
            } else {
                ValidationResult::SyntaxError
            }
        }
    }
}