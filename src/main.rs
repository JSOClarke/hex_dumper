use std::{fs::{self, File}, io::{self, Read}, path::{self, Path}};
mod lib;


use lib::process_line;
fn main()->io::Result<()> {

    



    let path = Path::new("../test_files/test_1.txt");
    let mut file_handle= File::open(path)?;
    let mut buffer = [0u8;16]; 
    let mut buff_string = String::new();

    let mut offset: usize = 0;

    loop {
        // fill the buffer with the values from the file_handle 
        let bytes_read = file_handle.read(&mut buffer)?;
        if bytes_read == 0 {
            
            break;
        }

        let chunk = &buffer[..bytes_read];
        
         // this is basically giving us read rights to the buffer array making sure that we are only touching teh bytes that have been streamed
        let res = process_line(chunk,offset);
        println!("{}",&res);
        offset +=bytes_read;
        println!()
    } 

    Ok(())
    
}


pub fn decimal_to_hex(){

}
