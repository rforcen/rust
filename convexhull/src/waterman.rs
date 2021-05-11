/*
    Waterman poly
*/
#![allow(dead_code)]

pub fn gen_waterman_poly(radius: f64) -> Vec<[f64; 3]> {
    let mut coords = vec![];

    let (a, b, c) = (0.0f64, 0.0f64, 0.0f64);
    let (mut max, mut min) = (-f64::MAX, f64::MAX);
    let mut s = radius; // .sqrt();
    let radius2 = s * s;

    let (xra, xrb) = ((a - s).ceil(), (a + s).floor());

    let mut x = xra;
    while x <= xrb {
        let r = radius2 - (x - a) * (x - a);
        if r < 0. {
            x += 1.;
            continue;
        }
        s = r.sqrt();
        let yra = (b - s).ceil();
        let yrb = (b + s).floor();
        let mut y = yra;

        let (mut zra, mut zrb): (f64, f64);

        while y <= yrb {
            let ry = r - (y - b) * (y - b);
            if ry < 0. {
                y += 1.;
                continue;
            } //case ry < 0

            if ry == 0. && c == c.floor() {
                //case ry=0
                if ((x + y + c) % 2.) != 0. {
                    y += 1.;
                    continue;
                } else {
                    zra = c;
                    zrb = c;
                }
            } else {
                // case ry > 0
                s = ry.sqrt();
                zra = (c - s).ceil();
                zrb = (c + s).floor();
                if ((x + y) % 2.) == 0. {
                    if (zra % 2.) != 0. {
                        if zra <= c {
                            zra = zra + 1.
                        } else {
                            zra = zra - 1.
                        }
                    }
                } else {
                    if zra % 2. == 0. {
                        if zra <= c {
                            zra = zra + 1.
                        } else {
                            zra = zra - 1.
                        }
                    }
                }
            }

            let mut z = zra;
            while z <= zrb {
                // save vertex x,y,z
                max = max.max(z).max(y).max(x);
                min = min.min(z).min(y).min(y);

                coords.push([x, y, z]);
                z += 2.
            }

            y += 1.;
        }

        x += 1.;
    }

    // let dif = (max - min).abs(); // scale
    // if dif > 0. {
    //     coords = coords
    //         .iter()
    //         .map(|c| [c[0] / dif, c[1] / dif, c[2] / dif])
    //         .collect::<Vec<_>>();
    // }
    coords
}

pub fn gen_waterman_flat(rad: f64) -> Vec<f64> {
    gen_waterman_poly(rad)
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<f64>>()
}
