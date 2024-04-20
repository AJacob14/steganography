use std::fs;

use image::{GenericImage, GenericImageView, Pixel};

pub fn encode(src: &str, msg_src: &str, dst: &str) -> Result<(), &'static str> {
    let message = fs::read(msg_src);
    if message.is_err() {
        return Err("Error reading message file");
    }
    let message = message.unwrap();
    let message_size = message.len();
    println!("Message size: {}", message_size);
    let mut payload = Vec::new();
    payload.extend_from_slice(&message_size.to_be_bytes());
    payload.extend_from_slice(&message);
    let mut payload = BitIterator::new(&payload);
    let img = image::open(src);
    if img.is_err() {
        return Err("Error opening image file");
    }

    let mut img = img.unwrap();
    let height = img.height();
    let width = img.width();

    if message_size * 8 > (height * width * 3) as usize {
        return Err("Message too large to encode in image");
    }

    //let mut debug_counter = 0;
    'main_loop:
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mut new_pixel = pixel.clone();
            let mut channels = 0;
            for channel in new_pixel.channels_mut() {
                channels += 1;
                if channels == 4 { // Skip alpha channel
                    break;
                }

                // if debug_counter < 64 {
                //     debug_counter += 1;
                //     println!("Channel: {}", channel);
                // }
                if let Some(bit) = payload.next() {
                    *channel = (*channel & 0b1111_1110) | bit;
                } 
                else {
                    break;
                }
            }
            img.put_pixel(x, y, new_pixel);
            if payload.exhausted() {
                break 'main_loop;
            }
        }
    }

    let result = img.save(dst);
    if result.is_err() {
        return Err("Error saving image file");
    }

    Ok(())
}

pub fn decode(src: &str, dst: &str) -> Result<(), &'static str> {
    let img = image::open(src);
    if img.is_err() {
        return Err("Error opening image file");
    }

    let img = img.unwrap();
    let height = img.height();
    let width = img.width();
    let mut payload = Vec::new();
    let mut byte = 0;
    let mut bits = 0;
    let mut message_size = None;
    'main_loop:
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mut channels = 0;
            for channel in pixel.channels() {
                channels += 1;
                if channels == 4 { // Skip alpha channel
                    break;
                }

                byte = (byte << 1) | (channel & 1);
                bits += 1;
                if bits != 8 {
                    continue;
                }

                payload.push(byte);
                byte = 0;
                bits = 0;

                if message_size.is_none() {
                    if payload.len() == 8 {
                        let size = u64::from_be_bytes([payload[0], payload[1], payload[2], payload[3], payload[4], payload[5], payload[6], payload[7]]) as usize;
                        println!("Message size: {}", size);
                        message_size = Some(size);
                        payload.clear();
                    }
                }
                else if let Some(size) = message_size {
                    if payload.len() == size {
                        break 'main_loop;
                    }
                }
            }
        }
    }

    let result = fs::write(dst, &payload);
    if result.is_err() {
        return Err("Error writing message file");
    }
    
    Ok(())
}

pub struct BitIterator<'a> {
    bytes: &'a [u8],
    byte_index: usize,
    bit_index: u8,
}

impl<'a> BitIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        BitIterator {
            bytes,
            byte_index: 0,
            bit_index: 0,
        }
    }

    pub fn exhausted(&self) -> bool {
        self.byte_index >= self.bytes.len()
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_index >= self.bytes.len() {
            return None;
        }

        let result = (self.bytes[self.byte_index] >> (7 - self.bit_index)) & 1;

        self.bit_index += 1;
        if self.bit_index > 7 {
            self.bit_index = 0;
            self.byte_index += 1;
        }

        Some(result)
    }
}