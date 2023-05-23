#![allow(non_snake_case)]

use std::fs;
use vial::{
    http_parser::{parse, Status},
    Error, Request,
};

#[cfg(feature = "multipart")]
use vial::request::Multipart;

////
// helpers

fn fixture(name: &str) -> String {
    fs::read_to_string(name).unwrap()
}

fn parse_fixture(name: &str) -> Request {
    match parse(fixture(name).as_bytes().to_vec()).unwrap() {
        Status::Complete(request) => request,
        _ => panic!("Expected Status::Complete"),
    }
}

////
// tests

#[test]
fn parses_simple_GET() {
    let request = parse_fixture("tests/http/simple_GET.txt");
    assert_eq!("/", request.path());
    assert_eq!("GET", request.method());
    assert_eq!("www.codecademy.com", request.header("Host").unwrap());
}

#[test]
fn parses_another_GET() {
    let request = parse_fixture("tests/http/another_GET.txt");
    assert_eq!("/docs/index.html", request.path());
    assert_eq!("GET", request.method());
    assert_eq!("www.nowhere123.com", request.header("Host").unwrap());
    assert_eq!("en-us", request.header("Accept-Language").unwrap());
    assert_eq!(
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1)",
        request.header("User-Agent").unwrap()
    );
    assert_eq!(
        "image/gif, image/jpeg, */*",
        request.header("Accept").unwrap()
    );
}

#[test]
fn parses_big_GET() {
    let request = parse_fixture("tests/http/big_GET.txt");
    assert_eq!("/big", request.path());
    assert_eq!("GET", request.method());
    assert!(request.header("X-SOME-HEADER").is_some());
    assert!(request.header("X-SOMEOTHER-HEADER").is_some());
    assert_eq!(request.header("X-ONEMORE-HEADER").unwrap(),
        "gWbWykBHgObDHriErqIKRBqebBekBpHsqUJqQcDtDctkaeeFBwNelgvzigaEkUPKAfcnYGhgbzDOvGumdewDzCqOantKfsvaZuggZaTjqtUzOXHVYwsSjknsMTPyWzvzGrNdRExaSIjiehYvuSAMdOMpwakKlKxCPwYAyAlpqXpoiargAZnAVIRfUJVpBnotmQRsDtAZoFfSXyRvqGQluzWWVTOCItNSCqBPUfFQGoxoSewvuSStgDtCYfCnFCFNczEwGkLiPidmrpbQDPuIvopUbxvojuUrBfgjoTwslrnDIJGAWIMoMkOQzYdzxVaCDfSQlmHwkpdkxByhuWXmuLgAzgJvIuhAMMlXaHIMcGmymGCxsgUjUkzKwrzafCsfkSivOXIzNSmTGhdgBufQTqdlRbuDBZijZCOXmpwhKFzlaSleXzgMaEpDiEjxzPUwIOwhomPDVSzaTqEZCpivNWyfunffMNUaLdkxLudYEpSgwTOGUipJjvXbocrKbfFG"
    );
}

#[test]
fn parses_stacked_headers_GET() {
    let request = parse_fixture("tests/http/stacked_headers_GET.txt");
    assert_eq!("/index.html", request.path());
    assert_eq!("GET", request.method());
    assert_eq!("www.nowhere123.com", request.header("Host").unwrap());
    assert_eq!(
        "image/gif, image/jpeg, */*",
        request.header("Accept").unwrap()
    );
}

#[test]
fn parses_simple_POST() {
    let fixture = fs::File::open("tests/http/simple_POST.txt").unwrap();
    let request = Request::from_reader(fixture).unwrap();
    assert_eq!("/cgi-bin/process.cgi", request.path());
    assert_eq!("POST", request.method());
    assert_eq!(Some("hi there"), request.form("content"));
    assert_eq!(Some("1234"), request.form("licenseID"));
    assert_eq!(Some("<abc></abc>"), request.form("paramsXML"));
    assert_eq!(None, request.form("something"));

    let fixture = fs::File::open("tests/http/simple_POST2.txt").unwrap();
    let request = Request::from_reader(fixture).unwrap();
    assert_eq!("/", request.path());
    assert_eq!("POST", request.method());
    assert_eq!("keep-alive", request.header("Connection").unwrap());
    assert_eq!("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36", request.header("User-Agent").unwrap());
    assert_eq!(Some("Bobert"), request.form("name"));
    assert_eq!(Some("50-99"), request.form("age"));
}

#[test]
#[cfg(feature = "multipart")]
fn parses_multipart_POST() {
    let fixture = fs::File::open("tests/http/multipart_POST.txt").unwrap();
    let request = Request::from_reader(fixture).unwrap();
    assert_eq!("---------------------------60699960924810079512517829907", request.header("boundary").unwrap());
    let parts: Multipart = request.parse_multipart().unwrap();
    let mut count = 0;
    for p in parts {
        count += 1;
        assert_eq!(request.body_part(&p.span).len(), 208);
        // println!("part: {}:\n|{}|", p.file_name, request.body_part(&p.span));
    }
    assert_eq!(count, 1);
}

#[test]
#[cfg(feature = "multipart")]
fn parses_multipart2_POST() {
    use vial::request;

    let fixture = fs::File::open("tests/http/multipart2_POST.txt").unwrap();
    let request = Request::from_reader(fixture).unwrap();
    assert_eq!("---------------------------63696236615513812933444437561", request.header("boundary").unwrap());
    let parts: Multipart = request.parse_multipart().unwrap();
    let mut count = 0;
    for p in parts {
        count += 1;
        // println!("part: {:?}", p); //request.body_part(&p.span));
        if count == 1 {
            assert_eq!(p.file_name, "move.rs");
            assert_eq!(request.body_part(&p.span).len(), 523);
        } else {
            assert_eq!(p.file_name, "borrow.rs");
            assert_eq!(request.body_part(&p.span).len(), 208);
        }
    }
    assert_eq!(count, 2);
}

#[test]
fn rejects_malformed_headers() {
    let fixture = fs::File::open("tests/http/bad_GET.txt").unwrap();
    let err = Request::from_reader(fixture);
    assert_eq!(err.unwrap_err(), Error::ParseHeaderName);

    let fixture = fs::File::open("tests/http/bad_GET2.txt").unwrap();
    let err = Request::from_reader(fixture);
    assert_eq!(err.unwrap_err(), Error::ParseHeaderName);

    let fixture = fs::File::open("tests/http/bad_POST.txt").unwrap();
    let err = Request::from_reader(fixture);
    assert_eq!(err.unwrap_err(), Error::ParseHeaderName);
}

#[test]
fn rejects_large_headers() {
    let fixture = fs::File::open("tests/http/bad_BIG_HEADERS.txt").unwrap();
    let err = Request::from_reader(fixture);
    assert_eq!(err.unwrap_err(), Error::ParseHeaderValue);
}

#[test]
fn rejects_expected_but_no_body() {
    let fixture = fs::File::open("tests/http/bad_POST2.txt").unwrap();
    let err = Request::from_reader(fixture);
    assert_eq!(err.unwrap_err(), Error::ConnectionClosed);
}

#[test]
fn fuzzed1() {
    let fixture = fs::File::open("tests/http/fuzzed1.txt").unwrap();
    let request = Request::from_reader(fixture).unwrap();
    assert_eq!("/Ogk/process.cgi", request.path());
    assert_eq!("POST", request.method());
}
