use std::{fs::{self, File}, io::{self, BufWriter, Read}, path::{self, Path}};
use std::io::Write;
mod lib;
mod trait_understanding;
fn main()->io::Result<()> {

    



    let path = Path::new("test_folder/test_file.txt");
    let mut file_handle= File::open(path)?;
    let mut buffer = [0u8;16]; 
    let file = File::create("hex_dump_output.txt")?;
    let mut new_file = BufWriter::new(file);
    let mut hex_repr_buf = String::new();
    let mut asci_repr_buf = String::new();
    

    let mut offset: usize = 0;
    // println!("Offset   Hexadecimal Representation               ASCII Representation");
    loop {
        // fill the buffer with the values from the file_handle 
        let bytes_read = file_handle.read(&mut buffer)?;

        write!(new_file,"{:08x} ",offset)?;

        if bytes_read == 0 {
            break;
        }


        hex_repr_buf.clear();
        asci_repr_buf.clear();

        let chunk = &buffer[..bytes_read];

        lib::convert_chunk_v2(chunk,&mut hex_repr_buf,&mut asci_repr_buf);
        lib::format_line_write(&hex_repr_buf,&asci_repr_buf,&mut new_file)?;
        write!(new_file,"\n")?;

        offset +=bytes_read;
    } 

    Ok(())
    
}

