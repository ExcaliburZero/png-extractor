use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;

//let PNG_PREFIX: Vec<u8> = vec![89, 50, 4E, 47, 0D, 0A, 1A, 0A];
const PNG_PREFIX: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

const IEND: [u8; 4] = [73, 69, 78, 68];

fn main() -> std::io::Result<()> {
    let mut file = File::open("")?;
    let mut buf_reader = BufReader::new(file);

    let mut next = 0;
    let mut count = 0;
    let mut img_count = 0;
    loop {
    //for byte in buf_reader.bytes() {
        //let value = byte.unwrap();
        let mut buf = [0];
        buf_reader.read(&mut buf);
        let mut value = buf[0];
        //print!("{:x?} ", value);

        if (next < 8) {
            if (value == PNG_PREFIX[next]) {
                next += 1;

                if next == 8 {
                    println!("Found png byte: {}", count);

                    let png = read_png(&mut buf_reader);

                    let mut file = File::create(format!("image_{}.png", img_count))?;
                    img_count += 1;
                    file.write_all(&png);
                    println!("Wrote png to ...");

                    next = 0;
                }
            } else {
                next = 0;
            }
        } else {
            panic!("Ahhhh!!!");
        }

        count += 1;

        if count % 100000 == 0 {
            println!("{}", count);
        }
    }

    Ok(())
}

fn read_png<R: Read>(stream: &mut R) -> Vec<u8> {
    let mut png = PNG_PREFIX.to_vec();

    loop {
        let mut length_buf = [0u8; 4];
        stream.read_exact(&mut length_buf);
        png.extend(length_buf.iter());

        let chunk_length = u32::from_be_bytes(length_buf);

        let mut chunk_type_buf = [0u8; 4];
        stream.read_exact(&mut chunk_type_buf);
        png.extend(chunk_type_buf.iter());

        let mut content_buf = vec![0u8; chunk_length as usize];
        stream.read_exact(&mut content_buf);
        png.extend(content_buf.iter());

        let mut crc_buf = [0u8; 4];
        stream.read_exact(&mut crc_buf);
        png.extend(crc_buf.iter());

        if chunk_type_buf == IEND {
            return png
        }
    }
}
