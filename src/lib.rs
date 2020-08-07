use std::io::{Read, BufReader};
use std::iter::Iterator;


pub struct H74Encoder<R> {
	reader: BufReader<R>,
	buffer: Option<u8>,
}

impl<R: Read> H74Encoder<R> {
	pub fn new(reader: R) -> H74Encoder<R> {
		let reader = BufReader::new(reader);
		H74Encoder {
			reader,
			buffer: None,
		}
	}
}

impl<R: Read> Iterator for H74Encoder<R> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(b) = self.buffer {
			self.buffer = None;
			return Some(b);
		}

		let mut buffer: [u8; 1] = [0; 1];
		match self.reader.read(&mut buffer) {
			Ok(n) => if n == 0 { return None },
			Err(_) => panic!("failed to read from input stream"),
		}

		let mut bytes = split_byte(buffer[0]);
		bytes.0 = encode(bytes.0);
		bytes.1 = encode(bytes.1);

		self.buffer = Some(bytes.1);

		Some(bytes.0)
	}
}


pub struct H74Decoder<R> {
	reader: BufReader<R>,
}

impl<R: Read> H74Decoder<R> {
	pub fn new(reader: R) -> H74Decoder<R> {
		let reader = BufReader::new(reader);
		H74Decoder {
			reader,
		}
	}
}

impl <R: Read> Iterator for H74Decoder<R> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		let mut buffer: [u8; 2] = [0; 2];

		match self.reader.read(&mut buffer) {
			Ok(n) => if n == 0 {return None},
			Err(_) => panic!("failed to read from input stream"),
		}

		for mut byte in buffer.iter_mut() {
			let got = extract_parity(*byte);
			let expected = get_parity(extract_data(*byte));

			correct(&mut byte, expected, got);
		}

		let d0 = extract_data(buffer[0]);
		let d1 = extract_data(buffer[1]);

		Some((d0 << 4) | d1)
	}
}


fn get_parity(data: u8) -> u8 {
	assert!(data <= 0x0f);

	let d3 = (data & 0x08) >> 3;
	let d2 = (data & 0x04) >> 2;
	let d1 = (data & 0x02) >> 1;
	let d0 = data & 0x01;

	let p2 = d3 ^ d2 ^ d1;
	let p1 = d3 ^ d2 ^ d0;
	let p0 = d3 ^ d1 ^ d0;

	(p2 << 2) | (p1 << 1) | p0
}

fn encode(data: u8) -> u8 {
	let p = get_parity(data);

	let d3 = (data & 0x08) << 3;
	let d2 = (data & 0x04) << 3;
	let d1 = (data & 0x02) << 3;
	let d0 = (data & 0x01) << 2;

	let p2 = (p & 0x04) << 1;
	let p1 = p & 0x02;
	let p0 = p & 0x01;

	d3 | d2 | d1 | d0 | p2 | p1 | p0
}

fn split_byte(byte: u8) -> (u8, u8) {
	let upper = byte >> 4;
	let lower = byte & 0x0f;

	(upper, lower)
}

fn extract_parity(byte: u8) -> u8 {
	let p2 = (byte & 0x08) >> 1;
	let p1 = byte & 0x02;
	let p0 = byte & 0x01;

	p2 | p1 | p0
}

fn extract_data(byte: u8) -> u8 {
	let d3 = (byte & 0x40) >> 3;
	let d2 = (byte & 0x20) >> 3;
	let d1 = (byte & 0x10) >> 3;
	let d0 = (byte & 0x04) >> 2;

	d3 | d2 | d1 | d0
}

fn correct(byte: &mut u8, parity1: u8, parity2: u8) {
	let diff = parity1 ^ parity2;

	*byte ^= (0x01 << (diff)) >> 1;
}