extern crate minerva_refract;
use minerva_refract::expose_under_target;

fn main() {
    println!("cargo:rerun-if-changed=src/test_serializer.rs");
    println!("cargo:rerun-if-changed=src/test_decoder.rs");
    expose_under_target("src/test_serializer.rs", Some("refract_moz_cbor"), "test_serializer.rs").unwrap();
    expose_under_target("src/test_decoder.rs", Some("refract_moz_cbor"), "test_decoder.rs").unwrap();
}
