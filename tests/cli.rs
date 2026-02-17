use assert_cmd::cargo_bin_cmd;

#[test]
fn missing_prd_json_exits_with_error() {
    let temp = std::env::temp_dir().join("jodex_test_no_prd");
    std::fs::create_dir_all(&temp).unwrap();

    // Ensure no prd.json exists in the temp directory
    let prd_path = temp.join("prd.json");
    if prd_path.exists() {
        std::fs::remove_file(&prd_path).unwrap();
    }

    cargo_bin_cmd!("jodex")
        .current_dir(&temp)
        .assert()
        .failure()
        .code(1)
        .stderr(predicates::str::contains("prd.json"));
}

#[test]
fn help_displays_program_description() {
    cargo_bin_cmd!("jodex")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("autonomous agent loop"));
}
