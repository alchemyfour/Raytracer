use crate::vector3d::Vector3d;

struct Camera {
    origin: Vector3d,
    fovy: f64,
    fovx: f64,
    resolutionx: f64,
    resolutiony: f64,
    dir: Vector3d,
}

impl Camera {
    pub fn new(origin: Vector3d, fovy: f64, fovx: f64, resolutionx: f64, resolutiony: f64, dir: Vector3d) -> Self {
        Camera {origin, fovy, fovx, resolutionx, resolutiony, dir}
    }

    pub fn fire(&mut self) {
        // should iteratively fire every X and Y ray by taking the direction and accounting for FOV
        // takes each one of these rays and sticks ones that hit into a hits Vec<Pixel>, and a misses Vec<Pixel>
        // once done, refires all pixels in the hits section by calculating their reflection (doesn't account for roughness ATM)
        // adds hits and misses to their respective Vec
        // repeats until all rays are in the misses field, or repeats more than some set ammount of times (we'll say 10)
        // returns a vec with all pixels in it, to have their colors computed seprately based on their hits
    }
}