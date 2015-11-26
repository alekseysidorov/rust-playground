use std::fs::File;
use std::io;
use std::io::{ BufReader, BufRead };
use std::path::Path;

use pixmap::Pixmap;
use tgaimage::{ ImageLoader, TgaImage };
use vector3d::{ Vec3f };

pub type Result<T> = io::Result<T>;

pub struct Model {
    pub verticies: Vec<Vec3f>,
    pub faces: Vec<[i32; 3]>,
    pub diffuse: Pixmap
}

impl Model {
    fn new() -> Model {
        Model {
            verticies: Vec::new(),
            faces: Vec::new(),
            diffuse: Pixmap::new(0, 0, 0)
        }
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
                
                let mut points : [f32; 3] = [ 0.0, 0.0, 0.0 ];
                for i in 0..3 {
                    points[i] = words[i+1].parse::<f32>().unwrap();
                }                
                let v = Vec3f::new(points[0], points[1], points[2]);
                model.verticies.push(v);
            } else if line.starts_with("f ") {
                let mut face = [-1, -1, -1];

                for i in 0..3 {
                    let mut words = words[i+1].split("/");
                    face[i] = words.next().unwrap().parse::<i32>().unwrap();
                    face[i] -= 1;
                }
                model.faces.push(face);
            }
        }
        Ok(model)
    }
}