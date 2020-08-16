use std::env;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::process::Command;

use directories::BaseDirs;

pub fn edit_configuration() {
    let config_path = get_config_path();
    let editor_cmd = get_editor();

    let mut cmd_iter = editor_cmd
        .to_str()
        .expect("environment variable contains invalid unicode")
        .split_whitespace();

    let editor = cmd_iter.next().unwrap_or(STD_EDITOR);
    let args: Vec<_> = cmd_iter.collect();

    let command = Command::new(editor).args(args).arg(config_path).status();

    match command {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!(
                    "Error: editor {:?} was not found. Did you set your $EDITOR or $VISUAL \
                    environment variables correctly?",
                    editor
                );
                eprintln!("Full error: {:?}", error);
                std::process::exit(1)
            }
            other_error => panic!("failed to open file: {:?}", other_error),
        },
    };
}


fn get_editor() -> OsString {
    get_editor_internal(env::var_os("VISUAL"), env::var_os("EDITOR"))
}

fn get_editor_internal(visual: Option<OsString>, editor: Option<OsString>) -> OsString {
    let mut editor_name = visual.unwrap_or_else(|| "".into());
    if !editor_name.is_empty() {
        return editor_name;
    }
    editor_name = editor.unwrap_or_else(|| "".into());
    if !editor_name.is_empty() {
        return editor_name;
    }
    STD_EDITOR.into()
}


fn get_config_path() -> OsString {
    BaseDirs::new().expect("couldn't find home directory").into()
}