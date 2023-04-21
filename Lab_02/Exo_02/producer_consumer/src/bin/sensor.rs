
extern crate nix;
use rand::prelude::*;
use fcntl::*;
//use nix::fcntl::{flock, FlockArg};
use std::{fs::*, io::{Read, Write}};
use std::os::unix::io::AsRawFd;
use std::os::raw::{c_int, c_short};
use nix::libc::{F_GETLK, F_SETLK, F_SETLKW, F_RDLCK, F_WRLCK, F_UNLCK, flock};
use nix::unistd::*;


pub const NUM_SENSOR_TO_READ : usize = 10;

// sensor impl


#[derive(Debug, Default, Clone)]
pub struct Sensor{}

impl Sensor{

    pub fn rand_sensor_data_gen(&self) -> f32{
         rand::thread_rng().gen()
    }

}

// sensor data

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct SensorData {
pub seq: u32, // sequenza letture
pub values: [f32; 10],
pub timestamp: u32
}

// begin circularbuffer spec

// write
const CIRCULAR_BUFFER_LEN: usize = 20;

#[derive(Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct CircularBufferSensorDataWrite{
    index: usize,
    vec_buffer: [SensorData; CIRCULAR_BUFFER_LEN]
}


// read
#[derive(Default, Debug)]
#[repr(C)]
pub struct CircularBufferSensorDataRead{
    index: usize,
    vec_buffer: Vec::<SensorData>
}



impl CircularBufferSensorDataWrite{


    // write into buffer
    fn push_into_vec_buffer(&mut self, some_sensor_data:&mut SensorData){

        let mut index = self.index;
        if index % CIRCULAR_BUFFER_LEN != 0 && index !=0{
            self.vec_buffer[index] = some_sensor_data.clone();
            self.vec_buffer[index].seq = index as u32;
            self.index = index + 1;
        } else
        if index % CIRCULAR_BUFFER_LEN == 0{
            index = 0;
            self.vec_buffer[index] = some_sensor_data.clone();
            self.vec_buffer[index].seq = index as u32;
            self.index = index + 1;
        }

    }

    pub fn write_into_vec_buffer(&mut self, path: String, some_sensor_data:&mut SensorData){
        
        let mut file = File::options().write(true).append(false).open(path).unwrap();
        let fd = file.as_raw_fd();
        let pid = nix::unistd::Pid::this();
        let lock = flock {
        l_type: F_WRLCK as c_short,  // shared read lock
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: Pid::this().as_raw()
    };
        match lock_file(&file,  Some(lock), Some(FcntlLockType::Write) ){
            Ok(true) => println!("Write lock acquired"),
            Ok(false) => println!("Could'nt acquire write lock"),
            Err(err) => println!("Write acquisition fail")
            }

        match is_file_locked(&file, Some(lock) ){

            Ok(false) => 
            unsafe{
                let mut circular_buffer_size = std::mem::size_of::<CircularBufferSensorDataWrite>();
                self.push_into_vec_buffer(some_sensor_data);
                let mut slice_u8_from_self: &mut [u8] = std::slice::from_raw_parts_mut(self as *mut CircularBufferSensorDataWrite as *mut u8,
                    circular_buffer_size);
                    file.write(&slice_u8_from_self).unwrap();
            }
            ,
            
            Ok(true) => println!("Can't perform write because lock is busy"),

            Err(err) => print!("Control on lock for write fail")
            
        }
            
        match unlock_file(&file, Some(lock)) {
         Ok(true) => println!("Lock successfully released"),
         Ok(false) => println!("Falied to release lock"),
         Err(err) => println!("Error: {:?}", err),
        }
    }

}
    // end write into buffer

   


impl CircularBufferSensorDataRead{

     // read from buffer
    pub fn read_from_vec_buffer(&mut self, path: String) -> &mut CircularBufferSensorDataRead{

        let mut file = File::options().read(true).append(false).open(path).unwrap();
        let mut sensor_data: SensorData = SensorData::default();
        let fd = file.as_raw_fd();
        let lock = flock {
        l_type: F_RDLCK as c_short,  // shared read lock
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: Pid::this().as_raw()
    };
         match lock_file(&file,  Some(lock), Some(FcntlLockType::Read)){
            Ok(true) => println!("Read lock acquired"),
            Ok(false) => println!("Could'nt acquire lock"),
            Err(err) => println!("Acquisition fail")
        }

        match is_file_locked(&file, Some(lock)) {

            Ok(false) => unsafe{
                
                let mut circular_buffer_size: usize = usize::default();
                let mut slice_u8_from_self: &mut [u8] = Default::default();
                let mut index = usize::default();
                self.vec_buffer.clear();
                circular_buffer_size = std::mem::size_of::<usize>();
                slice_u8_from_self = std::slice::from_raw_parts_mut(index.to_le_bytes().as_mut_ptr().cast() as *mut u8,
                circular_buffer_size);
                file.read_exact(slice_u8_from_self).unwrap();
                self.index = slice_u8_from_self[0].try_into().unwrap();
                for i in 0..CIRCULAR_BUFFER_LEN{
                    if i < self.index{
                        
                        circular_buffer_size = std::mem::size_of::<SensorData>();
                        slice_u8_from_self = std::slice::from_raw_parts_mut(&mut sensor_data as *mut SensorData as *mut u8,
                    circular_buffer_size);
                        file.read_exact(slice_u8_from_self).unwrap();
                        self.vec_buffer.push(sensor_data);
                    }
                }
                println!("{:?}",self);
               /* circular_buffer_size = std::mem::size_of::<CircularBufferSensorData>();
                slice_u8_from_self = std::slice::from_raw_parts_mut(self as *mut CircularBufferSensorData as *mut u8,
                    circular_buffer_size);
                file.read_exact(slice_u8_from_self).unwrap();*/
        },
        
        Ok(true) => println!("Can't perform read because lock is busy"),

        Err(err) => print!("Control on lock for read fail")
            
        }
        match unlock_file(&file, Some(lock)) {

        Ok(true) => println!("Lock successfully released"),
        Ok(false) => println!("Falied to release lock"),
        Err(err) => println!("Error: {:?}", err),
        
        }
        self

    }

}

fn main(){
}

