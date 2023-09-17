use crate::{util::Point, modnar::Modnar};
use std::ops::Deref;

trait PointOps {
    fn r_f(&self) -> f32;
    fn rsq_f(&self) -> f32;
    fn theta_f(&self) -> f32;
    fn phi_f(&self) -> f32;
}

impl PointOps for Point {
    fn r_f(&self) -> f32 {
        self.dot(self.deref()).sqrt()
    }

    fn rsq_f(&self) -> f32 {
        self.dot(self.deref())
    }

    fn theta_f(&self) -> f32 {
        (self.x / (self.y + f32::EPSILON)).atan()
    }

    fn phi_f(&self) -> f32 {
        (self.y / (self.x + f32::EPSILON)).atan()
    }
}

#[inline(always)]
fn phi(p: &Point) -> f32 {
    (p.y / (p.x + f32::EPSILON)).atan()
}

#[inline(always)]
fn psi(rnd: &mut Modnar) -> f32 {
    rnd.gen_f32()
}

#[inline(always)]
fn omega(rnd: &mut Modnar) -> f32 {
    if rnd.gen_f32() > 0.5 { std::f32::consts::PI } else { 0.0 }
}

#[inline(always)]
fn lambda(rnd: &mut Modnar) -> f32 {
    if rnd.gen_f32() < 0.5 { -1.0 } else { 1.0 }
}

#[inline(always)]
fn sinus(p: &Point) -> Point {
    Point::new(p.x.sin(), p.y.sin())
}

#[inline(always)]
fn spherical(p: &Point) -> Point {
    let rsq = p.rsq_f();
    Point::new(p.x/rsq, p.y/rsq)
}

#[inline(always)]
fn swirl(p: &Point) -> Point {
    let rsq = p.rsq_f();
    Point::new(p.x*rsq.sin() - p.y*rsq.cos(), p.x*rsq.cos() + p.y*rsq.sin())
}

#[inline(always)]
fn horseshoe(p: &Point) -> Point { 
    Point::new(
        (p.x -p.y)*(p.x+p.y)/p.r_f(),
        2.0*p.x*p.y/p.r_f()
    )
}

#[inline(always)]
fn polar(p: &Point) -> Point {
    Point::new(
        p.theta_f() / std::f32::consts::PI,
        p.r_f() - 1.0
    )
}

#[inline(always)]
fn handkerchief(p: &Point) -> Point { 
    Point::new(
        p.r_f()*((p.theta_f() + p.r_f()).sin()),
        p.r_f()*(p.theta_f() - p.r_f().cos())
    )
}

macro_rules! theta {
    ($p:ident, $var:ident) => {
        let $var = $p.theta_f();
    };
}

macro_rules! r {
    ($p:ident, $var:ident) => {
        let $var = $p.r_f();
    };
}

macro_rules! rsq {
    ($p:ident, $var:ident) => {
        let $var = $p.rsq_f();
    };
}

macro_rules! pi {
    ($var:ident) => {
        let $var = std::f32::consts::PI;
    };
}

macro_rules! psi {
    ($rnd:ident, $var:ident) => {
        let $var = psi($rnd);
    };
}

macro_rules! lambda {
    ($rnd:ident, $var:ident) => {
        let $var = lambda($rnd);
    };
}

macro_rules! omega {
    ($rnd:ident, $var:ident) => {
        let $var = omega($rnd);
    };
}

macro_rules! phi {
    ($p:ident, $var:ident) => {
        let $var = phi($p);
    };
}

fn sin(val: f32) -> f32 { val.sin() }
fn cos(val: f32) -> f32 { val.cos() }
fn sqrt(val: f32) -> f32 { val.sqrt() }
fn tan(val: f32) -> f32 { val.tan() }

fn heart(p: &Point) -> Point {
    theta!(p, theta);
    Point::new(
        p.r_f()*((theta*p.r_f()).sin()),
        -p.r_f()*((theta*p.r_f()).cos())
    )
}

#[inline(always)]
fn disc(p: &Point) -> Point {
    r!(p, r);
    theta!(p, theta);
    pi!(pi);

    Point::new(
        theta / pi * sin(pi*r),
        theta/pi*cos(pi*r)
    )
}

