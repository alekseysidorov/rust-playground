use std::mem;
use std::fs::File;
use std::io;
use std::io::{ Read, BufReader, Error, ErrorKind };
use std::path::Path;

use pixmap::Pixmap;

pub type Result<T> = io::Result<T>;

pub trait ImageLoader
{
    fn load(path: &str) -> Result<Pixmap>;
}

const HEADERSIZE: usize = 18; // 18 = sizeof(TgaHeader)
#[repr(C, packed)]
struct TgaHeader {
	idlength: i8,
	colormaptype: i8,
	datatypecode: i8,
	colormaporigin: i16,
	colormaplength: i16,
	colormapdepth: i8,
	x_origin: i16,
	y_origin: i16,
	width: i16,
	height: i16,
	bitsperpixel: i8,
	imagedescriptor: i8,
}

pub struct TgaImage;

impl TgaImage {
    fn read_header(file: &mut BufReader<File>) -> Result<TgaHeader> {
        let mut header_bytes: [u8; HEADERSIZE] = [0; HEADERSIZE];
        try!(file.read(&mut header_bytes));
        
        let header = unsafe { mem::transmute::<[u8; HEADERSIZE], TgaHeader>(header_bytes) };
        Ok(header)
    }
    
    fn read_raw(inp: &mut BufReader<File>, out: &mut Vec<u8>) -> Result<()> {
        try!(inp.read_to_end(out));
        
        Ok(())
    }
    
    fn read_rle(pixelcount: usize, bytespp: usize, inp: &mut BufReader<File>, out: &mut Vec<u8>) -> Result<()> {
        let mut buffer = Vec::new();
        try!(inp.read_to_end(&mut buffer));
        
        let mut pos = 0;
        let mut pix = 0;
        while pix < pixelcount {
            let mut chunkheader = buffer[pos] as usize;
            pos+=1;
            if chunkheader<128 {
                chunkheader+=1;
                let endpos = pos+chunkheader*bytespp;
                while pos < endpos {
                    out.push(buffer[pos]);
                    pos+=1;
                }
            } else {
                chunkheader -= 127;
                for _i in 0..chunkheader{
                    for j in 0..bytespp {
                        out.push(buffer[pos+j]);
                    }
                }
                pos+=bytespp;
            }
            pix+=chunkheader;
        }
        
        Ok(())
    }
}

impl ImageLoader for TgaImage {
    fn load(path: &str) -> Result<Pixmap> {
        let path = Path::new(path);
        let file = try!(File::open(&path));
        let mut file = BufReader::new(file);
        
        let header = try!(Self::read_header(&mut file));
        let w = header.width as usize;
        let h = header.height as usize;
        let bpp = (header.bitsperpixel>>3) as usize;

        let mut buffer: Vec<u8> = Vec::with_capacity(w*h*bpp);
        try!(match header.datatypecode {
            2...3    => Self::read_raw(&mut file, &mut buffer),
            10...11  => Self::read_rle(w*h, bpp, &mut file, &mut buffer),
            _        => Err(Error::new(ErrorKind::Other, "oh no!")),
        });

        let mut pixmap = Pixmap::new(w, h, 0);
        
        for y in 0..h {
            for x in 0..w {
                match bpp {
                    1 =>  {
                        let intensity = buffer[y*w+x] as u32;
                        pixmap[x][y] = (intensity + (intensity << (8*1)) + (intensity << (8*2))) as i32;
                    },
                    v @ 3...4 => {
                        let bytes = &buffer[(y*w+x)*v..(y*w+x+1)*v];
                        pixmap[x][y] = (bytes[0] as u32 + ((bytes[1] as u32) << (8*1)) + ((bytes[2] as u32) << (8*2))) as i32;
                    },
                    _ => {}
                }
            }
        }
        
        Ok(pixmap)
    }
}