//! Prints common `richls` command examples.
//!
//! Run with:
//!
//! ```bash
//! cargo run --example basic_usage
//! ```

struct UsageExample {
    description: &'static str,
    command: &'static [&'static str],
}

fn main() {
    for example in examples() {
        println!("# {}", example.description);
        println!("{}", shell_command(example.command));
        println!();
    }
}

fn examples() -> [UsageExample; 6] {
    [
        UsageExample {
            description: "カレントディレクトリを表示",
            command: &["richls"],
        },
        UsageExample {
            description: "隠しファイルを含めて詳細表示",
            command: &["richls", "-la"],
        },
        UsageExample {
            description: "PDFメタデータTitleを表示",
            command: &["richls", "--pdf-title", "./papers"],
        },
        UsageExample {
            description: "ignoreファイルを考慮して更新日時順に表示",
            command: &[
                "richls",
                "-l",
                "--respect-ignore",
                "--sort",
                "mtime",
                "documents/",
            ],
        },
        UsageExample {
            description: "削除候補になりそうな通常ファイルを表示",
            command: &["richls", "--clean-suggest"],
        },
        UsageExample {
            description: "シェル補完ファイルを生成",
            command: &["richls", "--complete"],
        },
    ]
}

fn shell_command(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| quote_shell_part(part))
        .collect::<Vec<_>>()
        .join(" ")
}

fn quote_shell_part(part: &str) -> String {
    if part.chars().any(char::is_whitespace) {
        format!("'{}'", part.replace('\'', "'\\''"))
    } else {
        part.to_string()
    }
}
