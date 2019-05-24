extern crate argparse;

use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::prelude::*;

use argparse::{ArgumentParser, Store};

const PNG_PREFIX: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const IEND: [u8; 4] = [73, 69, 78, 68];

fn main() -> std::io::Result<()> {
    let mut input_file_path = String::new();

    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut input_file_path)
            .add_argument("input_file", Store, "")
            .required();
        parser.parse_args_or_exit();
    }

    let file = File::open(input_file_path)?;
    let mut buf_reader = BufReader::new(file);

    find_embedded_pngs(&mut buf_reader, |png, png_num| {
        let out_file_path = format!("image_{}.png", png_num);
        let mut out_file = File::create(&out_file_path)?;

        out_file.write_all(&png)?;

        println!("Wrote png to: {}", out_file_path);

        Ok(())
    })?;

    Ok(())
}

fn find_embedded_pngs<R: Read, F>(stream: &mut R, png_found_function: F) -> std::io::Result<()>
        where F: Fn(Vec<u8>, u32) -> std::io::Result<()> {
    let mut png_count = 0;
    let mut byte_buf = [0u8; 1];
    let mut next = 0;
    loop {
        match stream.read_exact(&mut byte_buf) {
            Err(e) => if e.kind() == ErrorKind::UnexpectedEof {
                return Ok(())
            } else {
                return Err(e);
            },
            _ => ()
        };

        let byte = byte_buf[0];
        if byte == PNG_PREFIX[next] {
            next += 1;

            if next == PNG_PREFIX.len() {
                let png = read_png(stream)?;

                png_found_function(png, png_count)?;

                png_count += 1;
                next = 0;
            }
        } else {
            next = 0;
        }
    }
}

fn read_png<R: Read>(stream: &mut R) -> std::io::Result<Vec<u8>> {
    let mut png = PNG_PREFIX.to_vec();

    loop {
        let mut length_buf = [0u8; 4];
        stream.read_exact(&mut length_buf)?;
        png.extend(length_buf.iter());

        let chunk_length = u32::from_be_bytes(length_buf);

        let mut chunk_type_buf = [0u8; 4];
        stream.read_exact(&mut chunk_type_buf)?;
        png.extend(chunk_type_buf.iter());

        let mut content_buf = vec![0u8; chunk_length as usize];
        stream.read_exact(&mut content_buf)?;
        png.extend(content_buf.iter());

        let mut crc_buf = [0u8; 4];
        stream.read_exact(&mut crc_buf)?;
        png.extend(crc_buf.iter());

        if chunk_type_buf == IEND {
            return Ok(png)
        }
    }
}
