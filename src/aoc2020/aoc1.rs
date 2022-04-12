pub mod aocCommon {

  use std::path::PathBuf;

  #[derive(Copy, Clone)]
  pub enum AocYear {
    Aoc2020,
    Aoc2021,
  }
  #[derive(Copy, Clone)]
  pub enum AocDay {
    Day1,
    Day5,
    Day8,
  }

  #[derive(Copy, Clone)]
  pub enum DataFileType {
    Data,
    SampleData,
    Debug,
  }
  pub fn get_data_file_path(year: AocYear, day: AocDay, datafile_type: DataFileType) -> String {
    let filename_suffix = match datafile_type {
      DataFileType::Data => "",
      DataFileType::SampleData => "Example",
      DataFileType::Debug => "Debug",
    };
    let aocYear = match year {
      AocYear::Aoc2020 => "2020",
      AocYear::Aoc2021 => "2021"
    };
    let aocDay = match day {
      AocDay::Day1 => "1",
      AocDay::Day5 => "5",
      AocDay::Day8 => "8",
    };
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    format!(
      "{}/src/aoc{}/data/Day{}_Data{}.txt",
      d.display(),
      aocYear,
      aocDay,
      filename_suffix
    )
  }

  pub fn load_input_lines(year: AocYear, day: AocDay, datafile_type: DataFileType) -> String {
    let file_path = get_data_file_path(year, day, datafile_type);
    std::fs::read_to_string(file_path).unwrap()
  }
}
