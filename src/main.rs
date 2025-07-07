use byteorder::{ReadBytesExt, WriteBytesExt};
use clap::Parser;
use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Parser, Debug)]
struct Args {
    /// The input file to compress or decompress
    input: String,

    /// The output file to write the compressed or decompressed data
    output: String,

    /// Decompress the input file instead of compressing it
    #[clap(short, long)]
    decompress: bool,
}

fn compress<R: Read, W: Write>(input: R, output: W) -> std::io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut i = 0;
    while i < buffer.len() {
        let current_byte = buffer[i];
        let mut run_length = 1;

        while i + run_length < buffer.len()
            && buffer[i + run_length] == current_byte
            && run_length < 127
        {
            run_length += 1;
        }

        if run_length >= 3 {
            let header = run_length as u8;
            writer.write_u8(header)?;
            writer.write_u8(current_byte)?;
            i += run_length;
        } else {
            let start = i;
            let mut literal_length = 0;

            while i < buffer.len() && literal_length < 127 {
                let current = buffer[i];
                let mut next_run = 1;

                while i + next_run < buffer.len() && buffer[i + next_run] == current && next_run < 3
                {
                    next_run += 1;
                }

                if next_run >= 3 {
                    break;
                }

                i += next_run;
                literal_length += next_run;
            }

            let header = 0b10000000 | (literal_length as u8);
            writer.write_u8(header)?;
            writer.write_all(&buffer[start..start + literal_length])?;
        }
    }

    writer.write_u8(0)?;
    writer.flush()?;
    Ok(())
}

fn decompress<R: Read, W: Write>(input: R, output: W) -> std::io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    loop {
        let header = reader.read_u8()?;
        let literal = header & 0b10000000 != 0;
        let length = header & 0b01111111;

        if length == 0 {
            break;
        }

        if literal {
            let mut buffer = vec![0; length as usize];
            reader.read_exact(&mut buffer)?;
            writer.write_all(&buffer)?;
        } else {
            let value = reader.read_u8()?;
            for _ in 0..length {
                writer.write_u8(value)?;
            }
        }
    }

    writer.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let input_file = std::fs::File::open(&args.input)?;
    let output_file = std::fs::File::create(&args.output)?;

    if args.decompress {
        decompress(input_file, output_file)?;
        println!(
            "Successfully decompressed {} to {}",
            args.input, args.output
        );
    } else {
        compress(input_file, output_file)?;
        println!("Successfully compressed {} to {}", args.input, args.output);
    }

    Ok(())
}
