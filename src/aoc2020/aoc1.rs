pub mod aocCommon {

  use std::path::PathBuf;
  #[derive(Copy, Clone)]
  pub enum DataFileType {
      Data,
      SampleData,
      Debug,
  }
  pub fn get_data_file_path(day: u32, datafile_type: DataFileType) -> String {
      let filename_suffix = match datafile_type {
          DataFileType::Data => "",
          DataFileType::SampleData => "Example",
          DataFileType::Debug => "Debug",
      };
      let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
      format!(
          "{}/src/aoc2020/data/Day{}_Data{}.txt",
          d.display(),day, filename_suffix
      )
  }

  pub fn load_input_lines(day: u32, datafile_type: DataFileType) -> String {
      let file_path = get_data_file_path(day, datafile_type);
      std::fs::read_to_string(file_path).unwrap()
  }
}

