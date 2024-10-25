use jsonptr::{error::Report, resolve, PointerBuf, ReplaceError};

fn mutable() {
    use jsonptr::error::ReportErrMut;
    let mut ptr = PointerBuf::parse("/example").unwrap();
    let err: Report<ReplaceError> = ptr.report_err().replace(2, "invalid").unwrap_err();
    // 👍
}

fn immutable() {
    use jsonptr::error::ReportErr;
    let ptr = PointerBuf::parse("/example").unwrap();
    let err: Report<resolve::Error> = ptr
        .report_err()
        .resolve(&serde_json::Value::Null)
        .unwrap_err();
    // 👍
}

fn both() {
    use jsonptr::error::{ReportErr, ReportErrMut};

    let mut ptr = PointerBuf::parse("/example").unwrap();
    let err: Report<resolve::Error> = ptr
        .report_err()
        .resolve(&serde_json::Value::Null)
        .unwrap_err();
    // 👍

    let err: Report<ReplaceError> = ptr.report_err().replace(2, "invalid").unwrap_err();
    // ❌ no method named `replace` found for struct `jsonptr::reporter::Immutable` in the current scope
}

fn main() {}
