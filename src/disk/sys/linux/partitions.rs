use std::fs;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use crate::disk::{FileSystem, Partition};
use crate::utils::invalid_data;

impl FromStr for Partition {
	type Err = io::Error;

	fn from_str(line: &str) -> io::Result<Partition> {
		// Example: `/dev/sda3 /home ext4 rw,relatime,data=ordered 0 0`
		let fields: Vec<&str> = line.split_whitespace().collect();

		if fields.len() < 4 {
			return Err(invalid_data(
				"failed to load partition information on '/proc/mounts'",
			));
		}

		Ok(Partition {
			device: String::from(fields[0]),
			mountpoint: PathBuf::from(fields[1]),
			filesystem: FileSystem::from_str(fields[2]).unwrap(), // infallible unwrap
			mount_options: String::from(fields[3]),
		})
	}
}

pub fn partitions() -> io::Result<Vec<Partition>> {
	fs::read_to_string("/proc/mounts")?
		.lines()
		.map(|line| Partition::from_str(line))
		.collect()
}

pub fn partitions_physical() -> io::Result<Vec<Partition>> {
	Ok(partitions()?
		.into_iter()
		.filter(|partition| partition.filesystem.is_physical())
		.collect())
}
