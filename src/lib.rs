use std::fmt::Write;

#[cfg(test)]

#[test]

fn test_line_process(){
    let line:[u8;5] = [104,2,3,4,5];
    let result = process_line(&line,0);
    assert_eq!(result,String::from("blood"));

}

#[test]

fn test_format_line(){
    let offset = 0;
    let hex_layout = "4656 4343 656";
    let asci_rep = "hello world";
    let mut buff_string:String = String::new();

    format_line(offset, hex_layout, asci_rep, &mut buff_string);
    println!("{}",buff_string);
}




// Output -> Fully formatted hexdump line to print
pub fn format_line(offset:usize,hex_layout:&str,asci_rep:&str,buff_string:&mut String){
    let _ = write!(buff_string,"{:08x} | {:?} | {:?}",offset,hex_layout,asci_rep);
}

pub fn process_line(line:&[u8],offset:usize)->String{
    let mut result = String::new();
    // byte_offset

    let _ = write!(result,"{:08x}",offset);
    let _ = write!(result, " | ");


    // hexdecimal
    let mut hex_string = String::new();
    for (i,byte) in line.iter().enumerate(){
       let _ =  write!(hex_string,"{:02x}",byte);
       if (i+1) %2 == 0 {
        let _ = write!(result," ");
       }
    }
    let _ = write!(result,"{:32}",hex_string);
    let _ = write!(result, " | ");

    
    //asci representation

    for byte in line{
        if *byte >=32 && *byte <=126 {
        let _ = write!(result, "{}",*byte as char);
            continue;
        }
        let _ = write!(result,".");
    } 

    let _ = write!(result,"");
    result
}