
use std::path::PathBuf;

use clap::Parser;

//Simple assembler for the SPU by Killar.exe
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AssemblerArgs {
    ///Input/Source file.
    pub input_file: PathBuf,

    //Output/Compiled file.
    #[arg(short, long)]
    pub output_file: Option<PathBuf>
}

impl AssemblerArgs {
    pub fn get_file_output(&self) -> PathBuf {
        if let Some(output_file) = &self.output_file {
            output_file.to_path_buf()
        } else {
            let mut output_file: PathBuf = self.input_file.clone();
            output_file.set_extension("bin");
            output_file
        }
    }
}
