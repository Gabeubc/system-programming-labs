#[path = "./sensor.rs"] mod sensor;

use std::{time::Duration};

use sensor::{CircularBufferSensorData};


fn consume(circular_buffer:&mut CircularBufferSensorData, path: String){

    circular_buffer.read_from_vec_buffer(path);

}


fn main (){

    let mut circular_buffer: CircularBufferSensorData = CircularBufferSensorData::default();
    let time_to_wait : Duration = Duration::from_secs(10);
    while true{
        std::thread::sleep(time_to_wait);
        consume(&mut circular_buffer, "C:/Users/youbi/Desktop/Process/Polito/Laurea-Magistrale/first year/Programmazione di Sistema/system-programming-labs/Lab_02/Exo_02/producer_consumer/src/resource/file.bin".to_string());
    }
}