
mod perlin;

pub use perlin::PerlinNoise;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() { 
        use super::PerlinNoise;
        let perlin = PerlinNoise::new(100.0); //use seed


      

        assert_eq!(  perlin.perlin2(0.01,0.01), -0.00000009860109363395148);
    }
}