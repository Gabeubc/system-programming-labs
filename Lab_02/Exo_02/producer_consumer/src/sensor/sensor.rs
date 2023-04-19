#[path = "../bin/producer/main.rs"] mod producer;

use producer::*;
use bincode::de;
use rand::prelude::*;
use std::{fs::*, default, io::{Read, Write}};

use producer::SensorData;


const NUM_SENSOR_TO_READ : usize = 10;

// sensor impl


#[derive(Debug, Default, Clone)]
pub struct SomeSensor{}

impl SomeSensor{
    pub fn rand_sensor_data_gen(&self) -> f32{

        
         rand::thread_rng().gen()

    }
}



//// circular buffer impl

const CIRCULAR_BUFFER_LEN: usize = 10;

#[derive(Default)]
pub struct CircularBuffer{
    index: usize,
    vec_buffer: [[SensorData; NUM_SENSOR_TO_READ]; CIRCULAR_BUFFER_LEN]
}




impl CircularBuffer{

    fn push_into_vec_buffer(&mut self, sensor_data: [SensorData; NUM_SENSOR_TO_READ]){
        
        if self.index % (CIRCULAR_BUFFER_LEN) != 0{
            self.vec_buffer[self.index] = sensor_data;
            self.index = self.index + 1;
        }
        else{
            self.index = 0;
            self.vec_buffer[self.index] = sensor_data;
        }  
    }

    fn pop_into_vec_buffer(&mut self){
        
        if self.index % CIRCULAR_BUFFER_LEN != 0{
            self.vec_buffer[self.index] = sensor_data;
            self.index = self.index + 1;
        }
        else{
            self.index = 0;
            self.vec_buffer[self.index] = sensor_data;
        }  
    }


    pub fn read_circular_buffer(&mut self, path: String) -> (){
        let mut circular_buffer_instance = CircularBuffer::default();
        let mut file = File::options().read(true).open(&path).unwrap();
        let mut CIRCULAR_BUFFER_LEN: usize = Default::default();
        let mut attribute_size: usize = usize::default();
        let mut buf_read: &mut [u8] = Default::default();
        unsafe{
            attribute_size = std::mem::size_of::<usize>();
            buf_read = std::slice::from_raw_parts_mut(self.index.to_le_bytes().as_mut_ptr().cast(),
            attribute_size );
            file.read_exact(buf_read).unwrap();

            if self.index % CIRCULAR_BUFFER_LEN != 0{
                self.index = self.index + 1;
                attribute_size = std::mem::size_of::< [some_sensor; CIRCULAR_BUFFER_LEN] >();
                buf_read = std::slice::from_raw_parts_mut(self.vec_buffer.as_mut_ptr().cast(),
                attribute_size );
                file.read_exact(buf_read).unwrap();
            }
            else{
                circular_buffer_instance.index = CIRCULAR_BUFFER_LEN ;
                attribute_size = std::mem::size_of::< [some_sensor; CIRCULAR_BUFFER_LEN] >();
                buf_read = std::slice::from_raw_parts_mut( self.vec_buffer.as_mut_ptr().cast(),
                attribute_size );
                file.read_exact(buf_read).unwrap();
            } 
            //update index lettura
            file = File::options().read(true).open(&path).unwrap();
            attribute_size = std::mem::size_of::<usize>();
            buf_read = std::slice::from_raw_parts_mut(self.index.to_le_bytes().as_mut_ptr().cast(),
            attribute_size );
            file.write(&buf_read).unwrap();   

        }
    }

    pub fn write_circular_buffer(&mut self, path: String, sensor_data: SensorData) -> (){

        let mut file = File::options().write(true).open(&path).unwrap();
        let mut circular_buffer_len: usize = Default::default();
        let mut buf_read: &mut [u8] = Default::default();
        //get circular buffer from file
        unsafe{
            self.push_into_vec_buffer(&sensor_data);
            circular_buffer_len = std::mem::size_of::<CircularBuffer>();
            buf_read = std::slice::from_raw_parts_mut(&mut self as *mut CircularBuffer as *mut u8,
            circular_buffer_len );
            file.write(&buf_read).unwrap();           
        }

    }
}