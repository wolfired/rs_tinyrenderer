#![feature(generic_arg_infer)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;
use std::f32::consts::PI;
use std::mem::size_of;
use std::mem::size_of_val;
use std::ops::Add;
use std::ops::Deref;
use std::path::Path;

use rs_tinyrenderer::cg::bresenham;
use rs_tinyrenderer::la::Cross;
use rs_tinyrenderer::la::CrossAssign;
use rs_tinyrenderer::la::Dot;
use rs_tinyrenderer::la::Magnitude;
use rs_tinyrenderer::la::Matrix;
use rs_tinyrenderer::la::NormalAssign;
use rs_tinyrenderer::la::ScaleAssign;
use rs_tinyrenderer::la::Transpose;
use rs_tinyrenderer::la::TransposeAssign;
use rs_tinyrenderer::la::Vector2;
use rs_tinyrenderer::la::Vector3;
use rs_tinyrenderer::la::Vector4;
use rs_tinyrenderer::la::R;
use rs_tinyrenderer::la::W;
use rs_tinyrenderer::la::X;
use rs_tinyrenderer::la::Y;
use rs_tinyrenderer::la::Z;
use rs_tinyrenderer::obj::Obj;
use rs_tinyrenderer::tga::TGACoord;
use rs_tinyrenderer::tga::TGAImage;
use rs_tinyrenderer::tga::RED;
use rs_tinyrenderer::tga::WHITE;

fn main() -> Result<(), Box<dyn Error>> {
    african_head("./african_head.obj", "./66666.tga")?;

    Ok(())
}

fn african_head<P: AsRef<Path>>(obj_file: P, tga_file: P) -> Result<(), Box<dyn Error>> {
    let obj = Obj::load(obj_file)?;

    let mut img = TGAImage::new(1024, 1024);

    let w = img.get_width() - 1;
    let h = img.get_height() - 1;

    let mut vs: Vec<Vector2<i32>> = Vec::with_capacity(obj.vs.capacity());

    for v in obj.vs {
        let x = (v.x() + 1.0) * w as f32 / 2.0;
        let y = (v.y() + 1.0) * h as f32 / 2.0;
        vs.push([x as i32, y as i32].into());
    }

    for f in obj.fs {
        let p0 = vs[f.vs[0].vi - 1];
        let p1 = vs[f.vs[1].vi - 1];
        let p2 = vs[f.vs[2].vi - 1];

        bresenham(p0, p1, |x, y| img.set_color([x as u16, y as u16].into(), WHITE.into()));

        bresenham(p1, p2, |x, y| img.set_color([x as u16, y as u16].into(), WHITE.into()));

        bresenham(p2, p0, |x, y| img.set_color([x as u16, y as u16].into(), WHITE.into()));
    }

    img.save(tga_file)?;

    Ok(())
}
