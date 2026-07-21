#[cfg(test)]

#[test]

fn test_line_process(){
    let line:[u8;5] = [1,2,3,4,5];
    process_line(&line,0);

}


pub fn process_line(line:&[u8],offset:i32){

    // byte_offset
    print!("{:02x} | ",offset);


    // hexdecimal
    for byte in line{
        print!("{:02x}",byte);
    }
    print!(" | ");

    
    //asci representation

    for byte in line{
        if *byte >=32 && *byte <126 {
        print!("{}",*byte as char);
            continue;
        }
        print!(".");
    } 

    println!("");
}