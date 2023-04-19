#[path ="../../sensor/sensor.rs"] mod sensor;
use core::time;
use std::{fs::File, io::{Write, BufWriter, BufReader, Read}, time::Duration};
//use serde::{Serialize, Deserialize};
use sensor::*;

const NUM_SENSOR_TO_READ : usize = 10;


#[derive(Default, Debug, Serialize, Deserialize)]
#[repr(C)]
struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; NUM_SENSOR_TO_READ],
    timestamp: u32
}


fn read_sensors(sensor_data: &mut  SensorData, sensors: &mut Vec<SomeSensor>) -> (){
    let len = sensor_data.values.len();
    for i in 0.. len{
       sensor_data.values[i] = sensors[i].rand_sensor_data_gen();
    }
}

fn publish (path: String , circular_buffer: &mut CircularBuffer, sensor_data: &mut SensorData, sensors: &mut Vec::<SomeSensor> ,time_to_wait: Duration){

        std::thread::sleep(time_to_wait);
        read_sensors(&mut sensor_data, &mut sensors);
        circular_buffer.write_circular_buffer(&path, sensor_data);
      /*bincode_from_struct_sensor_data = bincode::serialize(&sensor_data).unwrap();
        buf_writer.write(&bincode_from_struct_sensor_data).unwrap();*/

}


fn main(){ 
    let args: Vec<String> = args().skip(1).collect();
    if args.len() <=0 {
        panic!("missing argument")
    }
    let mut sensors: Vec::<SomeSensor> = vec![SomeSensor::default(); NUM_SENSOR_TO_READ] ;
    let mut sensor_data = SensorData::default();
    let time_to_wait = time::Duration::from_secs(1);
    //let mut bincode_from_struct_sensor_data: Vec::<u8> = Vec::<u8>::default();
    let mut circular_buffer_instance: CircularBuffer = CircularBuffer::default();

    


    while true {
        
        publish(args[1] ,circular_buffer, &mut sensor_data, &mut sensors, time_to_wait);
      /*bincode_from_struct_sensor_data = bincode::serialize(&sensor_data).unwrap();
        buf_writer.write(&bincode_from_struct_sensor_data).unwrap();*/
    }
}