use rand::prelude::*;
use std::{fs::*, io::{Read, Write}};

//use producer::SensorData;


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


const CIRCULAR_BUFFER_LEN: usize = 10;

#[derive(Default, Debug, Clone, Copy)]
pub struct CircularBufferSensorData{
    index: usize,
    vec_buffer: [SensorData; CIRCULAR_BUFFER_LEN]
}




impl CircularBufferSensorData{


    // write into buffer
    fn push_into_vec_buffer(&mut self, some_sensor_data:&mut SensorData){

        let mut index = self.index;
        if index % CIRCULAR_BUFFER_LEN != 0 && index !=0{
            self.vec_buffer[index] = some_sensor_data.clone();
            self.index = index + 1;
        } else
        if index % CIRCULAR_BUFFER_LEN == 0{
            index = 0;
            self.vec_buffer[index] = some_sensor_data.clone();
            self.index = index + 1;
        }

    }

    pub fn write_into_vec_buffer(&mut self, path: String, some_sensor_data:&mut SensorData){
        
        let mut file = File::options().write(true).open(path).unwrap();
        let mut circular_buffer_size = std::mem::size_of::<CircularBufferSensorData>();
        unsafe{
            self.push_into_vec_buffer(some_sensor_data);
            let mut slice_u8_from_self: &mut [u8] = std::slice::from_raw_parts_mut(self as *mut CircularBufferSensorData as *mut u8,
                circular_buffer_size);
                file.write(&slice_u8_from_self).unwrap();
        }

    }
    // end write into buffer

    // read from buffer
    pub fn read_from_vec_buffer(&mut self, path: String) -> &mut CircularBufferSensorData{

        let mut file = File::options().read(true).open(&path).unwrap();
        let mut circular_buffer_size = std::mem::size_of::<CircularBufferSensorData>();
        let mut index = self.index;
        unsafe{
            let mut slice_u8_from_self: &mut [u8] = std::slice::from_raw_parts_mut(self as *mut CircularBufferSensorData as *mut u8,
                circular_buffer_size);
                file.read_exact(slice_u8_from_self).unwrap();
        }
        println!("{:?}",self);
        self

    }

}

fn main(){
}
