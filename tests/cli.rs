use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

use grrs::find_matches;

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _ = find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    // 一時的なファイルを作成する
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    // ファイルにテキストを書き込む
    file.write_str("A test\nActual content\nMore content\nAnother test")?;
    // `grrs` という名前のローカルカーゴバイナリを実行するためのコマンドをセットアップ
    let mut cmd = Command::cargo_bin("grrs")?;
    // コマンドに "test" と一時的なファイルのパスを引数として追加する
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}
