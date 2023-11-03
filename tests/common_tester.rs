#![cfg(feature = "test_bin")]

/// example_name: "rerooting_abc222-g"
/// in_out_filenames: ["sample1", "sample2"]
use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};

pub fn test_bin(example_name: &str, in_out_filenames: Vec<&str>) {
    let input_dir = format!("examples/{}/in", example_name);
    let output_dir = format!("examples/{}/out", example_name);
    let bin_path = format!("./target/release/examples/{}", example_name);
    for filename in in_out_filenames {
        let input_path = format!("{}/{}.txt", input_dir, filename);
        let output_path = format!("{}/{}.txt", output_dir, filename);
        let input_data = fs::read(&input_path)
            .expect(format!("Failed to read input file: {}", &input_path).as_str());
        let expected_output = fs::read_to_string(&output_path)
            .expect(format!("Failed to read output file: {}", &output_path).as_str());
        let mut child = Command::new(&bin_path)
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
