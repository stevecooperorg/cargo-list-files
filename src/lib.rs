use cargo::core::Workspace;
use cargo::sources::PathSource;
use cargo::Config;
use std::path::*;

pub fn list_files(cargo_toml_location: &Path) -> Vec<PathBuf> {
    let config = Config::default().unwrap();
    let ws = Workspace::new(cargo_toml_location, &config).unwrap();

    let mut result = vec![];

    for pkg in ws.members() {
        let root = pkg.root();
        let src_id = pkg.package_id().source_id();
        let config = ws.config();
        let mut src = PathSource::new(root, src_id, config);
        src.update().unwrap();

        let mut src_files = src.list_files(pkg).unwrap();
        result.append(&mut src_files);
    }

    result
}
