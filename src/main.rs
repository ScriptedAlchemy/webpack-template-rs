#[macro_use]
extern crate lazy_static;

use regex::{Regex};

const START_LOWERCASE_ALPHABET_CODE: u32 = 'a' as u32;
const START_UPPERCASE_ALPHABET_CODE: u32 = 'A' as u32;
const DELTA_A_TO_Z: u32 = ('z' as u32 - START_LOWERCASE_ALPHABET_CODE + 1) as u32;
const NUMBER_OF_IDENTIFIER_START_CHARS: u32 = DELTA_A_TO_Z * 2 + 2; // a-z A-Z _ $
const NUMBER_OF_IDENTIFIER_CONTINUATION_CHARS: u32 = NUMBER_OF_IDENTIFIER_START_CHARS + 10; // a-z A-Z _ $ 0-9

lazy_static! {
    static ref FUNCTION_CONTENT_REGEX: Regex = Regex::new(r"^function\s?\(\)\s?\{\r?\n?|\r?\n?\}(\s*;)?\s*$").unwrap();
    static ref INDENT_MULTILINE_REGEX: Regex = Regex::new(r"^\t").unwrap();
    static ref LINE_SEPARATOR_REGEX: Regex = Regex::new(r"\r?\n").unwrap();
    static ref IDENTIFIER_NAME_REPLACE_REGEX: Regex = Regex::new(r"^([^a-zA-Z$_])").unwrap();
    static ref IDENTIFIER_ALPHA_NUMERIC_NAME_REPLACE_REGEX: Regex = Regex::new(r"[^a-zA-Z0-9$]+").unwrap();
    static ref COMMENT_END_REGEX: Regex = Regex::new(r"\*/").unwrap();
    static ref PATH_NAME_NORMALIZE_REPLACE_REGEX: Regex = Regex::new(r"[^a-zA-Z0-9_!§$()=\-^°]+").unwrap();
    static ref MATCH_PADDED_HYPHENS_REPLACE_REGEX: Regex = Regex::new(r#"^-|-+$"#).unwrap();
}

pub struct Template {}

impl Template {
    pub fn get_function_content<F: FnOnce() -> String>(r#fn_string: F) -> String {
        let fn_str = fn_string();
        LINE_SEPARATOR_REGEX.replace_all(&INDENT_MULTILINE_REGEX.replace_all(&FUNCTION_CONTENT_REGEX.replace_all(&fn_str, ""), ""), "\n").to_string()
    }
    pub  fn to_identifier(s: &str) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        let id_replace = IDENTIFIER_NAME_REPLACE_REGEX.replace_all(s, "_$1");
        IDENTIFIER_ALPHA_NUMERIC_NAME_REPLACE_REGEX.replace_all(&id_replace, "_").to_string()
    }

    pub fn to_commemnt(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }
        format!("/*! {} */", COMMENT_END_REGEX.replace_all(s, "* /"))
    }

    pub fn to_normal_commemnt(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }
        format!("/* {} */", COMMENT_END_REGEX.replace_all(s, "* /"))
    }

    pub fn to_path(str: &str) -> String {
        if str.is_empty() {
            return "".to_string();
        }
        let normalized = PATH_NAME_NORMALIZE_REPLACE_REGEX.replace_all(str, "-");
        let bundle_safe_path = MATCH_PADDED_HYPHENS_REPLACE_REGEX.replace_all(&normalized, "");
        return bundle_safe_path.to_string();
    }

    pub fn number_to_identifier(n: u32) -> String {
        if n >= NUMBER_OF_IDENTIFIER_START_CHARS {
            // use multiple letters
            return format!(
                "{}{}",
                Template::number_to_identifier(n % NUMBER_OF_IDENTIFIER_START_CHARS),
                Template::number_to_identifier_continuation(n / NUMBER_OF_IDENTIFIER_START_CHARS)
            );
        }

        // lower case
        if n < DELTA_A_TO_Z {
            return std::char::from_u32(START_LOWERCASE_ALPHABET_CODE + n).unwrap().to_string();
        }
        let mut n = n - DELTA_A_TO_Z;

        // upper case
        if n < DELTA_A_TO_Z {
            return std::char::from_u32(START_UPPERCASE_ALPHABET_CODE + n).unwrap().to_string();
        }
        n -= DELTA_A_TO_Z;

        if n == DELTA_A_TO_Z {
            return "_".to_string();
        }
        return "$".to_string();
    }

     pub fn number_to_identifier_continuation(n: u32) -> String {
        if n >= NUMBER_OF_IDENTIFIER_CONTINUATION_CHARS {
            // use multiple letters
            return format!(
                "{}{}",
                Template::number_to_identifier_continuation(n % NUMBER_OF_IDENTIFIER_CONTINUATION_CHARS),
                Template::number_to_identifier_continuation(n / NUMBER_OF_IDENTIFIER_CONTINUATION_CHARS)
            );
        }

        // lower case
        if n < DELTA_A_TO_Z {
            return std::char::from_u32(START_LOWERCASE_ALPHABET_CODE + n).unwrap().to_string();
        }
        let mut n = n - DELTA_A_TO_Z;

        // upper case
        if n < DELTA_A_TO_Z {
            return std::char::from_u32(START_UPPERCASE_ALPHABET_CODE + n).unwrap().to_string();
        }
        n -= DELTA_A_TO_Z;

        // numbers
        if n < 10 {
            return n.to_string();
        }

        if n == 10 {
            return "_".to_string();
        }
        return "$".to_string();
    }

    pub fn indent(s: &str) -> String {
        if let Some(lines) = s.lines().collect::<Vec<&str>>().split_last() {
            let (last_line, rest_lines) = lines;
            let mut indented = rest_lines
                .iter()
                .map(|line| format!("\t{}\n", line))
                .collect::<String>();
            if !rest_lines.is_empty() {
                indented.push('\n');
            }
            format!("{}{}", indented, last_line.trim())
        } else {
            String::new()
        }
    }

    pub fn prefix(s: impl ToString, prefix: &str) -> String {
        let str = s.to_string().trim();
        if str.is_empty() {
            return "".to_string();
        }
        let ind = if str.starts_with('\n') { "" } else { prefix };
        let replace = str.replace("\n", &format!("\n{}", prefix))
            .replace(&format!("\n{}", prefix), "\n")
            .insert_str(0, ind);

       return replace;

    }
}

fn main() {
    let fn_string = "function() {
        function() {
            return 'thing';
        }
        return 'test';
    }
    ".to_string();
    let content = Template::get_function_content(|| fn_string);
    println!("{}", content);

    let identifier = Template::to_identifier("123test-identifier@thing1234");
    println!("{}", identifier);

    let comment = Template::to_commemnt("test comment");
    println!("{}", comment);

    let comment = Template::to_normal_commemnt("test normal comment");
    println!("{}", comment);

    let path = Template::to_path("test/path/thing");
    println!("{}", path);

    let nti = Template::number_to_identifier(1234);
    println!("{}", nti);

    let indent = Template::indent("
    test
    indent
    ");
    println!("{}", indent);
}


