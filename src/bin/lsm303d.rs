use std::{thread, time::Duration};

use embedded_toolbox_rs::lsm303d::{LSM303D, MagnetometerConfiguration, MagnetometerDataRate, MagneticSensorMode, MagnetometerFullScale, AccelerometerConfiguration, AccelerationDataRate, AccelerationFullScale, InternalTemperatureConfiguration, Register};
use rppal::i2c::I2c;

fn main() {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(0x1d).unwrap();

    let mut lsm303d = LSM303D::new(i2c);
    lsm303d.check_connection().unwrap();
    lsm303d.configure_magnetometer(MagnetometerConfiguration {
        data_rate: MagnetometerDataRate::Hz50,
        mode: MagneticSensorMode::ContinuousConversion,
        scale: MagnetometerFullScale::Mag2Gauss,
    }).unwrap();
    lsm303d.configure_accelerometer(AccelerometerConfiguration {
        axis_x: true,
        axis_y: true,
        axis_z: true,
        data_rate: AccelerationDataRate::Hz50,
        scale: AccelerationFullScale::Acc2G,
    }).unwrap();
    lsm303d.configure_internal_temperature(
        InternalTemperatureConfiguration { active: true }
    ).unwrap();

    loop {
        let res = lsm303d.read_measurements().unwrap();
        println!(
            "acc: {:3.3} {:3.3} {:3.3}\t mag: {:3.3} {:3.3} {:3.3}",
            res.accelerometer.x,
            res.accelerometer.y,
            res.accelerometer.z,

            res.magnetometer.x,
            res.magnetometer.y,
            res.magnetometer.z,
        );
        thread::sleep(Duration::from_millis(100));
    }
}
