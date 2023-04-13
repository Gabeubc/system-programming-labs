use core::fmt;
use std::{fs::{self, File}, io::Read, fmt::{Debug, Formatter}};
use std::io::BufReader;



#[derive(Clone, Copy, Debug)]
#[repr(packed)]
#[repr(C)]
struct value_struct{
    tipo: i32,
    val: f32,
    timestamp: i32
}


#[derive(Clone, Copy, Debug)]
#[repr(packed)]
#[repr(C)]
struct m_value_struct{
    tipo: i32,
    val: [f32; 10],
    timestamp: i32
}

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
#[repr(C)]
struct message_struct{
    tipo: i32,
    message: [u8; 21]
}

#[repr(packed)]
#[repr(C)]
union union_for_export_data{
    val: value_struct,
    m_val: m_value_struct,
    messages: message_struct
}

#[repr(packed)]
#[repr(C)]
struct export_data{
    tipo: i32,
    union: union_for_export_data
}

impl fmt::Debug for export_data{

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{

        unsafe{
            match self.tipo {
                
                1 => self.union.val.fmt(f),
                2 => self.union.m_val.fmt(f),
                3 => self.union.messages.fmt(f),
                _ => write!(f, "")
    
            }
        }

    }

}


impl export_data{

    fn from_file(path: String) -> Vec<export_data>{

        let mut reader = BufReader::new(File::open(&path).unwrap());
        let  num_bytes= fs::metadata(&path).unwrap().len() as usize;
        let  size_export_data= std::mem::size_of::<export_data>();
        let  num_struct = num_bytes/size_export_data;
        let mut var_export_data: Vec::<export_data>= Vec::<export_data>::with_capacity(num_struct);
        unsafe{

            let  buffer: &mut [u8]= std::slice::from_raw_parts_mut(var_export_data.as_mut_ptr().cast(),
             num_bytes);
            reader.read_exact(buffer).unwrap();

            var_export_data.set_len(num_struct);

            for i in 0..num_struct{
                println!(" {} -> {:#?}",i, var_export_data[i]);
            }
        }
        var_export_data

    }

}


fn main() {
    export_data::from_file("C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_01/legacy/C/myFile.bin".to_string());
}
