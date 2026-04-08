
use clap::Parser;
use compiler::{Compiler, CompilerOptions, Target};
use std::fs;
use std::path::PathBuf;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Parser, Debug)]
#[command(name = "mylang")]
#[command(about = "My new programming language compiler")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    #[command(about = "Compile source files")]
    Compile {
        #[arg(value_name = "FILE")]
        files: Vec<PathBuf>,
        
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        #[arg(short, long)]
        wasm: bool,
        
        #[arg(short, long, default_value_t = true)]
        optimize: bool,
    },
    
    #[command(about = "Run a source file")]
    Run {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    
    #[command(about = "Print version information")]
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { files, output, wasm, optimize } => {
            let options = CompilerOptions {
                optimize,
                target: if wasm { Target::Wasm32 } else { Target::Native },
                output_file: output.map(|p| p.to_string_lossy().to_string()),
            };
            
            files.par_iter().for_each(|file| {
                match fs::read_to_string(file) {
                    Ok(source) => {
                        let mut compiler = Compiler::new(options.clone());
                        match compiler.compile(&source) {
                            Ok(result) => {
                                println!("Compilation successful for {}!", file.to_string_lossy());
                                if let Some(asm) = result.assembly {
                                    println!("Assembly for {}: {}", file.to_string_lossy(), asm);
                                }
                            },
                            Err(e) => {
                                println!("Compilation failed for {}: {}", file.to_string_lossy(), e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Failed to read file {}: {}", file.to_string_lossy(), e);
                    }
                }
            });
        }
        
        Commands::Run { file } => {
            let source = fs::read_to_string(&file)?;
            
            let options = CompilerOptions::default();
            let mut compiler = Compiler::new(options);
            let _result = compiler.compile(&source)?;
            
            println!("Execution would happen here");
        }
        
        Commands::Version => {
            println!("MyLang Compiler v0.1.0");
        }
    }

    Ok(())
}

