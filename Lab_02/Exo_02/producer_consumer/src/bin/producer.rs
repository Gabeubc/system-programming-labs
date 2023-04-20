#[path = "./sensor.rs"] mod sensor;

use std::{env::args, time::Duration};
use sensor::{Sensor, SensorData, CircularBufferSensorDataWrite, NUM_SENSOR_TO_READ};


const TIME_TO_WAIT: u64 = 1;

fn read_sensors(sensor_data: &mut SensorData, sensors:&mut [Sensor; NUM_SENSOR_TO_READ]){
    
    for i in 0..NUM_SENSOR_TO_READ {
        sensor_data.values[i] = sensors.iter().nth(i).unwrap().rand_sensor_data_gen();
    }

}

fn publish(circularbuffer:&mut CircularBufferSensorDataWrite, sensor_data:&mut SensorData, path: String){

    circularbuffer.write_into_vec_buffer(path, sensor_data);

}


fn main(){

    let mut args: Vec<String> = args().skip(1).collect();
    let mut circularbuffer =CircularBufferSensorDataWrite::default();
    let mut sensor_data: SensorData = SensorData::default();
    let mut sensors: [Sensor; NUM_SENSOR_TO_READ] = Default::default();
    let mut time_to_wait: Duration = Duration::from_secs(TIME_TO_WAIT);

    while true {
        std::thread::sleep(time_to_wait);
        read_sensors(&mut sensor_data, &mut sensors);
        publish(&mut circularbuffer,&mut sensor_data, "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_02/producer_consumer/src/resource/file.bin".to_string());
    }
    
}