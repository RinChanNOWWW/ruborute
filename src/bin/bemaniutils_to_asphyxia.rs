use std::fs;
use std::io::BufWriter;
use std::io::Write;

use ruborute::storage;
use ruborute::BemaniutilsDataSource;
use ruborute::Config;
use ruborute::Result;

fn main() -> Result<()> {
    let mut cfg = Config::load_from_args();
    if !cfg.config_file.is_empty() {
        let config_file = cfg.config_file;
        cfg = Config::load_from_file(config_file.as_str()).unwrap();
        cfg.config_file = config_file;
    }
    let bemanitutils_config = cfg.bemaniutils;
    let asphyxia_config = cfg.asyphyxia;
    let refid = asphyxia_config.refid;

    let ds = BemaniutilsDataSource::open(bemanitutils_config)?;
    let records = ds.get_records();
    let save_data_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(asphyxia_config.record_path)?;

    let mut writer = BufWriter::new(save_data_file);
    for (idx, r) in records.iter().enumerate() {
        let asphyxia_record = storage::AsphyxiaRecord::new_sdvx_record(
            refid.clone(),
            r.get_music_id(),
            r.get_difficulty().into(),
            r.get_score(),
            r.get_clear_type().into(),
            r.get_grade().into(),
            format!("{}", idx),
        );
        serde_json::to_writer(&mut writer, &asphyxia_record)?;
        writer.write(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}