#[inline(always)]
fn spiral(p: &Point) -> Point { 
    theta!(p, theta);
    r!(p, r);

    let rr = 1.0/r;
    Point::new(
        rr*(cos(theta) + sin(r)),
        rr*(sin(theta) - cos(r))
    )
}

#[inline(always)]
fn hyperbolic(p: &Point) -> Point {  
    theta!(p, theta);
    r!(p, r);

    let mul = if p.y > 0.0 { 1.0 } else { -1.0 };

    Point::new(
        sin(theta)/r,
        mul*r*cos(theta)
    )
}

#[inline(always)]
fn diamond(p: &Point) -> Point {  
    theta!(p, theta);
    r!(p, r);

    Point::new(
        sin(theta)*cos(r),
        cos(theta)*sin(r)
    )
}

#[inline(always)]
fn ex(p: &Point) -> Point { 
    theta!(p, theta);
    r!(p, r);

    let p0 = sin(theta+r);
    let p1 = cos(theta-r);

    Point::new(
        r*(p0*p0*p0+p1*p1*p1),
        r*(p0*p0*p0-p1*p1*p1)
    )
}

#[inline(always)]
fn julia(p: &Point, rnd: &mut Modnar) -> Point { 
    theta!(p, theta);
    r!(p, r);

    let omega = omega(rnd);

    Point::new(
        r.sqrt()*cos(theta/2.0+omega),
        r.sqrt()*sin(theta/2.0+omega)
    )
}

#[inline(always)]
fn bent(p: &Point) -> Point { 
    if p.x < 0.0 && p.y >= 0.0 {
        return Point::new(2.0*p.x, p.y);
    }

    if p.x >= 0.0 && p.y < 0.0 {
        return Point::new(p.x, p.y / 2.0);
    }

    if p.x < 0.0 && p.y < 0.0 {
        return Point::new(2.0*p.x, p.y / 2.0);
    }

    return Point::new(p.x, p.y);
} 

#[inline(always)]
fn waves(p: &Point, b: f32, c: f32, e: f32, f: f32) -> Point {
    return Point::new(
        p.x + b*sin(p.y/(c*c)),
        p.y + e*sin(p.x/(f*f))
    );
}

#[inline(always)]
fn fisheye(p: &Point) -> Point { 
    r!(p, r);
    let r = 2.0/(r + 1.0);
    Point::new(
        r*p.y,
        r*p.x
    )
}

#[inline(always)]
fn popcorn(p: &Point, c: f32, f: f32) -> Point {
    Point::new(
        p.x+c*sin(tan(3.0*p.y)),
        p.y+f*sin(tan(3.0*p.x))
    )
}

#[inline(always)]
fn exponential(p: &Point) -> Point { 
    let e = (p.x - 1.0).exp();
    pi!(pi);

    Point::new(
        e*cos(pi*p.y),
        e*sin(pi*p.y)
    )
}

#[inline(always)]
fn power(p: &Point) -> Point { 
    theta!(p, theta);
    let r = p.r_f().powf(sin(theta));

    Point::new(
        r*cos(theta),
        r*sin(theta)
    )
}

fn sinh(val: f32) -> f32 { val.sinh() }
fn cosh(val: f32) -> f32 { val.cosh() }

#[inline(always)]
fn cosine(p: &Point) -> Point { 
    pi!(pi);

    Point::new(
        cos(pi*p.x)*cosh(p.y),
        -sin(pi*p.x)*sinh(p.y)
    )
}

fn fmod(val: f32, val1: f32) -> f32 { val % val1 }

#[inline(always)]
fn rings(p: &Point, c: f32) -> Point { 
    theta!(p, t);
    r!(p, r);
    let com = fmod(r+c*c, 2.0*c*c) - c*c+r*(1.0-c*c);

    Point::new(
        cos(t)*com,
        sin(t)*com
    )
}

