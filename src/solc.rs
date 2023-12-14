use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

/// Builds the Solidity source in `path_to_source_sol`.
/// Returns the path to the build output directory.
///
/// For each each contract found in the source code, solc creates a file:
/// - `contract_name.bin`: The code blob.
/// Where `contract_name` is equal to the name of that contract
///
pub fn build_contract<P>(path_to_source_sol: P) -> anyhow::Result<PathBuf>
    where
        P: AsRef<Path> + Copy,
{
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let bin_path = PathBuf::from(manifest_dir).join("bin").join("solc");

    let out_dir = path_to_source_sol
        .as_ref()
        .parent()
        .expect("source file is not in root dir");

    match Command::new(&bin_path)
        .arg("--bin")
        .arg("-o")
        .arg(out_dir)
        .arg(path_to_source_sol.as_ref())
        .output()
    {
        Ok(output) if output.status.success() => Ok(out_dir.to_path_buf()),
        Ok(output) => Err(anyhow::anyhow!("Failed to compile contract:\n {output:?}")),
        Err(msg) => Err(anyhow::anyhow!("Failed to execute {bin_path:?}: {msg:?}")),
    }
}