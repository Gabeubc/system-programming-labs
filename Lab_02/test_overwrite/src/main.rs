use std::{io::{BufReader, BufWriter, Read, Write}, fs::File};



fn main() {
    let path = "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/test_overwrite/file.bin".to_string();
    let mut data: u8 = 20 ;
    let mut data_u: u8 = 210 ;
    let mut slice_u8: &mut [u8]= Default::default();
    let mut size = std::mem::size_of::<[u8;10]>();
    let mut file = File::options().write(true).read(true).open(path).unwrap();
    unsafe{
        slice_u8 = std::slice::from_raw_parts_mut( &mut data, 
                                    std::mem::size_of::<[u8;21]>());
        file.read_exact(slice_u8);
        println!("{}", data);
        file.read_exact(slice_u8);
        println!("{}", data);
        file.read_exact(slice_u8);
        println!("{}", data);
    }
}
