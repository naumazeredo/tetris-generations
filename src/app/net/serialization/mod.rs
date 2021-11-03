mod deserialize;
mod serialize;

pub use deserialize::*;
pub use serialize::*;

// @Design refactor to use Rust std::io::Write and be serde compatible.
//   I'm not doing this right now since std::io::Write uses byte buffer instead of word buffer
//   and this will probably have worse performance. I'll test this compatible code when I have
//   performance testing tools.

#[derive(Debug)]
pub enum SerializationError {
    BufferTooSmall,
    WordTooSmall,
    ByteCountExceedsMaxSize,
    ValueOutOfRange,
    InvalidProtocol,
    InvalidVersion,
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for SerializationError {}

type Result<T> = std::result::Result<T, SerializationError>;

pub const fn calculate_bits_required(min: i128, max: i128) -> usize {
    if min == max { 0 } else { ((max - min).log2() + 1) as usize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_deserialization() {
        let mut buffer = [0; 23];

        // serialize

        let mut serializer = Serializer::new(&mut buffer);

        let v = true;
        v.serialize(&mut serializer).unwrap();

        let v = -1i8;
        v.serialize(&mut serializer).unwrap();
        let v = 1u8;
        v.serialize(&mut serializer).unwrap();

        let v = -0x1122i16;
        v.serialize(&mut serializer).unwrap();
        let v = 0xaabbu16;
        v.serialize(&mut serializer).unwrap();

        let v = -0x11223344i32;
        v.serialize(&mut serializer).unwrap();
        let v = 0xaabbccddu32;
        v.serialize(&mut serializer).unwrap();

        let v = -0x1122334455667788i64;
        v.serialize(&mut serializer).unwrap();
        let v = 0xaabbccddeeff0011u64;
        v.serialize(&mut serializer).unwrap();

        let v = -0x1122334455667788_99aabbccddeeff00i128;
        v.serialize(&mut serializer).unwrap();
        let v = 0xaabbccddeeff0011_2233445566778899u128;
        v.serialize(&mut serializer).unwrap();

        let v = -10i8;
        serializer.serialize_packed_i8::<-20, 20>(v).unwrap();
        let v = 10u8;
        serializer.serialize_packed_u8::<5, 20>(v).unwrap();

        let v = -0x1122i16;
        serializer.serialize_packed_i16::<-0x2233, 0x2233>(v).unwrap();
        let v = 0xaabbu16;
        serializer.serialize_packed_u16::<0x1122, 0xddee>(v).unwrap();

        let v = -0x11223344i32;
        serializer.serialize_packed_i32::<-0x22334455, 0x22334455>(v).unwrap();
        let v = 0xaabbccddu32;
        serializer.serialize_packed_u32::<0x11223344, 0xddccbbaa>(v).unwrap();

        let v = -0x11223344_55667788i64;
        serializer.serialize_packed_i64::<-0x22334455_66778899, 0x22334455_66778899>(v).unwrap();
        let v = 0xaabbccdd_eeff0011u64;
        serializer.serialize_packed_u64::<0x11223344_55667788, 0xddccbbaa_99887766>(v).unwrap();

        serializer.finish().unwrap();

        // deserialize

        let mut deserializer = Deserializer::new(&buffer);

        let v = bool::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, true);

        let v = i8::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, -1i8);
        let v = u8::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, 1u8);

        let v = i16::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, -0x1122i16);
        let v = u16::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, 0xaabbu16);

        let v = i32::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, -0x11223344i32);
        let v = u32::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, 0xaabbccddu32);

        let v = i64::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, -0x1122334455667788i64);
        let v = u64::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, 0xaabbccddeeff0011u64);

        let v = i128::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, -0x1122334455667788_99aabbccddeeff00i128);
        let v = u128::deserialize(&mut deserializer).unwrap();
        assert_eq!(v, 0xaabbccddeeff0011_2233445566778899u128);

        let v = deserializer.deserialize_packed_i8::<-20, 20>().unwrap();
        assert_eq!(v, -10i8);
        let v = deserializer.deserialize_packed_u8::<5, 20>().unwrap();
        assert_eq!(v, 10u8);

        let v = deserializer.deserialize_packed_i16::<-0x2233, 0x2233>().unwrap();
        assert_eq!(v, -0x1122i16);
        let v = deserializer.deserialize_packed_u16::<0x1122, 0xddee>().unwrap();
        assert_eq!(v, 0xaabbu16);

        let v = deserializer.deserialize_packed_i32::<-0x22334455, 0x22334455>().unwrap();
        assert_eq!(v, -0x11223344i32);
        let v = deserializer.deserialize_packed_u32::<0x11223344, 0xddccbbaa>().unwrap();
        assert_eq!(v, 0xaabbccddu32);


        let v = deserializer.deserialize_packed_i64::<-0x22334455_66778899, 0x22334455_66778899>().unwrap();
        assert_eq!(v, -0x11223344_55667788i64);
        let v = deserializer.deserialize_packed_u64::<0x11223344_55667788, 0xddccbbaa_99887766>().unwrap();
        assert_eq!(v, 0xaabbccdd_eeff0011u64);
    }
}
