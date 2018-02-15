use std::env;
#[derive(Debug)]
enum Token {
    Element(Element),
    Count(u32)
}
fn main() {
    let name = env::args().skip(1).next().unwrap();
    let mut tokens = Vec::new();
    let mut token_range = 0..1;
    for (i, c) in name.chars().enumerate().skip(1) {
        let split_at = (c.is_uppercase() || c.is_digit(10)) && !(c == '.');
        if split_at {
            if let Some(element) = Element::from_str(&name[token_range.clone()]) {
                tokens.push(Token::Element(element));
            } else if let Ok(num) = u32::from_str_radix(&name[token_range.clone()], 10) {
                tokens.push(Token::Count(num))
            } else {
                panic!("Bad molecule name {}", name);
            }
            token_range = i..i+1;
        } else {
            token_range.end += 1;
        }
    }
    if let Some(element) = Element::from_str(&name[token_range.clone()]) {
        tokens.push(Token::Element(element));
    } else if let Ok(num) = u32::from_str_radix(&name[token_range.clone()], 10) {
        tokens.push(Token::Count(num))
    } else {
        panic!("Bad molecule name");
    }

    let mut active_element: Option<Element> = None;
    let mut active_multiplier = None;
    let mut mass = 0.0;
    for token in tokens {
        match token {
            Token::Element(element) => {
                mass += active_element.map(|e| e.mass() * active_multiplier.unwrap_or(1.0)).unwrap_or(0.0);
                active_element = Some(element);
            },
            Token::Count(num) => active_multiplier = Some(num as f64)
        }
    }
    mass += active_element.map(|e| e.mass() * active_multiplier.unwrap_or(1.0)).unwrap_or(0.0);
    println!("{}", mass);
}

macro_rules! elements {
    ($($S:ident, $N:expr, $M:expr,)+) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Element {
            $($S = $N),+
        }

        impl Element {
            pub fn mass(self) -> f64 {
                match self {
                    $(Element::$S => $M as f64),+
                }
            }

            pub fn from_str(s: &str) -> Option<Element> {
                match s {
                    $(stringify!($S) => Some(Element::$S),)+
                    _ => None
                }
            }
        }
    }
}
elements!{
    H, 1, 1.008,
    He, 2, 4.0026,
    Li, 3, 6.94,
    Be, 4, 9.0122,
    B, 5, 10.81,
    C, 6, 12.011,
    N, 7, 14.007,
    O, 8, 15.999,
    F, 9, 18.998,
    Ne, 10, 20.180,
    Na, 11, 22.990,
    Mg, 12, 24.305,
    Al, 13, 26.982,
    Si, 14, 28.085,
    P, 15, 30.974,
    S, 16, 32.06,
    Cl, 17, 35.45,
    Ar, 18, 39.948,
    K, 19, 39.098,
    Ca, 20, 40.078,
    Sc, 21, 44.956,
    Ti, 22, 47.867,
    V, 23, 50.942,
    Cr, 24, 51.996,
    Mn, 25, 54.938,
    Fe, 26, 55.845,
    Co, 27, 58.933,
    Ni, 28, 58.693,
    Cu, 29, 63.546,
    Zn, 30, 65.38,
    Ga, 31, 69.723,
    Ge, 32, 72.630,
    As, 33, 74.922,
    Se, 34, 78.971,
    Br, 35, 79.904,
    Kr, 36, 83.798,
    Rb, 37, 85.468,
    Sr, 38, 87.62,
    Y, 39, 88.906,
    Zr, 40, 91.224,
    Nb, 41, 92.906,
    Mo, 42, 95.95,
    Tc, 43, (98),
    Ru, 44, 101.07,
    Rh, 45, 102.91,
    Pd, 46, 106.42,
    Ag, 47, 107.87,
    Cd, 48, 112.41,
    In, 49, 114.82,
    Sn, 50, 118.71,
    Sb, 51, 121.76,
    Te, 52, 127.60,
    I, 53, 126.90,
    Xe, 54, 131.29,
    Cs, 55, 132.91,
    Ba, 56, 137.33,
    La, 57, 138.91,
    Ce, 58, 140.12,
    Pr, 59, 140.91,
    Nd, 60, 144.24,
    Pm, 61, (145),
    Sm, 62, 150.36,
    Eu, 63, 151.96,
    Gd, 64, 157.25,
    Tb, 65, 158.93,
    Dy, 66, 162.50,
    Ho, 67, 164.93,
    Er, 68, 167.26,
    Tm, 69, 168.93,
    Yb, 70, 173.05,
    Lu, 71, 174.97,
    Hf, 72, 178.49,
    Ta, 73, 180.95,
    W, 74, 183.84,
    Re, 75, 186.21,
    Os, 76, 190.23,
    Ir, 77, 192.22,
    Pt, 78, 195.08,
    Au, 79, 196.97,
    Hg, 80, 200.59,
    Tl, 81, 204.38,
    Pb, 82, 207.2,
    Bi, 83, 208.98,
    Po, 84, (209),
    At, 85, (210),
    Rn, 86, (222),
    Fr, 87, (223),
    Ra, 88, (226),
    Ac, 89, (227),
    Th, 90, 232.04,
    Pa, 91, 231.04,
    U, 92, 238.03,
    Np, 93, (237),
    Pu, 94, (244),
    Am, 95, (243),
    Cm, 96, (247),
    Bk, 97, (247),
    Cf, 98, (251),
    Es, 99, (252),
    Fm, 100, (257),
    Md, 101, (258),
    No, 102, (259),
    Lr, 103, (266),
    Rf, 104, (267),
    Db, 105, (268),
    Sg, 106, (269),
    Bh, 107, (270),
    Hs, 108, (277),
    Mt, 109, (278),
    Ds, 110, (281),
    Rg, 111, (282),
    Cn, 112, (285),
    Nh, 113, (286),
    Fl, 114, (289),
    Mc, 115, (290),
    Lv, 116, (293),
    Ts, 117, (294),
    Og, 118, (294),
}
