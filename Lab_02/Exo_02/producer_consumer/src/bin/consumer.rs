#[path = "./sensor.rs"] mod sensor;

use std::{time::Duration};

use sensor::{CircularBufferSensorDataRead};

const TIME_TO_WAIT: u64 = 10;

fn consume(circular_buffer:&mut CircularBufferSensorDataRead, path: String){

    circular_buffer.read_from_vec_buffer(path);

}


fn main (){

    let mut circular_buffer: CircularBufferSensorDataRead = CircularBufferSensorDataRead::default();
    let time_to_wait : Duration = Duration::from_secs(TIME_TO_WAIT);
    while true{
        std::thread::sleep(time_to_wait);
        consume(&mut circular_buffer, "./resource/file.bin".to_string());
    }
}
