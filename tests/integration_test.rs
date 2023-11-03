#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::Write,
        process::{Command, Stdio},
    };

    #[test]
    fn test_rerooting_abc222_g() {
        let input_data = fs::read("examples/rerooting_abc222-g/in/sample1.txt")
            .expect("Failed to read input file");
        let expected_output = fs::read_to_string("examples/rerooting_abc222-g/out/sample1.txt")
            .expect("Failed to read expected output file");

        let mut child = Command::new("./target/release/examples/rerooting_abc222-g")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start child process");

        // 標準入力にデータを書き込む
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(&input_data)
            .expect("Failed to write to stdin");

        // 子プロセスが終了するのを待ちます。
        let output = child
            .wait_with_output()
            .expect("Failed to wait on child process");
        let output_str = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");

        assert_eq!(output_str, expected_output);
    }
}
