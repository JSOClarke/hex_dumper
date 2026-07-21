use std::{fs::{self, File}, io::{self, Read}, path::{self, Path}};
mod lib;
fn main()->io::Result<()> {

    



    let path = Path::new("test_folder/test_file.txt");
    let mut file_handle= File::open(path)?;
    let mut buffer = [0u8;16]; 
    

    let mut offset: usize = 0;
    println!("Offset   Hexadecimal Representation               ASCII Representation");
    loop {
        // fill the buffer with the values from the file_handle 
        let bytes_read = file_handle.read(&mut buffer)?;
        if bytes_read == 0 {
            
            break;
        }

        let chunk = &buffer[..bytes_read];
        
        let (hex_dumper,ascii_dumper) = lib::convert_chunk(chunk);
        let result = lib::format_line(offset,&hex_dumper,&ascii_dumper);
        println!("{}",&result);
        offset +=bytes_read;
    } 
    print!("{:08x}",offset);

    Ok(())
    
}

