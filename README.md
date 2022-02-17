# perlin_rust


A slightly modified version of https://github.com/wantaotao1994/perlin_rust
``` 
    let perlin = PerlinNoise::new(100.0); //use seed
    assert_eq!(  perlin.perlin2(0.01,0.01), -0.00000009860109363395148);
    assert_eq!(  perlin.perlin3(0.01, 0.01, 0.01), 0.01999940916000743);

```

See cargo doc for more implementations.
