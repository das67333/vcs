pub fn run() -> String {
    format!(
        "commit 2f3417b885724c3c5a0a383827080a9393ce69e9 (master)
Date:   Tue Oct 26 12:49:27 2021 +0100
Message: add dt_to_ms_timestamp
Changes:
\tmodified path/to/modified/file.rs
\tmodified path/to/modified/file2.rs
\tadded path/to/new/file.rs

commit 5ab279df6c80b139290af6ce60939b8fdfba242c
Date:   Tue Oct 26 12:36:46 2021 +0100
Message: Merged branch branch_with_feature.                                              
Changes:
\tmodified lok/kek.txt
\tmodified move/to/london.txt
\tadded new/rust/course.rs
\tadded new/rust/project.rs

commit ffc5ba1b18669c75d341e6be22c3f685237831b5
Date:   Tue Oct 26 14:10:03 2021 +0300
Message: Initial commit
No changes"
    )
}
