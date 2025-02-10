#![feature(generic_const_exprs)]

use serialization::{
    Decode, Deserialize, Encode, Serializable, Serialize,
    __private::{Mesh, SegmentDecoder, SegmentEncoder},
    bin::BinaryCodec,
};

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Mesh2 {
    pub triangles: Vec<Triangle>,
}

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub normal: Vector3,
}

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn test<T: std::fmt::Debug>(value: T)
where
    T: Encode + Mesh<BinaryCodec, SegmentEncoder>,
    T: Decode + Mesh<BinaryCodec, SegmentDecoder>,
    [(); size_of::<T>()]:,
{
    let mut dst = [0u8; 100000];
    serialization::bin::encode(&value, &mut dst).unwrap();
    let decoded = serialization::bin::decode::<T>(&mut dst).unwrap();
    // println!("{}", type_name::<T>());
    // println!("{:?}", &dst[..66]);
    // assert_eq!(value, decoded);
}

#[test]
fn test2() {
    test(Mesh2 {
        triangles: vec![],
    });
}
