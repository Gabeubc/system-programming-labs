use core::{fmt};
use std::{fs::{self, File}, io::Read, fmt::{Debug, Formatter}};
use std::io::BufReader;



#[derive(Clone, Copy, Debug, Default)]
#[repr(packed)]
#[repr(C)]
struct value_struct{
    tipo: i32,
    val: f32,
    timestamp: i32
}


#[derive(Clone, Copy, Debug, Default)]
#[repr(packed)]
#[repr(C)]
struct m_value_struct{
    tipo: i32,
    val: [f32; 10],
    timestamp: i32
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(packed)]
#[repr(C)]
struct message_struct{
    tipo: i32,
    message: [u8; 21]
}


#[derive(Clone, Copy)]
#[repr(packed)]
#[repr(C)]
union union_for_export_data{
    val: value_struct,
    m_val: m_value_struct,
    messages: message_struct
}


impl Default for union_for_export_data{

    fn default() -> Self {
    
        union_for_export_data {m_val: m_value_struct::default()}

    }
}


#[derive(Clone, Copy)]
#[repr(packed)]
#[repr(C)]
struct export_data{
    tipo: i32,
    union: union_for_export_data
}



impl Default for export_data{

    fn default() -> Self {
        export_data { tipo: i32::default(), union: union_for_export_data::default() }
    }
}

impl Debug for export_data{

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        
        unsafe{
            match self.tipo{
    
                1 => self.union.val.fmt(f),
                2 => self.union.m_val.fmt(f),
                3 => self.union.messages.message.fmt(f),
                _ => write!(f,"")
    
            }
        }

    }

}


impl export_data{

