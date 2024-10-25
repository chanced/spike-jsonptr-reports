use jsonptr::error::{ReportErr, ReportErrMut};

fn main() {
    let mut ptr = jsonptr::PointerBuf::parse("/example").unwrap();

    let err = ptr.report_err().replace(2, "invalid").unwrap_err();
}
