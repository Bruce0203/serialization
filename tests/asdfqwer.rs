#![feature(generic_const_exprs)]

use serialization::{
    Decode, Deserialize, Encode, Serializable, Serialize,
    __private::{Mesh, SegmentDecoder, SegmentEncoder},
    bin::BinaryCodec,
};

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Updates {
    pub updates: Vec<Update>,
}

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Update {
    pub contacts: Vec<Contact>,
    pub score: u32,
    pub world_radius: f32,
    pub terrain_updates: Vec<TerrainUpdate>,
}

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Contact {
    pub damage: u8,
    pub entity_id: u32,
    pub entity_type: Option<EntityType>,

    pub guidance: Guidance,
    pub player_id: Option<u16>,
    pub reloads: Vec<bool>,
    pub transform: Transform,
    pub turret_angles: Vec<u16>,
}

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
#[repr(u8)]
pub enum EntityType {
    ArleighBurke,
    Bismarck,

    Clemenceau,

    Fletcher,

    G5,

    Iowa,

    Kolkata,

    Osa,

    Yasen,

    Zubr,
}

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Guidance {
    pub angle: u16,
    pub submerge: bool,
    pub velocity: i16,
}

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Transform {
    pub altitude: i8,
    pub angle: u16,
    pub position: (f32, f32),
    pub velocity: i16,
}

#[derive(Serializable, Serialize, Deserialize, Debug, PartialEq)]
pub struct TerrainUpdate {
    chunk_id: (i8, i8),
    data: Vec<u8>,
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
    test(Updates { updates: vec![] });
}
