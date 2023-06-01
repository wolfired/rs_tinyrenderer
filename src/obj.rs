//! <http://paulbourke.net/dataformats/obj/>
//!
//!

#![allow(dead_code)]

use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use crate::la::Vector2;
use crate::la::Vector3;

#[derive(Debug)]
pub struct Vertex {
    pub vi: usize,
    pub vti: usize,
    pub vni: usize,
}

#[derive(Debug)]
pub struct Triangle {
    pub vs: Vec<Vertex>,
}

#[derive(Debug)]
pub struct Obj {
    pub vs: Vec<Vector3<f32>>,
    vts: Vec<Vector2<f32>>,
    vns: Vec<Vector3<f32>>,
    pub fs: Vec<Triangle>,
}

impl Obj {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(path)?;

        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let mut vs = Vec::new();
        let mut vts = Vec::new();
        let mut vns = Vec::new();
        let mut fs = Vec::new();

        for line in content.lines() {
            if line.starts_with("v ") {
                let ds: Vec<f32> = line.split_whitespace().skip(1).take(3).map(|w| w.parse().unwrap()).collect();
                let Ok(v0) = ds[0].try_into() else {
                    return Err("err".into());
                };
                let Ok(v1) = ds[1].try_into() else {
                    return Err("err".into());
                };
                let Ok(v2) = ds[2].try_into() else {
                    return Err("err".into());
                };
                vs.push([v0, v1, v2].into());
            } else if line.starts_with("vt ") {
                let ds: Vec<f32> = line.split_whitespace().skip(1).take(2).map(|w| w.parse().unwrap()).collect();
                let Ok(v0) = ds[0].try_into() else {
                        return Err("err".into());
                    };
                let Ok(v1) = ds[1].try_into() else {
                        return Err("err".into());
                    };
                vts.push([v0, v1].into());
            } else if line.starts_with("vn ") {
                let ds: Vec<f32> = line.split_whitespace().skip(1).take(3).map(|w| w.parse().unwrap()).collect();
                let Ok(v0) = ds[0].try_into() else {
                        return Err("err".into());
                    };
                let Ok(v1) = ds[1].try_into() else {
                        return Err("err".into());
                    };
                let Ok(v2) = ds[2].try_into() else {
                        return Err("err".into());
                    };
                vns.push([v0, v1, v2].into());
            } else if line.starts_with("f ") {
                let vs: Vec<Vertex> = line
                    .split_whitespace()
                    .skip(1)
                    .take(3)
                    .map(|w| {
                        let ds: Vec<usize> = w.split("/").map(|w| w.parse().unwrap()).collect();
                        Vertex {
                            vi: ds[0],
                            vti: ds[1],
                            vni: ds[2],
                        }
                    })
                    .collect();
                fs.push(Triangle { vs });
            }
        }

        Ok(Self { vs, vts, vns, fs })
    }
}
