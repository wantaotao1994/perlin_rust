#[derive(Clone)]
pub struct PerlinNoise {
    perm: [usize; 512],
    grad_p:[ Grad;512]
}

#[derive(Clone,Copy)]
 struct  Grad {
    x:f64,
    y:f64,
    z:f64
}
impl  Grad {
    fn new(x:f64,y:f64,z :f64)->Grad{
        Grad{
            x,
            y,
            z,
        }
    }
    fn dot2(&self,x:f64,y:f64)->f64 {
        self.x * x + self.y*y
    }
    fn dot3(&self,x:f64,y:f64,z:f64)->f64{
        self.x * x + self.y*y+self.z*z

    }
}
impl PerlinNoise {

    pub fn new(seed:f64) -> PerlinNoise {
        let mut perm:[usize;512] = [0; 512];        
       let   p:[usize;256] = [
            151,160,137,91,90,15,
            131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
            190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
            88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
            77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
            102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
            135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
            5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
            223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
            129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
            251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
            49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
            138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180,
            
        ];
        let grad3 = [ Grad::new(1.0,1.0,0.0), Grad::new(-1.0,1.0,0.0), Grad::new(1.0,-1.0,0.0), Grad::new(-1.0,-1.0,0.0),
                                Grad::new(1.0,0.0,1.0), Grad::new(-1.0,0.0,1.0), Grad::new(1.0,0.0,-1.0), Grad::new(-1.0,0.0,-1.0),
                                Grad::new(0.0,1.0,1.0), Grad::new(0.0,-1.0,1.0), Grad::new(0.0,1.0,-1.0), Grad::new(0.0,-1.0,-1.0)];
   
        let mut  grand3:[Grad;512]=[Grad::new(0.0, 0.0, 0.0);512];

        let mut seed0 = seed;
        if seed0 > 0.0 && seed0 < 1.0 {
          // Scale the seed out
          seed0 *= 65536.0;
        }
    
        let mut seed0 = seed0.floor() as u64;

        if seed0 < 256 {
            seed0 |= seed0 << 8;
        }
        
        for i in 0..256 {
            let mut  v: usize =0;
            if i & 1 == 1 {
                v = p[i] ^ (seed0 & 255) as usize;
            } else {
                v = p[i] ^ ((seed0>>8) & 255) as usize;
            }
            perm[i] =v;
            perm[i + 256] =v;

            grand3[i] =  grad3[(v % 12) as usize];

            grand3[i + 256]= grand3[i];
        }
        PerlinNoise {
            perm,
            grad_p:grand3
        }
    }
    
    pub fn perlin2 (&self,x:f64,y:f64) ->f64{
            let x0 = (x.floor() as usize) & 255;
            let y0 = (y.floor() as usize) & 255;

            let x =x- x.floor();
            let y =y- y.floor();



            let n00 = self.grad_p[x0+self.perm[y0]].dot2(x, y);
            let n01 =  self.grad_p[x0+self.perm[y0+1]].dot2(x, y-1.0);
            let n10 =  self.grad_p[x0+1+self.perm[y0]].dot2(x-1.0, y);
            let n11 =  self.grad_p[x0+1+self.perm[y0+1]].dot2(x-1.0, y-1.0);
 
            let  u = fade(x);

            lerp(
                lerp(n00, n10, u),
                lerp(n01, n11, u),
                fade(y))
    }

