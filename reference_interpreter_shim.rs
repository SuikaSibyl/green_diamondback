use std::{path::Path, process::Command};

pub(crate) enum TestKind {
    Success,
    RuntimeError,
    StaticError,
}

#[macro_export]
macro_rules! success_tests {
    ($($tt:tt)*) => { $crate::tests!(Success => $($tt)*); }
}

#[macro_export]
macro_rules! runtime_error_tests {
    ($($tt:tt)*) => { $crate::tests!(RuntimeError => $($tt)*); }
}

#[macro_export]
macro_rules! static_error_tests {
    ($($tt:tt)*) => { $crate::tests!(StaticError => $($tt)*); }
}

#[macro_export]
macro_rules! tests {
    ($kind:ident =>
        $(
            {
                name: $name:ident,
                file: $file:literal,
                $(input: $input:literal,)?
                expected: $expected:literal $(,)?
                $(" $(tt:$tt)* ")?
            }
        ),*
        $(,)?
    ) => {
        $(
            #[test]
            fn $name() {
                #[allow(unused_assignments, unused_mut)]
                let mut input = None;
                $(input = Some($input);)?
                let kind = $crate::infra::TestKind::$kind;
                $crate::infra::run_test(stringify!($name), $file, input, $expected, kind);
            }
        )*
    };
}

pub(crate) fn run_test(
    name: &str,
    file: &str,
    input: Option<&str>,
    expected: &str,
    kind: TestKind,
) {
    let file = Path::new("tests").join(file);
    match kind {
        TestKind::Success => run_success_test(name, &file, expected, input),
        TestKind::RuntimeError => run_runtime_error_test(name, &file, expected, input),
        TestKind::StaticError => run_static_error_test(name, &file, expected),
    }
}

fn run_success_test(_name: &str, file: &Path, expected: &str, input: Option<&str>) {
    let actual_output = run_interpreter(file, input).unwrap();
    diff(expected, &actual_output);
}

fn run_runtime_error_test(_name: &str, file: &Path, _expected: &str, input: Option<&str>) {
    let err = run_interpreter(file, input).unwrap_err();
    assert!(err.contains("Type") || err.contains("Overflow"));
}

fn run_static_error_test(_name: &str, file: &Path, expected: &str) {
    let err = run_interpreter(file, None).unwrap_err();
    assert!(err.contains("Syntax") || err.contains("BadProgram"));
    if expected == "Invalid" {
        assert!(err.contains("Syntax"));
    }
}

fn run_interpreter(snek_file: &Path, input: Option<&str>) -> Result<String, String> {
    let output = Command::new("./target/reference-interpreter")
        .arg(snek_file)
        .arg(input.unwrap_or("false"))
        .output()
        .expect("could not run the interpreter");
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}

fn diff(expected: &str, found: &str) {
    let expected = expected.trim();

    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = found.lines().collect();
    if expected_lines != actual_lines {
        eprintln!(
            "output differed!\n{}",
            prettydiff::diff_lines(found, expected)
        );
        panic!("test failed");
    }
}
