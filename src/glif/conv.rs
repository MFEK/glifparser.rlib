use crate::glif::Glif;
use crate::glif::read::read_ufo_glif;
use crate::glif::write::{write_ufo_glif, write_ufo_glif_data};
use crate::point::PointData;

impl<PD: PointData, S: AsRef<str>> From<S> for Glif<PD> {
    fn from(s: S) -> Self {
        read_ufo_glif(s.as_ref()).expect("Called Into<&str/String> on invalid .glif data!")
    }
}

impl<PD: PointData> From<&Glif<PD>> for Vec<u8> {
    fn from(g: &Glif<PD>) -> Self {
        write_ufo_glif_data(g).expect("Called Into<Vec<u8>> on invalid in-memory Glif!")
    }
}

impl<PD: PointData> From<&Glif<PD>> for String {
    fn from(g: &Glif<PD>) -> Self {
        write_ufo_glif(g).expect("Called Into<String> on invalid in-memory Glif!")
    }
}
