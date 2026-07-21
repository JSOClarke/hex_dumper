use std::fmt::Write;

#[cfg(test)]

#[test]

fn test_conver_chunk(){
    let data = b"Jordan Clarke";
    let (hex_repr,asci_repr) = convert_chunk(data);
    assert_eq!(hex_repr,"string");
    assert_eq!(asci_repr,"string");

}

#[test]

fn test_format_line(){
    let offset = 0;
    let hex_repr = "4656 4343 656";
    let asci_rep = "hello world";


    let result = format_line(offset, hex_repr, asci_rep);
    assert_eq!(result,"string");
}
#[test]

fn test_e2e(){
    let data = b"This is the first thing that i could think of";
    let (hex_repr,asci_repr) = convert_chunk(data);
    let result = format_line(0, &hex_repr, &asci_repr);
    
    assert_eq!(result,"string");

}




// Output -> Fully formatted hexdump line to print
pub fn format_line(offset:usize,hex_repr:&str,asci_repr:&str)->String{
    let mut result:String = String::new();
    let _ = write!(result,"{:08x} | {:32} | {}",offset,hex_repr,asci_repr);
    return result;
}
// Goal - Converts the line so that it can get passed to the format line
pub fn convert_chunk(line:&[u8])->(String,String){
    let mut hex_repr:String = String::new();
    let mut asci_repr:String = String::new();

    for (i,byte) in line.iter().enumerate(){
        let _ = write!(hex_repr,"{:02x}",byte);
        if (i+1) % 2 == 0{
            let _ = write!(hex_repr," ");
        }
            if *byte >= 32 && *byte <= 126 {
                let _ = write!(asci_repr,"{}",*byte as char);
            }
            else {
                let _ = write!(asci_repr,".",);
            }   
        
    }

    return (hex_repr,asci_repr);
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