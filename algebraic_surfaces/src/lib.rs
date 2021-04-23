// Parametric Surface
#![allow(dead_code)]

use nalgebra::{Point2, Point3, Vector3};
use std::f32::consts::PI;
use std::slice::Iter;

const TWO_PI: f32 = PI * 2.;

const N_SURFACES: usize = 27;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FuncNames {
    CAP,
    BOY,
    ROMAN,
    SEASHELL,
    TUDORROSE,
    BREATHER,
    KLEINBOTTLE,
    KLEINBOTTLE0,
    BOUR,
    DINI,
    ENNEPER,
    SCHERK,
    CONICALSPIRAL,
    BOHEMIANDOME,
    ASTROIDALELLIPSE,
    APPLE,
    AMMONITE,
    PLUCKERCONOID,
    CAYLEY,
    UPDOWNSHELL,
    BUTTERFLY,
    ROSE,
    KUEN,
    TANAKA0,
    TANAKA1,
    TANAKA2,
    TANAKA3,
}

static FUNC_NAMES: [FuncNames; N_SURFACES] = [
    FuncNames::CAP,
    FuncNames::BOY,
    FuncNames::ROMAN,
    FuncNames::SEASHELL,
    FuncNames::TUDORROSE,
    FuncNames::BREATHER,
    FuncNames::KLEINBOTTLE,
    FuncNames::KLEINBOTTLE0,
    FuncNames::BOUR,
    FuncNames::DINI,
    FuncNames::ENNEPER,
    FuncNames::SCHERK,
    FuncNames::CONICALSPIRAL,
    FuncNames::BOHEMIANDOME,
    FuncNames::ASTROIDALELLIPSE,
    FuncNames::APPLE,
    FuncNames::AMMONITE,
    FuncNames::PLUCKERCONOID,
    FuncNames::CAYLEY,
    FuncNames::UPDOWNSHELL,
    FuncNames::BUTTERFLY,
    FuncNames::ROSE,
    FuncNames::KUEN,
    FuncNames::TANAKA0,
    FuncNames::TANAKA1,
    FuncNames::TANAKA2,
    FuncNames::TANAKA3,
];

impl FuncNames {
    pub fn first() -> FuncNames {
        FuncNames::CAP
    }
    pub fn last() -> FuncNames {
        FuncNames::TANAKA3
    }
    pub fn next(&self) -> FuncNames {
        if *self != Self::last() {
            FUNC_NAMES[*self as usize + 1]
        } else {
            Self::first()
        }
    }
    pub fn prev(&self) -> FuncNames {
        if *self != Self::first() {
            FUNC_NAMES[*self as usize - 1]
        } else {
            Self::last()
        }
    }
    pub fn iterator() -> Iter<'static, FuncNames> {
        FUNC_NAMES.iter()
    }
    pub fn to_string(&self) -> String {
        static SURF_NAMES: [&str; N_SURFACES] = [
            "cap",
            "boy",
            "roman",
            "sea shell",
            "tudor rose",
            "breather",
            "klein bottle",
            "klein bottle 0",
            "bour",
            "dini",
            "enneper",
            "scherk",
            "conical spiral",
            "bohemian dome",
            "astrodial ellipse",
            "apple",
            "ammonite",
            "plucker comoid",
            "cayley",
            "up down shell",
            "butterfly",
            "rose",
            "kuen",
            "tanaka-0",
            "tanaka-1",
            "tanaka-2",
            "tanaka-3",
        ];
        SURF_NAMES[*self as usize].to_string()
    }
}

pub type Mesh = (
    Vec<Point3<f32>>,
    Vec<Vector3<f32>>,
    Vec<Point2<f32>>,
    Vec<Point3<u16>>,
);
type Range = (f32, f32);

pub const RANGES: [[(f32, f32); 2]; 27] = [
    [(0., PI), (0., PI)],
    [(0., PI), (0., PI)],
    [(0., 1.), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., PI), (0., PI)],
    [(-20., 20.), (20., 80.)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(-1., 1.), (-1., 1.)],
    [(1., 30.), (1., 30.)],
    [(0., 1.), (-1., 1.)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (-PI, PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(-2., 2.), (-1., 1.)],
    [(0., 3.), (0., TWO_PI)],
    [(-10., 10.), (-10., 10.)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(-4., 4.), (-3.75, 3.75)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
    [(0., TWO_PI), (0., TWO_PI)],
];
