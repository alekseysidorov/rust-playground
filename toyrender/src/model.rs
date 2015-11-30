use std::fs::File;
use std::io;
use std::io::{ BufReader, BufRead };
use std::path::Path;

use pixmap::Pixmap;
use tgaimage::{ ImageLoader, TgaImage };
use vector3d::{ Vec3f, Vec3i };

pub type Result<T> = io::Result<T>;

#[derive(Default)]
pub struct Model {
    pub verticies: Vec<Vec3f>,
    pub faces: Vec<[Vec3i; 3]>,
    pub normals: Vec<Vec3f>,
    pub diffuse: Pixmap,
    pub uv: Vec<[f32; 2]>
}

impl Model {
    fn new() -> Model {
        Model {
            ..Default::default()
        }
    }
    
    pub fn uv(&self, iface: usize, nvert: usize) -> Vec3f {
        let idx = self.faces[iface][nvert][1] as usize;

        Vec3f::new(self.uv[idx][0] * self.diffuse.width() as f32,
                           self.uv[idx][1] * self.diffuse.height() as f32, 
                           0.0)
    }

}

pub struct Loader; //TODO

impl Loader {

    pub fn from_files(obj_path: &str, diffuse_path: &str) -> Result<Model> {
        let mut model = try!(Self::load_obj(obj_path));
        model.diffuse = try!(TgaImage::load(diffuse_path));
        
        Ok(model)
    }

    fn load_obj(path: &str) -> Result<Model> {
        let path = Path::new(path);
        let file = BufReader::new(try!(File::open(&path)));
    
        let mut model = Model::new();
        
        for line in file.lines() {
            let line = try!(line);

            let words: Vec<&str>;
            words = line.split_whitespace().collect();
            if line.starts_with("v ") {                
                let mut v = Vec3f { ..Default::default() };
                for i in 0..3 {
                    v[i] = words[i+1].parse::<f32>().unwrap();
                }                
                model.verticies.push(v);
            } else if line.starts_with("f ") {
                let mut face: [Vec3i; 3] = [Vec3i::new(-1, -1, -1); 3];
                let words: Vec<&str> = line.split_whitespace().collect();
                for i in 0..3 {
                    let mut j = 0;
                    for num in words[i+1].split("/") {
                        face[i][j] = num.parse::<i32>().unwrap() - 1;
                        j += 1;
                    } 
                }
                model.faces.push(face);
            } else if line.starts_with("vt ") {
                let w: Vec<&str> = line.split_whitespace().collect();
                model.uv.push([w[1].parse().unwrap(), w[2].parse().unwrap()]); //FIXME remove unwraps
            } else if line.starts_with("vn ") {
                let mut v = Vec3f { ..Default::default() };
                for i in 0..3 {
                    v[i] = words[i+1].parse::<f32>().unwrap();
                }                
                model.normals.push(v);
            } 
        }
        Ok(model)
    }
}
