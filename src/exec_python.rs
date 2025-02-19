use anyhow;
use regex::Regex;
use rustpython::vm::Settings;
use rustpython::InterpreterConfig;
use rustpython_vm as vm;

pub fn run_python_capture(code: &str) -> anyhow::Result<String, String> {
    let interpreter = InterpreterConfig::new().init_stdlib().interpreter();
    interpreter.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        let code_with_redirect_and_output = format!(
            "import io\nimport sys\noutput = io.StringIO()\nsys.stdout = output\n{}\ncaptured_output = output.getvalue()",
            code,
        );

        let code_obj = vm
            .compile(
                &code_with_redirect_and_output,
                vm::compiler::Mode::Exec,
                "<embedded>".to_owned(),
            )
            .map_err(|err| format!("Compilation error: {}", err))?;

       match  vm.run_code_obj(code_obj, scope.clone())
          {
            Ok(_) => {
                match scope.globals.get_item("captured_output", vm) {
                    Ok(res) => match res.downcast_ref::<vm::builtins::PyStr>() {
                        Some(py_str) =>                   Ok(py_str.as_str().to_string()),
                        None=>                     Err("res is not a string".to_string())
                    },
                Err(_) => Err("error getting captured_output".to_string()),

                }
            },
            Err(e) => {
                let error_message = if let Some(args) = e.args().as_slice().first() {
                    args.downcast_ref::<vm::builtins::PyStr>()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "Unknown error".to_string())
                } else {
                    "No error message available".to_string()
                };
                Err(format!("Code execution error message: {}", error_message))
            }
        }
    })
}

pub fn run_python(code: &str) -> anyhow::Result<String, String> {
    let interpreter = InterpreterConfig::new().init_stdlib().interpreter();
    interpreter.enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let code_obj = vm
            .compile(code, vm::compiler::Mode::Exec, "<embedded>".to_owned())
            .map_err(|err| format!("Compilation error: {}", err))?;

        let result = vm.run_code_obj(code_obj, scope);

        match result {
            Ok(output) => {
                let output_str = output
                    .downcast_ref::<vm::builtins::PyStr>()
                    .ok_or_else(|| "".to_string())?
                    .to_string();
                Ok(format!("Code executed, output: {}", output_str))
            }
            Err(e) => {
                let error_message = if let Some(args) = e.args().as_slice().first() {
                    args.downcast_ref::<vm::builtins::PyStr>()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "Unknown error".to_string())
                } else {
                    "No error message available".to_string()
                };
                Err(format!("Code execution error message: {}", error_message))
            }
        }
    })
}

pub fn run_python_string(code: &str) -> anyhow::Result<String, String> {
    let interpreter = InterpreterConfig::new().init_stdlib().interpreter();
    interpreter.enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let result = vm.run_code_string(scope, code, "<...>".to_string());

        match result {
            Ok(output) => {
                let output_str = output
                    .downcast_ref::<vm::builtins::PyStr>()
                    .ok_or_else(|| "Code executed, failed due to internal error".to_string())?
                    .to_string();
                Ok(format!("Code executed, output: {}", output_str))
            }
            Err(e) => {
                let error_message = if let Some(args) = e.args().as_slice().first() {
                    args.downcast_ref::<vm::builtins::PyStr>()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "Unknown error".to_string())
                } else {
                    "No error message available".to_string()
                };
                Err(format!("Code execution error message: {}", error_message))
            }
        }
    })
}

pub fn run_python_vm(code: &str) {
    let settings = Settings::default();
    let settings = Settings::with_path(settings, "/Users/jichen/.cargo/bin/rustpython".to_owned());
    // let settings = Settings::with_path(
    //     settings,
    //     "/Users/jichen/Downloads/RustPython-0.3.1/pylib/Lib/".to_owned(),
    // );

    vm::Interpreter::with_init(settings, |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
        vm.add_frozen(rustpython_vm::py_freeze!(
            dir = "/Users/jichen/Downloads/RustPython-0.3.1/pylib/Lib/"
        ));
    })
    .enter(|vm| {
        vm.run_code_string(vm.new_scope_with_builtins(), code, "<...>".to_owned());
    });
}

pub fn run_python_func(func_path: &str) -> anyhow::Result<String, String> {
    match std::process::Command::new("/Users/jichen/.cargo/bin/rustpython")
        .arg(func_path)
        .output()
    {
        Ok(out) => {
            if !out.stdout.is_empty() {
                Ok(format!(
                    "Output: {}",
                    String::from_utf8(out.stdout).unwrap()
                ))
            } else {
                Err("empty result".to_string())
            }
        }

        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}

pub fn extract_code(text: &str) -> String {
    let multi_line_pattern = r"```python(.*?)```";
    let mut program = String::new();

    let multi_line_regex = Regex::new(multi_line_pattern).unwrap();
    for cap in multi_line_regex.captures_iter(text) {
        let code = cap.get(1).unwrap().as_str().trim().to_string();
        program.push_str(&code);
    }

    program
}

pub fn extract_code_blocks(
    text: &str,
    detect_single_line_code: bool,
) -> Vec<(Option<String>, String)> {
    // Adjust regex pattern to handle both Unix and Windows line endings and optional language specifier
    let multi_line_pattern = r"```[ \t]*(\w+)?[ \t]*\r?\n(.*?)\r?\n[ \t]*```";
    let single_line_pattern = r"`([^`]+)`";
    let mut results: Vec<(Option<String>, String)> = Vec::new();

    let multi_line_regex = Regex::new(multi_line_pattern).unwrap();
    for cap in multi_line_regex.captures_iter(text) {
        let language = cap
            .get(1)
            .map_or(None, |m| Some(m.as_str().trim().to_string()));
        let code = cap.get(2).unwrap().as_str().trim().to_string();
        results.push((language.clone(), code.clone()));
        // println!("Matched multi-line code block: Language: {:?}, Code: {}", language, code);
    }

    if detect_single_line_code {
        let single_line_regex = Regex::new(single_line_pattern).unwrap();
        for cap in single_line_regex.captures_iter(text) {
            results.push((None, cap.get(1).unwrap().as_str().trim().to_string()));
            // println!("Matched single-line code: {}", cap.get(1).unwrap().as_str().trim());
        }
    }

    results
}

// export DYLD_LIBRARY_PATH=/Users/jichen/miniconda3/lib:$DYLD_LIBRARY_PATH
// export PYO3_PYTHON=/Users/jichen/miniconda3/bin/python
// export DYLD_LIBRARY_PATH=/Users/jichen/miniconda3/lib
