use std::process::Command;

fn main() {
    let ls = String::from_utf8(
        Command::new("ls")
            .output()
            .expect("failed to run `ls`")
            .stdout,
    )
    .expect("failed to get utf-8 from `ls` output");

    let mut ls: Vec<_> = ls.split("\n").collect();

    ls.remove(ls.len() - 1);
    ls.retain(|i| {
        ![
            "CODE",
            "DIRECTORIES",
            "BINARIES",
            "PICTURES",
            "OTHERS",
            "VIDEOS",
            "THING", // THIS IS THE BINARY FOR THIS CODE
        ]
        .contains(i)
    });

    for i in ls {
        let type_ = String::from_utf8(
            Command::new("file")
                .arg(i)
                .output()
                .expect(format!("failed to run `file {}`", i).as_str())
                .stdout,
        )
        .expect(format!("failed to get utf-8 from `file {}` output", i).as_str());
        let type_ = type_.trim();

        Command::new("mv")
            .arg(i)
            .arg(if type_.contains("directory") {
                "DIRECTORIES"
            } else if type_.contains("JSON") || type_.contains("script") || type_.contains("source")
            {
                "CODE"
            } else if type_.contains("image") {
                "PICTURES"
            } else if type_.contains("ELF") {
                "BINARIES"
            } else if type_.contains("WebM") || type_.contains("Matroska") {
                "VIDEOS"
            } else {
                "OTHERS"
            })
            .spawn()
            .expect("failed to spawn `mv`");
    }
}
