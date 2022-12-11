use test_log::{self, test};
use glifparser as gp;
type GPResult = Result<gp::Glif<()>, gp::error::GlifParserError>;

mod issue54;