    fn from_file(path: String) -> Vec::<export_data>{

        let mut reader = BufReader::new(File::open(&path).unwrap());
        let num_bytes_of_files = fs::metadata(&path).unwrap().len() as usize;
        let  tipo: i32= i32::default();
        let  val: f32 = f32::default();
        let  timestamp: i32 = i32::default();
        let  my_char: u8 = Default::default();
        let  val_size = std::mem::size_of::<f32>();
        let  timestamp_size = std::mem::size_of::<i32>();
        let  my_char_size = std::mem::size_of::<u8>();
        let tipo_size = std::mem::size_of::<i32>();
        let mut size_bytes_already_read: usize = usize::default();
        let mut buffer: &mut [u8] = Default::default();
        let mut var_value_struct = value_struct::default();
        let mut var_m_value_struct = m_value_struct::default();
        let mut var_message_struct = message_struct::default();
        let mut var_union_for_export_data : union_for_export_data = union_for_export_data::default();
        let mut var_export_data = export_data::default();
        let export_data_size = std::mem::size_of::<export_data>();
        let value_struct_size = std::mem::size_of::<value_struct>();
        let m_value_struct_size = std::mem::size_of::<m_value_struct>();
        let message_struct_size = std::mem::size_of::<message_struct>();
        let mut vec_export_data: Vec::<export_data> = Vec::<export_data>::new();
        unsafe{


            while size_bytes_already_read < num_bytes_of_files{

                buffer = std::slice::from_raw_parts_mut(tipo.to_le_bytes().as_mut_ptr().cast() as *mut u8, 
                tipo_size);
                reader.read_exact(buffer).unwrap();
                size_bytes_already_read += tipo_size;
                var_export_data.tipo = i32::from_le_bytes(buffer[0..4].try_into().unwrap());

                var_export_data.union = match var_export_data.tipo {

                    1 => {
                        //get tipo
                        buffer = std::slice::from_raw_parts_mut(tipo.to_le_bytes().as_mut_ptr().cast(), 
                tipo_size);
                reader.read_exact(buffer).unwrap();
                size_bytes_already_read += tipo_size;
                var_value_struct.tipo = i32::from_le_bytes(buffer[0..4].try_into().unwrap());
                //get val
                        buffer = std::slice::from_raw_parts_mut(val.to_le_bytes().as_mut_ptr().cast(), 
                val_size);
                reader.read_exact(buffer).unwrap();
                size_bytes_already_read += val_size;
                var_value_struct.val = f32::from_le_bytes(buffer[0..4].try_into().unwrap());
                //get tmiestamp
                        buffer = std::slice::from_raw_parts_mut(timestamp.to_le_bytes().as_mut_ptr().cast(), 
                timestamp_size);
                reader.read_exact(buffer).unwrap();
                size_bytes_already_read += timestamp_size;
                var_value_struct.timestamp = i32::from_le_bytes(buffer[0..4].try_into().unwrap());
                        var_union_for_export_data.val = var_value_struct;
                        buffer = std::slice::from_raw_parts_mut(my_char.to_le_bytes().as_mut_ptr().cast(), 
                        (export_data_size - value_struct_size) - tipo_size as usize);
                        reader.read_exact(buffer).unwrap();
                        size_bytes_already_read += (export_data_size - value_struct_size) - tipo_size as usize; 
                        var_union_for_export_data
                    },

                    2 => {
                        //get tipo
                        buffer = std::slice::from_raw_parts_mut(tipo.to_le_bytes().as_mut_ptr().cast(), 
                tipo_size);
                reader.read_exact(buffer).unwrap();
                size_bytes_already_read += tipo_size;
                var_m_value_struct.tipo = i32::from_le_bytes(buffer[0..4].try_into().unwrap());
                //get val[]
                for i in 0..10{
                    
                    buffer = std::slice::from_raw_parts_mut(val.to_le_bytes().as_mut_ptr().cast(), 
                    val_size);
                    reader.read_exact(buffer).unwrap();
                    size_bytes_already_read += val_size;
                    var_m_value_struct.val[i] = f32::from_le_bytes(buffer[0..4].try_into().unwrap());
                }
                //get timestamp
                        buffer = std::slice::from_raw_parts_mut(timestamp.to_le_bytes().as_mut_ptr().cast(), 
                timestamp_size);
                reader.read_exact(buffer).unwrap();
                size_bytes_already_read += timestamp_size;
                var_m_value_struct.timestamp = i32::from_le_bytes(buffer[0..4].try_into().unwrap());
                        var_union_for_export_data.m_val = var_m_value_struct;
                        buffer = std::slice::from_raw_parts_mut(my_char.to_le_bytes().as_mut_ptr().cast(), 
                        (export_data_size - m_value_struct_size) - tipo_size as usize);
                        reader.read_exact(buffer).unwrap();
                        size_bytes_already_read += (export_data_size - m_value_struct_size) - tipo_size as usize; 
                        var_union_for_export_data

                    },

                    3 => {
                        // get tipo
                        buffer = std::slice::from_raw_parts_mut(tipo.to_le_bytes().as_mut_ptr().cast(), 
                tipo_size);
                reader.read_exact(buffer).unwrap();
                size_bytes_already_read += tipo_size;
                var_message_struct.tipo = i32::from_le_bytes(buffer[0..4].try_into().unwrap());
                //get message
                for i in 0..21{
                    
                    buffer = std::slice::from_raw_parts_mut(my_char.to_le_bytes().as_mut_ptr().cast(), 
                    my_char_size);
                    reader.read_exact(buffer).unwrap();
                    size_bytes_already_read += my_char_size;
                    var_message_struct.message[i] = buffer[0].try_into().unwrap();
                }
                        var_union_for_export_data.messages = var_message_struct;
                        buffer = std::slice::from_raw_parts_mut(my_char.to_le_bytes().as_mut_ptr().cast(), 
                        (export_data_size - message_struct_size) - tipo_size as usize);
                        reader.read_exact(buffer).unwrap();
                        size_bytes_already_read += (export_data_size - message_struct_size)- tipo_size as usize; 
                        var_union_for_export_data
                    },

                    _ => {
                        var_union_for_export_data
                    }
                    
                };
                
                vec_export_data.push(var_export_data);

            }

        }

        vec_export_data
    }

}


fn main () -> (){
    let result =   export_data::from_file("C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_01/legacy/C/myFile.bin".to_string());
    //let len = result.len();
     for (i, value) in result.iter().enumerate(){
            println!("{} -> {:#?}", i,  value)
     }
}