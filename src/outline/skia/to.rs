use skia_safe as skia;

use crate::point::{Point, PointData, PointType, WhichHandle};
use crate::{Contour, Outline, OutlineType};

/// glifparser returns for you two Skia paths when called on an outline, because it is extremely
/// liekly that you are going to want to draw open paths in a different way than you draw closed
/// ones. `Option`'s are used because it's possible, and most likely, that the Outline will only
/// contain paths of one type.
#[derive(Clone, PartialEq)]
pub struct SkiaPaths {
    pub open: Option<skia::Path>,
    pub closed: Option<skia::Path>,
}

impl SkiaPaths {
    pub fn combined(&self) -> skia::Path {
        let mut combined = skia::Path::new();
        self.open.as_ref().map(|p|combined.add_path(&p, (0., 0.), skia::path::AddPathMode::Append));
        self.closed.as_ref().map(|p|combined.add_path(&p, (0., 0.), skia::path::AddPathMode::Append));
        combined
    }
}

impl Into<skia::Path> for SkiaPaths {
    fn into(self) -> skia::Path {
        let mut ret = skia::Path::new();
        self.open.as_ref().map(|skpath|ret.add_path(&skpath, (0., 0.), skia::path::AddPathMode::Append));
        self.closed.as_ref().map(|skpath|ret.add_path(&skpath, (0., 0.), skia::path::AddPathMode::Append));
        ret
    }
}

#[derive(Copy, Clone)]
pub struct SkiaPointTransforms {
    pub calc_x: fn(f32) -> f32,
    pub calc_y: fn(f32) -> f32,
}

impl SkiaPointTransforms {
    pub fn new() -> Self {
        Self {
            calc_x: |f|f,
            calc_y: |f|f
        }
    }
}

pub trait ToSkiaPath {
    fn to_skia_path(&self, spt: Option<SkiaPointTransforms>) -> Option<skia::Path>;
}

pub trait ToSkiaPaths {
    fn to_skia_paths(&self, spt: Option<SkiaPointTransforms>) -> SkiaPaths;
}

impl<PD: PointData> ToSkiaPaths for Outline<PD> {
    fn to_skia_paths(&self, spt: Option<SkiaPointTransforms>) -> SkiaPaths {
        let mut ret = SkiaPaths {
            open: None,
            closed: None
        };

        let mut open = skia::Path::new();
        let mut closed = skia::Path::new();

        for contour in self {
            let firstpoint: &Point<PD> = match contour.first() {
                Some(p) => p,
                None => { continue } // contour has no points
            };
            let skpath = contour.to_skia_path(spt).unwrap(); // therefore we know it'll be Some
            if firstpoint.ptype == PointType::Move {
                &mut open
            } else {
                &mut closed
            }.add_path(&skpath, (0., 0.), skia::path::AddPathMode::Append);
        }

        if open.count_points() > 0 {
            ret.open = Some(open);
        }

        if closed.count_points() > 0 {
            ret.closed = Some(closed);
        }

        ret
    }
}

impl<PD: PointData> ToSkiaPath for Contour<PD> {
    fn to_skia_path(&self, spt: Option<SkiaPointTransforms>) -> Option<skia::Path> {
        if self.len() == 0 {
            return None;
        }

        let mut path = skia::Path::new();
        let firstpoint: &Point<PD> = self.first().unwrap();
        let mut prevpoint: &Point<PD> = self.first().unwrap();
        let mut outline_type = OutlineType::Cubic;

        let transforms = spt.unwrap_or(SkiaPointTransforms::new());
        let calc_x = transforms.calc_x;
        let calc_y = transforms.calc_y;

        path.move_to((calc_x(self[0].x), calc_y(self[0].y)));

        for (i, point) in self.iter().enumerate() {
            // the move_to handles the first point
            if i == 0 {
                continue;
            };
            match point.ptype {
                PointType::Line => {
                    path.line_to((calc_x(point.x), calc_y(point.y)));
                }
                PointType::Curve => {
                    if outline_type == OutlineType::Quadratic {
                        panic!("Got a cubic point after a quadratic point");
                    }
                    let h1 = prevpoint.handle_or_colocated(WhichHandle::A, calc_x, calc_y);
                    let h2 = point.handle_or_colocated(WhichHandle::B, calc_x, calc_y);
                    path.cubic_to(h1, h2, (calc_x(point.x), calc_y(point.y)));
                }
                PointType::QCurve => {
                    outline_type = OutlineType::Quadratic;
                    let h1 = prevpoint.handle_or_colocated(WhichHandle::A, calc_x, calc_y);
                    path.quad_to(h1, (calc_x(point.x), calc_y(point.y)));
                }
                _ => {}
            }
            prevpoint = &point;
        }

        if firstpoint.ptype != PointType::Move {
            match self.last() {
                Some(lastpoint) => {
                    let h1 = lastpoint.handle_or_colocated(WhichHandle::A, calc_x, calc_y);
                    match outline_type {
                        OutlineType::Cubic => {
                            let h2 = firstpoint.handle_or_colocated(WhichHandle::B, calc_x, calc_y);
                            path.cubic_to(h1, h2, (calc_x(firstpoint.x), calc_y(firstpoint.y)))
                        }
                        OutlineType::Quadratic => {
                            match lastpoint.ptype {
                                PointType::QClose => {
                                    // This is safe as a lone QClose is illegal and should
                                    // cause a crash anyway if it's happening.
                                    let prevpoint = &self[self.len() - 2];
                                    let ph =
                                        prevpoint.handle_or_colocated(WhichHandle::A, calc_x, calc_y);
                                    path.quad_to(ph, h1)
                                }
                                _ => path.quad_to(h1, (calc_x(firstpoint.x), calc_y(firstpoint.y))),
                            }
                        }
                        OutlineType::Spiro => panic!("Spiro as yet unimplemented."),
                    };
                }
                None => {}
            }
            path.close();
        }

        Some(path)
    }
}