#[inline(always)]
fn fan(p: &Point, c: f32, f: f32) -> Point {  
    pi!(pi);
    theta!(p, th);
    r!(p, r);
    let t = pi*c*c;
    let v = fmod(th+f, t);

    if v > t/2.0 {
        Point::new(
            r*cos(th - t/2.0),
            r*sin(th - t/2.0)
        )
    } else {
        Point::new(
            r*cos(th + t/2.0),
            r*sin(th + t/2.0)
        )
    }
}

#[inline(always)]
fn blob(p: &Point, blob_h: f32, blob_l: f32, blob_waves: f32) -> Point { 
    theta!(p, theta);
    r!(p, r);

    let v = r*(blob_l+((blob_h - blob_l)/2.0)*(sin(theta*blob_waves) + 1.0));

    Point::new(
        cos(theta) * v,
        sin(theta) * v
    )
}

#[inline(always)]
fn pdj(p: &Point, pdj_a: f32, pdj_b: f32, pdj_c: f32, pdj_d: f32) -> Point {  
    Point::new(
        sin(pdj_a*p.y) - cos(pdj_b*p.x),
        sin(pdj_c*p.x) - cos(pdj_d*p.y)
    )
}

fn trunc(val: f32) -> f32 {  val.trunc() }

#[inline(always)]
fn fan2(p: &Point, fx: f32, fy: f32) -> Point {
    theta!(p, theta);
    r!(p, r);
    pi!(pi);

    let p1 = pi*(fx*fx);
    let p2 = fy;

    let t = theta+p2-p1*trunc(2.0*theta*p2/p1);

    if t > p1/2.0 {
        Point::new(
            sin(theta - p1/2.0) * r,
            cos(theta - p1/2.0) * r
        )
    } else {
        Point::new(
            sin(theta + p1/2.0) * r,
            cos(theta + p1/2.0) * r
        )
    }
}  

#[inline(always)]
fn rings2(p: &Point, rings2_val: f32) -> Point { 
    theta!(p, theta);
    r!(p, r);

    let p1 = rings2_val*rings2_val;
    let t = r - 2.0*p1*trunc((r + p1)/(2.0*p1)) + r*(1.0 - p1);

    Point::new(
        sin(theta) * t,
        cos(theta) * t
    )
}

#[inline(always)]
fn eyefish(p: &Point) -> Point { 
    r!(p, r);

    Point::new(
        p.x*(2.0/(r+1.0)),
        p.y*(2.0/(r+1.0))
    )
}

#[inline(always)]
fn bubble(p: &Point) -> Point { 
    rsq!(p, rsq);

    Point::new(
        p.x*(4.0/(rsq + 4.0)),
        p.y*(4.0/(rsq + 4.0))
    )
}

#[inline(always)]
fn cylinder(p: &Point) -> Point { 
    Point::new(
        sin(p.x),
        p.y
    )
}

#[inline(always)]
fn perspective(p: &Point, p1_angle: f32, p2_dist: f32) -> Point { 
    let common = p2_dist/(p2_dist-p.y*sin(p1_angle));

    Point::new(
        p.x*common,
        common*cos(p1_angle)
    )
}

#[inline(always)]
fn noise(p: &Point, rnd: &mut Modnar) -> Point { 
    pi!(pi);
    let psi1 = psi(rnd);
    let psi2 = psi(rnd);

    Point::new(
        psi1*p.x*cos(2.0*pi*psi2),
        p.y*sin(2.0*pi*psi2)
    )
} 

#[inline(always)]
fn julian(p: &Point, rnd: &mut Modnar, power: f32, dist: f32) -> Point { 
    psi!(rnd, psi);
    phi!(p, phi);
    r!(p, r);
    pi!(pi);

    let p3 = trunc(power.abs()*psi);
    let t = (phi + 2.0*pi*p3)/power;

    Point::new(
        cos(t)*r.powf(dist / power),
        sin(t)*r.powf(dist / power)
    )
} 

