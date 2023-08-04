use rppal::gpio::Gpio;
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

    let sdcard = embedded_sdmmc::SdCard::new(
        sdmmc_spi,
        sdmmc_cs,
        Delay::new()
    );

    let time_source = FakeTimesource {};

    println!("Card size is {} bytes", sdcard.num_bytes().unwrap());
    // Now let's look for volumes (also known as partitions) on our block device.
    // To do this we need a Volume Manager. It will take ownership of the block device.
    let mut volume_mgr = embedded_sdmmc::VolumeManager::new(sdcard, time_source);
    // Try and access Volume 0 (i.e. the first partition).
    // The volume object holds information about the filesystem on that volume.
    // It doesn't hold a reference to the Volume Manager and so must be passed back
    // to every Volume Manager API call. This makes it easier to handle multiple
    // volumes in parallel.
    let mut volume0 = volume_mgr.get_volume(embedded_sdmmc::VolumeIdx(0)).unwrap();
    println!("Volume 0: {:?}", volume0);
    // Open the root directory (passing in the volume we're using).
    let root_dir = volume_mgr.open_root_dir(&volume0).unwrap();
    // Open a file called "MY_FILE.TXT" in the root directory
    let mut my_file = volume_mgr.open_file_in_dir(
        &mut volume0,
        &root_dir,
        "MY_FILE.TXT",
        embedded_sdmmc::Mode::ReadOnly,
    ).unwrap();
    // volume_mgr.write(&mut volume0, &mut my_file, "FOO\nLALA\naaaaa\nbar".as_bytes()).unwrap();
    // Print the contents of the file
    while !my_file.eof() {
        let mut buffer = [0u8; 32];
        let num_read = volume_mgr.read(&volume0, &mut my_file, &mut buffer).unwrap();
        for b in &buffer[0..num_read] {
            print!("{}", *b as char);
        }
    }
    volume_mgr.close_file(&volume0, my_file).unwrap();
    volume_mgr.close_dir(&volume0, root_dir);
}