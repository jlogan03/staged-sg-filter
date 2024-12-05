//use staged_sg_filter::par_sav_gol;
use staged_sg_filter::{sav_gol, sav_gol_f32};

fn main() {
    #[cfg(feature = "std")]
    {
        const N: usize = 100_000_000;
        // f32
        let v = [10.0f32; N];
        let mut buf = [0.0f32; N];
        let start = std::time::Instant::now();
        sav_gol_f32::<2, 2>(&mut buf, &v);
        let duration = start.elapsed();

        println!("f32: {:?}", duration);
        println!("{:?}", &buf[0..3]);

        // f64
        let v = [10.0; N];
        let mut buf = [0.0; N];
        let start = std::time::Instant::now();
        sav_gol::<2, 2>(&mut buf, &v);
        let duration = start.elapsed();

        println!("f64: {:?}", duration);
        println!("{:?}", &buf[0..3]);
    }
}
