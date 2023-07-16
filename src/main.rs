use sdl2::log;
//use crate::math::vector2d_struct::GVector2d;
// mod math;
// use crate::math::vector2d_module::Vector2d;
//use crate::math::vector2d_struct::GVector2d;
// use std::f32::consts::FRAC_PI_2;
// pub mod math;

use vectorlib::math::vector2d_module::Vector2d;
use vectorlib::math::vector2d_verbose_module::VerboseVector2d;

#[allow(clippy::let_unit_value)]
fn main() {
    // -> Result<(), String> {
    println!("******************************************************");
    println!("STARTING SDL2 -- rust_physics_engine --- Dev.");
    println!("******************************************************");
    log::log("Welcome to SDL2");
    let v = Vector2d::new(1.0, 2.0);
    let w = Vector2d::new(3, 4);
    println!("{v:#?}");
    println!("{w:#?}");
    let k: Vector2d<i32> = Vector2d::new(3, 4);
    let s = w + k;
    println!("Vector -> {s:#?}");
    let k = v.scale(10.0);
    println!("Vector -> {k}");
    let u = Vector2d::new(10.0, 20.0);
    let r = k.dot_product(u);
    println!("{r}");
    let _o1 = VerboseVector2d::new(v, true);
    let _o2 = VerboseVector2d::new(w, true);
    {
        let _o3 = VerboseVector2d::new(u, true);
    }

    // //Ok(())
}
