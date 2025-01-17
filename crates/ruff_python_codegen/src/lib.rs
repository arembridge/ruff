mod generator;
mod stylist;

pub use generator::Generator;
use ruff_source_file::Locator;
use rustpython_ast::Suite;
use rustpython_parser::{lexer, Mode, Parse, ParseError};
pub use stylist::{Quote, Stylist};

/// Run round-trip source code generation on a given Python code.
pub fn round_trip(code: &str, source_path: &str) -> Result<String, ParseError> {
    let locator = Locator::new(code);
    let python_ast = Suite::parse(code, source_path)?;
    let tokens: Vec<_> = lexer::lex(code, Mode::Module).collect();
    let stylist = Stylist::from_tokens(&tokens, &locator);
    let mut generator: Generator = (&stylist).into();
    generator.unparse_suite(&python_ast);
    Ok(generator.generate())
}