    pub fn perlin3(&self,x:f64,y:f64,z:f64){
        let x0 = (x.floor() as usize) & 255;
        let y0 = (y.floor() as usize) & 255;
        let z0 = (z.floor() as usize) & 255;

        let x =x- x.floor();
        let y  =y- y.floor();
        let z  =z- z.floor();


        // Calculate noise contributions from each of the eight corners
        let n000 = self.grad_p[x0+  self.perm[y0+  self.perm[z0  ]]].dot3(x,   y,     z);
        let n001 = self.grad_p[x0+  self.perm[y0+  self.perm[z0+1]]].dot3(x,   y,   z-1.0);
        let n010 = self.grad_p[x0+  self.perm[y0+1+self.perm[z0  ]]].dot3(x,   y-1.0,   z);
        let n011 = self.grad_p[x0+  self.perm[y0+1+self.perm[z0+1]]].dot3(x,   y-1.0, z-1.0);
        let n100 = self.grad_p[x0+1+self.perm[y0+  self.perm[z0  ]]].dot3(x-1.0,   y,   z);
        let n101 = self.grad_p[x0+1+self.perm[y0+  self.perm[z0+1]]].dot3(x-1.0,   y, z-1.0);
        let n110 = self.grad_p[x0+1+self.perm[y0+1+self.perm[z0  ]]].dot3(x-1.0, y-1.0,   z);
        let n111 = self.grad_p[x0+1+self.perm[y0+1+self.perm[z0+1]]].dot3(x-1.0, y-1.0, z-1.0);


        let u = fade(x);
        let v = fade(y);
        let w = fade(z);
        lerp(
            lerp(
              lerp(n000, n100, u),
              lerp(n001, n101, u), w),
            lerp(
              lerp(n010, n110, u),
              lerp(n011, n111, u), w),
           v);
    }

    
    
    pub fn  simplex2(&self,xin:f64,yin:f64) ->f64{   
    
        let f2 = 0.5*(3.0_f64.sqrt()-1.0);
        let g2 = (3.0-3.0_f64.sqrt())/6.0;
        let s = (xin+yin)*f2; // Hairy factor for 2D

        let i  = (xin+s).floor();
        let j = (yin+s).floor();

        let t = (i+j)*g2;

        let x0 = xin-i+t; // The x,y distances from the cell origin, unskewed.
        let y0 = yin-j+t;
        // For the 2D case, the simplex shape is an equilateral triangle.
        // Determine which simplex we are in.

        let mut  i1:usize=0;
        let mut j1:usize =0; // Offsets for second (middle) corner of simplex in (i,j) coords

        if x0>y0 { // lower triangle, XY order: (0,0)->(1,0)->(1,1)
          i1=1; j1=0;
        } else {    // upper triangle, YX order: (0,0)->(0,1)->(1,1)
          i1=0; j1=1;
        }
        // A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
        // a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
        // c = (3-sqrt(3))/6
        let x1 = x0 - i1 as f64 + g2; // Offsets for middle corner in (x,y) unskewed coords
        let y1 = y0 - j1 as f64+ g2;
        let x2 = x0 - 1.0 + 2.0 * g2; // Offsets for last corner in (x,y) unskewed coords
        let y2 = y0 - 1.0 + 2.0 * g2;

        // Work out the hashed gradient indices of the three simplex corners
        let i =i as usize & 255;
        let j =j as usize & 255;

        let gi0 = self.grad_p[i+self.perm[j]].clone();
        let gi1 = self.grad_p[i+i1 as usize+self.perm[j+j1 as usize]];
        let gi2 = self.grad_p[i+1+self.perm[j+1]].clone();
        // Calculate the contribution from the three corners
        let mut t0 = 0.5 - x0*x0-y0*y0;


        let mut n0=0.0;
        let mut n1=0.0;
        let mut n2=0.0; // Noise contributions from the three corners


        if t0<0.0 {
          n0 = 0.0;
        } else {
          t0 *= t0;
          n0 = t0 * t0 * gi0.dot2(x0, y0);  // (x,y) of grad3 used for 2D gradient
        }
        let mut t1 = 0.5 - x1*x1-y1*y1;
        if t1<0.0 {
          n1 = 0.0;
        } else {
          t1 *= t1;
          n1 = t1 * t1 * gi1.dot2(x1, y1);
        }
        let mut t2 = 0.5 - x2*x2-y2*y2;
        if t2<0.0 {
          n2 = 0.0;
        } else {
          t2 *= t2;
          n2 = t2 * t2 * gi2.dot2(x2, y2);
        }
        // Add contributions from each corner to get the final noise value.
        // The result is scaled to return values in the interval [-1,1].
        70.0 * (n0 + n1 + n2)
    }
    pub fn  simplex3() {
        //todo
        
    }

}
fn fade(t:f64) ->f64{
    t*t*t*(t*(t*6.0-15.0)+10.0)
}

// Linear Interpolate
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0-t)*a + t*b
}


// Fade function as defined by Ken Perlin.  This eases coordinate values
// so that they will "ease" towards integral values.  This ends up smoothing
// the final output.