#[inline(always)]
fn julias(p: &Point, rnd: &mut Modnar, power: f32, dist: f32) -> Point { 
    psi!(rnd, psi);
    phi!(p, phi);
    r!(p, r);
    pi!(pi);
    lambda!(rnd, lambda);

    let p3 = trunc(power.abs()*psi);
    let t = (phi*lambda + 2.0*pi*p3)/power;

    Point::new(
        cos(t)*r.powf(dist / power),
        sin(t)*r.powf(dist / power)
    )
}

#[inline(always)]
fn blur(p: &Point, rnd: &mut Modnar) -> Point { 
    psi!(rnd, psi);
    psi!(rnd, psi2);
    pi!(pi);

    Point::new(
        cos(2.0*pi*psi2) * psi,
        sin(2.0*pi*psi2) * psi
    )
}

#[inline(always)]
fn gaussian(p: &Point, rnd: &mut Modnar) -> Point { 
    pi!(pi);
    psi!(rnd, psi);
    psi!(rnd, psi2);
    psi!(rnd, psi3);
    psi!(rnd, psi4);
    psi!(rnd, psi5);

    let sum = (psi+psi2+psi3+psi4) - 2.0; 

    Point::new(
        cos(2.0*pi*psi5) * sum,
        sin(2.0*pi*psi5) * sum
    )   
}

/*

static inline void radian_blur A, float angle, float v36 B {
    PSI
    PSI2
    PSI3
    PSI4
    PHI
    R
    X_Y
    float t1 = v36*((psi+psi2+psi3+psi4)-2);
    float t2 = phi+t1;
    float t3 = t1*cos(angle) - 1;

    PF2M(r*cos(t2)+t3*x, r*sin(t2)+t3*y, (1/v36))
}

static inline void pie A, float slices, float rotation, float thickness B {
    PSI
    PSI2
    PSI3

    float t1 = trunc(psi*slices+0.5);
    float t2 = rotation +(2*pi/slices)*(t1 + psi2*thickness);

    PF2M(cos(t2), sin(t2), psi3)
}

static inline void ngon A, float power, float sides, float corners, float circle B {
    PHI
    R
    float t3 = phi - sides*floor(phi/sides);

    float t4 = t3 > sides/2 ? t3 : t3 - sides;

    float k = (corners*(1/cos(t4)-1)+circle) / (pow(r, power) + FLT_EPSILON);

    PF2M(p->x, p->y, k)
}

static inline void curl A, float c1, float c2 B {
    X_Y
    float t1 = 1+c1*x+c2*(x*x-y*y);
    float t2 = c1*y+2*c2*x*y;

    PF2M(x*t1+y*t2, y*t1 - x*t2, 1/(t1*t1+t2*t2))
}

static inline void rectangles A, float rect_x, float rect_y B {
    X_Y
    PF2((2*floor(x/rect_x)+1)*rect_x - x, (2*floor(y/rect_y)+1)*rect_y - y)
}

static inline void arch A, float v41 B {
    PSI
    PF2(sin(psi*pi*v41), sin(psi*pi*v41)*sin(psi*pi*v41)/cos(psi*pi*v41))
}

static inline void tangent A B {
    X_Y
    PF2(sin(x)/cos(y), tan(y))
}

static inline void square A B {
    PSI
    PSI2

    PF2(psi - 0.5, psi2 - 0.5)
}

static inline void rays A, float v44 B {
    PSI
    X_Y
    RSQ

    float m = (v44*tan(psi*v44))/rsq;
    PF2M(cos(x), sin(y), m)
}

static inline void blade A, float v45 B {
    PSI
    R
    X

    PF2M(cos(psi*r*v45)+sin(psi*r*v45), cos(psi*r*v45) - sin(psi*r*v45), x)
}

static inline void secant A, float v46 B {
    R
    X

    PF2(x, 1/(v46*cos(v46*r)))
}

static inline void twintrian A, float v47 B {
    X

    float psi = psi_f(e_source);

    float t = log10(pow(sin(psi*r_f(p)*v47), 2) + cos(psi*r_f(p)*v47));

    PF2(x*t, x*(t - pi*sin(psi*r_f(p)*v47)))
}

static inline void cross A B {
    X_Y
    float v = sqrt(1/pow((x*x-y*y),2));
    PF2M(x, y, v)
}
*/