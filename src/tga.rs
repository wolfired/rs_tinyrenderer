use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use crate::la::Vector2;
use crate::la::Vector4;
use crate::la::A;
use crate::la::B;
use crate::la::G;
use crate::la::R;
use crate::la::X;
use crate::la::Y;

pub type TGACoord = Vector2<u16>;

pub type TGAColor = Vector4<u8>;

pub const BLACK: [u8; 4] = [0u8, 0u8, 0u8, 255u8];
pub const WHITE: [u8; 4] = [255u8, 255u8, 255u8, 255u8];
pub const RED: [u8; 4] = [255u8, 0u8, 0u8, 255u8];
pub const GREEN: [u8; 4] = [0u8, 255u8, 0u8, 255u8];
pub const BLUE: [u8; 4] = [0u8, 0u8, 255u8, 255u8];

pub const TGAHEADER_BYTES_COUNT: usize = 18;

#[derive(Default)]
pub struct TGAHeader {
    pub id_length: u8,
    pub color_map_type: u8,
    pub image_type: u8,
    pub color_map_origin: u16,
    pub color_map_entries_count: u16,
    pub color_map_bits_per_entry: u8,
    pub image_origin_x: u16,
    pub image_origin_y: u16,
    pub image_width: u16,
    pub image_height: u16,
    pub image_bits_per_pixel: u8,
    pub image_descriptor: u8,
}

impl TGAHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        if TGAHEADER_BYTES_COUNT > bytes.len() {
            return Err("文件头解释失败".into());
        }

        Ok(Self {
            id_length: bytes[0],
            color_map_type: bytes[1],
            image_type: bytes[2],
            color_map_origin: u16::from_le_bytes(bytes[3..5].try_into()?),
            color_map_entries_count: u16::from_le_bytes(bytes[5..7].try_into()?),
            color_map_bits_per_entry: bytes[7],
            image_origin_x: u16::from_le_bytes(bytes[8..10].try_into()?),
            image_origin_y: u16::from_le_bytes(bytes[10..12].try_into()?),
            image_width: u16::from_le_bytes(bytes[12..14].try_into()?),
            image_height: u16::from_le_bytes(bytes[14..16].try_into()?),
            image_bits_per_pixel: bytes[16],
            image_descriptor: bytes[17],
        })
    }

    pub fn to_bytes(&self, mut bytes: &mut [u8]) -> Result<(), Box<dyn Error>> {
        if TGAHEADER_BYTES_COUNT > bytes.len() {
            return Err("文件头解释失败".into());
        }

        bytes.write_all(&self.id_length.to_le_bytes())?;
        bytes.write_all(&self.color_map_type.to_le_bytes())?;
        bytes.write_all(&self.image_type.to_le_bytes())?;
        bytes.write_all(&self.color_map_origin.to_le_bytes())?;
        bytes.write_all(&self.color_map_entries_count.to_le_bytes())?;
        bytes.write_all(&self.color_map_bits_per_entry.to_le_bytes())?;
        bytes.write_all(&self.image_origin_x.to_le_bytes())?;
        bytes.write_all(&self.image_origin_y.to_le_bytes())?;
        bytes.write_all(&self.image_width.to_le_bytes())?;
        bytes.write_all(&self.image_height.to_le_bytes())?;
        bytes.write_all(&self.image_bits_per_pixel.to_le_bytes())?;
        bytes.write_all(&self.image_descriptor.to_le_bytes())?;

        Ok(())
    }
}

impl Display for TGAHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"
ID Length: {},
Color Map Type: {},
Image Type: {},
Color Map Specification:
    Color Map Origin: {},
    Color Map Entries Count: {},
    Color Map Bits Per Entry: {},
Image Specification:
    X Origin of Image: {},
    Y Origin of Image: {},
    Width of Image: {},
    Height of Image: {},
    Image Bits Per Pixel: {},
    Image Descriptor: {:08b}
",
            self.id_length,
            self.color_map_type,
            self.image_type,
            self.color_map_origin,
            self.color_map_entries_count,
            self.color_map_bits_per_entry,
            self.image_origin_x,
            self.image_origin_y,
            self.image_width,
            self.image_height,
            self.image_bits_per_pixel,
            self.image_descriptor
        )
    }
}

pub struct TGAImage {
    header: TGAHeader,
    bytes: Vec<u8>,
}

impl TGAImage {
    pub fn new(image_width: u16, image_height: u16) -> Self {
        let image_bits_per_pixel = 32;
        Self {
            header: TGAHeader {
                image_type: 2,
                image_width,
                image_height,
                image_bits_per_pixel,
                image_descriptor: match image_bits_per_pixel {
                    16 => 0b00_00_0001,
                    24 => 0b00_00_0000,
                    32 => 0b00_00_1000,
                    _ => 0b00_00_1000,
                },
                ..TGAHeader::default()
            },
            bytes: vec![
                0;
                (image_width as u64 * image_height as u64 * image_bits_per_pixel as u64)
                    as usize
                    >> 3
            ],
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(path)?;

        let header_bytes = &mut [0; TGAHEADER_BYTES_COUNT][..];

        file.read_exact(header_bytes)?;

        let header = TGAHeader::from_bytes(header_bytes)?;

        let mut bytes = vec![
            0;
            (header.image_width as u64
                * header.image_height as u64
                * header.image_bits_per_pixel as u64) as usize
                >> 3
        ];

        file.read_exact(bytes.as_mut_slice())?;

        Ok(Self { header, bytes })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        let header_bytes = &mut [0; TGAHEADER_BYTES_COUNT][..];

        self.header.to_bytes(header_bytes)?;

        file.write_all(header_bytes)?;
        file.write_all(self.bytes.as_slice())?;

        Ok(())
    }

    pub fn get_width(&self) -> u16 {
        self.header.image_width
    }
    pub fn get_height(&self) -> u16 {
        self.header.image_height
    }

    pub fn set_color(&mut self, coord: TGACoord, color: TGAColor) {
        let index = ((coord.x() as u64 + coord.y() as u64 * self.header.image_width as u64)
            * self.header.image_bits_per_pixel as u64) as usize
            >> 3;
        self.bytes[index + 0] = color.b();
        self.bytes[index + 1] = color.g();
        self.bytes[index + 2] = color.r();
        self.bytes[index + 3] = color.a();
    }
}
