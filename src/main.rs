struct Parse {
    options: ParserOptions,
}

enum EcmaVersion {
    V3,
    V5,
    V6_2015,
    V7_2016,
    V8_2017,
    V10_2019,
    V11_2020,
    V12_2021,
    V13_2022,
    V14_2023,
    Latest,
}

enum SourceType {
    Script,
    Module,
}

struct ParserOptions {
    /// `ecmaVersion` indicates the ECMAScript version to parse. Must be
    /// either 3, 5, 6 (or 2015), 7 (2016), 8 (2017), 9 (2018), 10
    /// (2019), 11 (2020), 12 (2021), 13 (2022), 14 (2023), or `"latest"`
    /// (the latest version the library supports). This influences
    /// support for strict mode, the set of reserved words, and support
    /// for new syntax features.
    /// ecmaVersion: null,
    ecma_version: Option<EcmaVersion>,

    /// `sourceType` indicates the mode the code should be parsed in.
    /// Can be either `"script"` or `"module"`. This influences global
    /// strict mode and parsing of `import` and `export` declarations.
    /// sourceType: "script",
    source_type: Option<SourceType>,

    /// `onInsertedSemicolon` can be a callback that will be called
    /// when a semicolon is automatically inserted. It will be passed
    /// the position of the comma as an offset, and if `locations` is
    /// enabled, it is given the location as a `{line, column}` object
    /// as second argument.
    /// onInsertedSemicolon: null,
    on_inserted_semicolor: Option<String>,

    /// `onTrailingComma` is similar to `onInsertedSemicolon`, but for
    /// trailing commas.
    /// onTrailingComma: null,
    on_trailing_comma: Option<String>,

    /// By default, reserved words are only enforced if ecmaVersion >= 5.
    /// Set `allowReserved` to a boolean value to explicitly turn this on
    /// an off. When this option has the value "never", reserved words
    /// and keywords can also not be used as property names.
    /// allowReserved: null,
    allow_reserved: Option<bool>,

    /// When enabled, a return at the top level is not considered an error.
    /// allowReturnOutsideFunction: false,
    allow_return_outside_function: bool,

    /// When enabled, import/export statements are not constrained to
    /// appearing at the top of the program, and an import.meta expression
    /// in a script isn't considered an error.
    /// allowImportExportEverywhere: false,
    allow_import_export_everywhere: bool,

    /// By default, await identifiers are allowed to appear at the top-level scope only if ecmaVersion >= 2022.
    /// When enabled, await identifiers are allowed to appear at the top-level scope,
    /// but they are still not allowed in non-async functions.
    /// allowAwaitOutsideFunction: null,
    allow_await_outside_function: Option<bool>,

    /// When enabled, super identifiers are not constrained to
    /// appearing in methods and do not raise an error when they appear elsewhere.
    /// allowSuperOutsideMethod: null,
    allow_super_outside_method: Option<bool>,

    /// When enabled, hashbang directive in the beginning of file is
    /// allowed and treated as a line comment. Enabled by default when
    /// `ecmaVersion` >= 2023.
    /// allowHashBang: false,
    allow_hash_bang: bool,

    /// When `locations` is on, `loc` properties holding objects with
    /// `start` and `end` properties in `{line, column}` form (with
    /// line being 1-based and column 0-based) will be attached to the
    /// nodes.
    /// locations: false,
    locations: bool,

    /// A function can be passed as `onToken` option, which will
    /// cause Acorn to call that function with object in the same
    /// format as tokens returned from `tokenizer().getToken()`. Note
    /// that you are not allowed to call the parser from the
    /// callback—that will corrupt its internal state.
    /// onToken: null,
    on_token: Option<String>,

    /// A function can be passed as `onComment` option, which will
    /// cause Acorn to call that function with `(block, text, start,
    /// end)` parameters whenever a comment is skipped. `block` is a
    /// boolean indicating whether this is a block (`/* */`) comment,
    /// `text` is the content of the comment, and `start` and `end` are
    /// character offsets that denote the start and end of the comment.
    /// When the `locations` option is on, two more parameters are
    /// passed, the full `{line, column}` locations of the start and
    /// end of the comments. Note that you are not allowed to call the
    /// parser from the callback—that will corrupt its internal state.
    /// onComment: null,
    on_comment: Option<String>,

    /// Nodes have their start and end characters offsets recorded in
    /// `start` and `end` properties (directly on the node, rather than
    /// the `loc` object, which holds line/column data. To also add a
    /// [semi-standardized][range] `range` property holding a `[start,
    /// end]` array with the same numbers, set the `ranges` option to `true`.
    /// [range]: https://bugzilla.mozilla.org/show_bug.cgi?id=745678
    ranges: bool,

    /// It is possible to parse multiple files into a single AST by
    /// passing the tree produced by parsing the first file as
    /// `program` option in subsequent parses. This will add the
    /// toplevel forms of the parsed file to the `Program` (top) node
    /// of an existing parse tree.
    /// program: null,
    program: Option<String>,

    /// When `locations` is on, you can pass this to record the source
    /// file in every node's `loc` object.
    /// sourceFile: null,
    source_file: Option<String>,

    /// This value, if given, is stored in every node, whether
    /// `locations` is on or off.
    /// directSourceFile: null,
    direct_source_file: Option<String>,

    /// When enabled, parenthesized expressions are represented by
    /// (non-standard) ParenthesizedExpression nodes
    /// preserveParens: false
    preserve_parens: bool,
}

impl ParserOptions {
    fn new() -> Self {
        Self {
            ecma_version: None,
            source_type: None,
            on_inserted_semicolor: None,
            on_trailing_comma: None,
            allow_reserved: None,
            allow_return_outside_function: false,
            allow_import_export_everywhere: false,
            allow_await_outside_function: None,
            allow_super_outside_method: None,
            allow_hash_bang: false,
            locations: false,
            on_token: None,
            on_comment: None,
            ranges: false,
            program: None,
            source_file: None,
            direct_source_file: None,
            preserve_parens: false,
        }
    }
}

impl Parse {
    fn run(input: &str, options: ParserOptions) -> Self {
        Self { options }
    }
}

fn main() {
    let parse = Parse {
        options: ParserOptions::new(),
    };
}
