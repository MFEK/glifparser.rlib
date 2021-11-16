use skia_safe as skia;
use skia::{Point as SkPoint, Path as SkPath};
use skia::path::{Verb as SkVerb};

use crate::point::{Handle, Point, PointData, PointType};
use crate::{Contour, Outline};

use super::QuadsToCubics;

/// Get an outline from a Skia path. Outline is guaranteed to contain only Curve's, no QCurve's.
pub trait FromSkiaPath {
    fn from_skia_path(skp: &skia::Path) -> Self;
}

// These types should never be used outside this file and are only shorthand for FromSkiaPath
type SkPointTuple = (PointType, Vec<SkPoint>, Option<f32>);
type SkContour = Vec<SkPointTuple>;
type SkOutline = Vec<SkContour>;

/// Stability note: This is a complex conversion. I documented it as best I could, and left around
/// old debug eprintln's in case you find a broken case. It's quite complicated and takes multiple
/// passes because Skia path's can contain conics and quads, both of which we want to upconvert to
/// cubics.
impl<PD: PointData> FromSkiaPath for Outline<PD> {
    fn from_skia_path(skp: &skia::Path) -> Outline<PD> {
        // These are iterators over (Verb, Vec<skia_safe::Point>)
        // We need two of them because the for loop consumes `iter`, meaning we can't get the conic
        // weight from it. This is ugly, I know, but converting C API's to Rust often are.
        let iter = skia::path::Iter::new(&skp, false);
        let mut iter2 = skia::path::Iter::new(&skp, false);

        let mut skoutline: SkOutline = vec![];
        let mut skcontour: SkContour = vec![];

        // First we're going to split a path into its constituent contours
        for p in iter {
            iter2.next();
            let (verb, point) = p;
            let conic_weight = iter2.conic_weight();
            // This lets us know if the QCurve needs conversion
            let mut holding_conic = false;
            let ptype = match verb {
                SkVerb::Move => PointType::Move,
                SkVerb::Cubic => PointType::Curve,
                SkVerb::Quad => PointType::QCurve,
                SkVerb::Line => PointType::Line,
                SkVerb::Conic => {holding_conic = true; PointType::QCurve},
                // We call these "off curve" simply because it's a free PointType, to test against
                // later. Bit of a hack.
                SkVerb::Close => PointType::OffCurve,
                SkVerb::Done => { break }
            };
            if ptype == PointType::Move {
                if skcontour.len() > 0 {
                    skoutline.push(skcontour.clone());
                }
                skcontour = vec![];
            }
            //eprintln!("Verb: {:?}, Point: {:?}, Conic Weight: {:?}", verb, &point, conic_weight);
            if ptype != PointType::OffCurve {
                skcontour.push((ptype, point, if holding_conic { Some(conic_weight.unwrap()) } else { None }));
            } else {
                debug_assert!(skcontour[0].0 == PointType::Move);
                skcontour.remove(0);
                // drop "off curve" point
            }
        }

        if skcontour.len() > 0 {
            skoutline.push(skcontour);
        }

        // The path could contain conics, so our next task is to resolve conics to quads
        for skc in skoutline.iter_mut() {
            // skp: &mut (PointType, Vec<skia::Point>, Option<f32>)
            for skp in skc.iter_mut() {
                //  (&mut PointType, &mut Vec<skia::Point>, &mut Option<f32>)
                let (_, points, conic_weight) = skp;
                if let Some(cw) = conic_weight {
                    debug_assert_eq!(points.len(), 3);
                    // magic number 5 for pow2 1 from https://fiddle.skia.org/c/@Path_ConvertConicToQuads
                    let mut new_points = vec![SkPoint::default(); 5];
                    //               convert_conic_to_quads(p0: impl Into<Point>, p1: impl Into<Point>, p2: impl Into<Point>, 
                    //                                      w: scalar, pts: &mut [Point], pow2: usize) -> Option<usize>
                    let ok = SkPath::convert_conic_to_quads(points[0], points[1], points[2], 
                                                            *cw, &mut new_points, 1);
                    //      number of quad bezier's == 2
                    debug_assert!(ok.is_some() && ok.unwrap() == 2);
                    *points = new_points;
                    // We do this to mark the points as changed, so we can panic if it's ever not
                    // 1.
                    *conic_weight = Some(1.);
                }
            }
        }

        // Now we have to change the converted conics to two quads, to match other curves that
        // might be the result of quad_to as opposed to conic_to.
        let mut final_skoutline: SkOutline = vec![];

        for skc in skoutline {
            let mut skcontour: SkContour = vec![];
            for skp in skc {
                let (ptype, points, conic_weight) = skp;
                if ptype == PointType::QCurve {
                    // same magic number as above. we always use a pow2 of 1 to convert conics to
                    // quads, so we get 5 points
                    debug_assert!(points.len() == 3 || points.len() == 5);

                    // We don't want any quads, despite all the work we've done to get them. Skia
                    // doesn't have a function like convert_conic_to_cubics, only to_quads, so now
                    // that we have the quads we can make cubics.
                    skcontour.push((PointType::Curve, [points[0], points[1], points[2]].quads_to_cubics().to_vec(), None));
                    if points.len() == 5 {
                        skcontour.push((PointType::Curve, [points[2], points[3], points[4]].quads_to_cubics().to_vec(), None));
                    }
                } else {
                    skcontour.push((ptype, points, conic_weight));
                }
            }
            final_skoutline.push(skcontour);
        }

        // Now we know that we have no conics, and no quads. In Skia terms, we only have the verbs
        // Move, Line, Cubic, Close, and Done.
        //
        // We can now convert to a cubic Bézier-backed glifparser Outline.
        let mut ret: Outline<PD> = Outline::new();

        //eprintln!("{:#?}", &final_skoutline);

        for skc in final_skoutline.iter_mut() {
            let mut contour: Contour<PD> = Contour::new();
            let first_points: &SkPointTuple = &skc[0];
            let mut prev_points;
            let skc_len = skc.len();
            for (i, skp) in skc.iter().enumerate() {
                let (ptype, points, _) = skp;

                if i != 0 {
                    prev_points = &skc[i-1];
                } else {
                    prev_points = &skc[skc_len - 1];
                }

                let mut point = Point::<PD> {
                    name: None,
                    data: None,
                    x: points[0].x,
                    y: points[0].y,
                    // These will be fixed below, if needed
                    a: Handle::Colocated,
                    b: Handle::Colocated,
                    ptype: *ptype,
                };

                match ptype {
                    PointType::Move => {},
                    PointType::Curve => {
                        point.a = Handle::At(points[1].x, points[1].y);
                        if prev_points.0 == PointType::Curve {
                            point.b = Handle::At(prev_points.1[2].x, prev_points.1[2].y);
                        }

                        if i == skc_len-1 {
                            if first_points.0 == PointType::Curve {
                                contour[0].a = Handle::At(first_points.1[1].x, first_points.1[1].y);
                            }
                            contour[0].b = Handle::At(points[2].x, points[2].y);
                        }
                    },
                    PointType::Line => {
                        if i != 0 && prev_points.0 == PointType::Curve {
                            // Lines aren't allowed to have off-curve points in glif format
                            point.ptype = PointType::Curve;
                            point.b = Handle::At(prev_points.1[2].x, prev_points.1[2].y);
                        }
                    },
                    _ => unreachable!("")
                }
                contour.push(point);
            }

            // Skia doesn't put a last point like we expect for open contours
            match (contour.first(), contour.last()) {
                (Some(first), Some(last)) => {
                    if first.ptype == PointType::Move && (last.ptype == PointType::Curve || last.ptype == PointType::Line) {
                        let ptype = match skc[skc_len-1].1.len() {
                            2 => PointType::Line,
                            4 => PointType::Curve,
                            _ => {unreachable!()}
                        };
                        let p = match ptype {
                            PointType::Line => {
                                skc[skc_len-1].1[1]
                            },
                            PointType::Curve => {
                                skc[skc_len-1].1[3]
                            },
                            _ => {unreachable!()}
                        };
                        let mut glifl: Point<PD> = Point::from_x_y_type((p.x, p.y), ptype);
                        if ptype == PointType::Curve {
                            let h_prev = skc[skc_len-1].1[2];
                            if p != h_prev {
                                glifl.b = Handle::At(h_prev.x, h_prev.y);
                            }
                        }
                        contour.push(glifl);
                    }
                },
                _ => {}
            }

            // Skia adds Move followed by Line at same spot, .glif format only needs the Move
            match (contour.first(), contour.len()) {
                (Some(first), 2..) => {
                    if first.ptype == PointType::Move {
                        if contour[1].x == first.x && contour[1].y == first.y {
                            contour[0].a = contour[1].a;
                            contour[0].b = contour[1].b;
                            contour.remove(1);
                        }
                    }
                },
                _ => ()
            }

            ret.push(contour);
        }

        //eprintln!("{:#?}", &ret);

        ret
    }
}
