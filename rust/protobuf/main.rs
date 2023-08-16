use example_proto_rust;
use protobuf::Message;

fn main() {
    let mut example = example_proto_rust::Example::new();
    example.set_id(1);
    println!("{:?}", example);
    println!("is_initialized[{}]", example.is_initialized());
}