#[path = "./sensor.rs"] mod sensor;

use std::{env::args};
use sensor::{Sensor, SensorData, CircularBufferSensorData, NUM_SENSOR_TO_READ};








fn read_sensors(sensor_data: &mut SensorData, sensors:&mut [Sensor; NUM_SENSOR_TO_READ]){
    
    for i in 0..NUM_SENSOR_TO_READ {
        sensor_data.values[i] = sensors.iter().nth(i).unwrap().rand_sensor_data_gen();
    }

}

fn push(circularbuffer:&mut CircularBufferSensorData, sensor_data:&mut SensorData, path: String){

    circularbuffer.write_into_vec_buffer(path, sensor_data);

}


fn main(){

    let mut args: Vec<String> = args().skip(1).collect();
    let mut circularbuffer =CircularBufferSensorData::default();
    let mut sensor_data: SensorData = SensorData::default();
    let mut sensors: [Sensor; NUM_SENSOR_TO_READ] = Default::default();

    while true {
        read_sensors(&mut sensor_data, &mut sensors);
        push(&mut circularbuffer,&mut sensor_data, "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_02/producer_consumer/src/resource/file.bin".to_string());
    }
    
}