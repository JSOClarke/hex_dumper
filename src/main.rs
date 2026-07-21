use std::{error::Error, fs::{self, File}, io::{self, Read}, path::{self, Path}};
mod hex_dump;
use hex_dump::process_line;
fn main()->io::Result<()> {
    let path = Path::new("../test_files/test_1.txt");
    let mut file_handle= File::open(path)?;
    let mut buffer = [0u8;16]; 
    let mut offset: i32 = 0;

    loop {
        // fill the buffer with the values from the file_handle 
        let bytes_read = file_handle.read(&mut buffer)?;
        if bytes_read == 0 {
            
            break;
        }

        let chunk = &buffer[..bytes_read];
        
         // this is basically giving us read rights to the buffer array making sure that we are only touching teh bytes that have been streamed
        process_line(chunk,offset);
        offset +=bytes_read as i32;
        println!()
    } 

    Ok(())
    
}


pub fn decimal_to_hex(){

}
