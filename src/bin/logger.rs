use std::thread;
use std::time::{Duration, Instant};

use embedded_sdmmc::{BlockDevice, TimeSource};
use embedded_toolbox_rs::lsm303d::{AccelerometerConfiguration, MagnetometerConfiguration, LSM303D, MagnetometerDataRate, MagneticSensorMode, MagnetometerFullScale, AccelerationDataRate, AccelerationFullScale, InternalTemperatureConfiguration, Measurements};
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
use rppal::hal::Delay;

struct FakeTimesource();

impl embedded_sdmmc::TimeSource for FakeTimesource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

fn main() {
    let sdmmc_spi = Spi::new(
        Bus::Spi0,
        SlaveSelect::Ss0,
        4_000_000,
        Mode::Mode0
    ).unwrap();

    let sdmmc_cs = Gpio::new().unwrap().get(16).unwrap().into_output();

    let time_source = FakeTimesource {};

    let i2c = I2c::new().unwrap();

    let sdcard = embedded_sdmmc::SdCard::new(
        sdmmc_spi,
        sdmmc_cs,
        Delay::new()
    );

    let mut lsm303d = LSM303D::new(i2c);
    lsm303d.check_connection().unwrap();
    lsm303d.configure_magnetometer(MagnetometerConfiguration {
        data_rate: MagnetometerDataRate::Hz50,
        mode: MagneticSensorMode::ContinuousConversion,
        scale: MagnetometerFullScale::Mag2Gauss,
    }).unwrap();
    lsm303d.configure_accelerometer(AccelerometerConfiguration
         {
        axis_x: true,
        axis_y: true,
        axis_z: true,
        data_rate: AccelerationDataRate::Hz50,
        scale: AccelerationFullScale::Acc2G,
    }).unwrap();
    lsm303d.configure_internal_temperature(
        InternalTemperatureConfiguration { active: true }
    ).unwrap();

    let mut volume_mgr = embedded_sdmmc::VolumeManager::new(sdcard, time_source);

    let mut volume0 = volume_mgr.get_volume(embedded_sdmmc::VolumeIdx(0)).unwrap();
    let root_dir = volume_mgr.open_root_dir(&volume0).unwrap();
    let my_file = volume_mgr.open_file_in_dir(
        &mut volume0,
        &root_dir,
        "data.csv",
        embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
    ).unwrap();
    volume_mgr.close_file(&mut volume0, my_file).unwrap();

    let mut collection: [(Duration, Measurements); 10] = [(Duration::from_millis(0), Measurements::default()); 10];
    let mut idx = 0;
    let now = Instant::now();
    loop {
        let res = lsm303d.read_measurements().unwrap();
        collection[idx] = (now.elapsed(), res);
        println!(
            "{idx:2} acc: {:3.3} {:3.3} {:3.3}\t mag: {:3.3} {:3.3} {:3.3}",
            res.accelerometer.x,
            res.accelerometer.y,
            res.accelerometer.z,

            res.magnetometer.x,
            res.magnetometer.y,
            res.magnetometer.z,
        );
        idx += 1;
        if idx == 10 {
            idx = 0;
            store_measurements(
                &collection,
                &mut volume_mgr,
                &mut volume0,
                &root_dir,
            );
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn store_measurements<D: BlockDevice, T: TimeSource>(
    collection: &[(Duration, Measurements); 10],
    volume_mgr: &mut embedded_sdmmc::VolumeManager<D, T>,
    mut volume0: &mut embedded_sdmmc::Volume,
    root_dir: &embedded_sdmmc::Directory,
) {
    let mut my_file = volume_mgr.open_file_in_dir(
        &mut volume0,
        &root_dir,
        "data.csv",
        embedded_sdmmc::Mode::ReadWriteAppend,
    ).unwrap();

    for (timestamp, entry) in collection {
        volume_mgr.write(
            &mut volume0,
            &mut my_file,
            format!(
                "{},{:3.5},{:3.5},{:3.5},{:3.5},{:3.5},{:3.5}\n",
                timestamp.as_millis(),
                entry.accelerometer.x,
                entry.accelerometer.y,
                entry.accelerometer.z,

                entry.magnetometer.x,
                entry.magnetometer.y,
                entry.magnetometer.z,
            ).as_bytes()
        ).unwrap();
    }

    volume_mgr.close_file(&mut volume0, my_file).unwrap();
}