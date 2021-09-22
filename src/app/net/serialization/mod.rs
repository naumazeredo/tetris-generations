// @Design refactor to use Rust std::io::Write and be serde compatible.
//   I'm not doing this right now since std::io::Write uses byte buffer instead of word buffer
//   and this will probably have worse performance. I'll test this compatible code when I have
//   performance testing tools.

#[derive(Debug)]
pub enum SerializationError {
    BufferTooSmall,
    WordTooSmall,
    ValueOutOfRange,
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for SerializationError {}

type Result<T> = std::result::Result<T, SerializationError>;

/*
// @TODO reserve bits
pub struct SerializerReserve {
    scratch_bits: usize,
    word_index: usize,
}
*/

// Serialization

macro_rules! impl_serialize_packed {
    ($func:ident, $type:ty) => {
        fn $func<const MIN: $type, const MAX: $type>(&mut self, v: $type) -> Result<()>
        where [(); calculate_bits_required(MIN as i128, MAX as i128)]: // @Hack just like C++ hacks... shouldn't exist!
        {
            if v < MIN || v > MAX { return Err(SerializationError::ValueOutOfRange); }
            self.serialize::<{ calculate_bits_required(MIN as i128, MAX as i128) }>((v - MIN) as u32)
        }
    }
}

pub struct Serializer<'a> {
    scratch: u64,
    scratch_bits: usize,
    word_index: usize,
    buffer: &'a mut [u32],
}

impl<'a> Serializer<'a> {
    pub fn new(buffer: &'a mut [u32]) -> Self {
        Self {
            scratch: 0,
            scratch_bits: 0,
            word_index: 0,
            buffer,
        }
    }

    // @TODO reserve bits
    //pub fn reserve<const N: usize>(&self) -> Result<SerializerReserve> { }

    pub fn finish(&mut self) -> Result<()> {
        if self.scratch_bits > 0 {
            if self.word_index >= self.buffer.len() {
                return Err(SerializationError::BufferTooSmall);
            }

            self.buffer[self.word_index] = self.scratch as u32;
            self.word_index += 1;
            self.scratch = 0;
            self.scratch_bits = 0;
        }

        Ok(())
    }

    // Serialize up to 32 bits
    fn serialize<const N: usize>(&mut self, v: u32) -> Result<()> {
        // @Fix should be a compile-time assert, but it's not supported in Rust (only via hacks).
        if N > 32 { return Err(SerializationError::WordTooSmall); }
        if N == 0 { return Ok(()); }

        let v = v.to_le();

        self.scratch |= (v as u64) << self.scratch_bits;
        self.scratch_bits += N;

        if self.scratch_bits >= 32 {
            if self.word_index >= self.buffer.len() {
                return Err(SerializationError::BufferTooSmall);
            }

            self.buffer[self.word_index] = self.scratch as u32;
            self.word_index += 1;
            self.scratch >>= 32;
            self.scratch_bits -= 32;
        }

        Ok(())
    }

    // Full range serialization

    fn serialize_bool(&mut self, v: bool) -> Result<()> { self.serialize::<1>(v as u32) }

    fn serialize_i8 (&mut self, v:  i8) -> Result<()> { self.serialize::<8>(v.to_le_bytes()[0] as u32) }
    fn serialize_u8 (&mut self, v:  u8) -> Result<()> { self.serialize::<8>(v as u32) }
    fn serialize_i16(&mut self, v: i16) -> Result<()> { self.serialize::<16>(u16::from_le_bytes(v.to_le_bytes()) as u32) }
    fn serialize_u16(&mut self, v: u16) -> Result<()> { self.serialize::<16>(v as u32) }
    fn serialize_i32(&mut self, v: i32) -> Result<()> { self.serialize::<32>(v as u32) }
    fn serialize_u32(&mut self, v: u32) -> Result<()> { self.serialize::<32>(v) }

    fn serialize_i64(&mut self, v: i64) -> Result<()> { self.serialize_u64(u64::from_le_bytes(v.to_le_bytes())) }
    fn serialize_u64(&mut self, v: u64) -> Result<()> {
        let word = v as u32;
        self.serialize::<32>(word)?;
        let word = (v >> 32) as u32;
        self.serialize::<32>(word)?;
        Ok(())
    }

    fn serialize_i128(&mut self, v: i128) -> Result<()> { self.serialize_u128(u128::from_le_bytes(v.to_le_bytes())) }
    fn serialize_u128(&mut self, v: u128) -> Result<()> {
        let word = v as u32;
        self.serialize::<32>(word)?;
        let word = (v >> 32) as u32;
        self.serialize::<32>(word)?;
        let word = (v >> 64) as u32;
        self.serialize::<32>(word)?;
        let word = (v >> 96) as u32;
        self.serialize::<32>(word)?;
        Ok(())
    }

    // Bit packing

    impl_serialize_packed!(serialize_packed_i8, i8);
    impl_serialize_packed!(serialize_packed_u8, u8);
    impl_serialize_packed!(serialize_packed_i16, i16);
    impl_serialize_packed!(serialize_packed_u16, u16);
    impl_serialize_packed!(serialize_packed_i32, i32);
    impl_serialize_packed!(serialize_packed_u32, u32);

    fn serialize_packed_i64<const MIN: i64, const MAX: i64>(&mut self, v: i64) -> Result<()>
    where
        [(); calculate_bits_required(MIN as i128, MAX as i128)]:, // @Hack just like C++ hacks... shouldn't exist!
        [(); calculate_bits_required(MIN as i128, MAX as i128) - 32]: // @Hack just like C++ hacks... shouldn't exist!
    {
        if v < MIN || v > MAX { return Err(SerializationError::ValueOutOfRange); }
        let v = v - MIN;

        // @XXX this should be a compile-time conditional, but Rust still has no way to do it
        let bits_required = calculate_bits_required(MIN as i128, MAX as i128);
        if bits_required > 32 {
            let bytes = v.to_le_bytes();
            let low  = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let high = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            self.serialize::<32>(low)?;
            self.serialize::<{ calculate_bits_required(MIN as i128, MAX as i128) - 32 }>(high)?;
        } else {
            self.serialize::<{ calculate_bits_required(MIN as i128, MAX as i128) }>(v as u32)?;
        }
        Ok(())
    }

    fn serialize_packed_u64<const MIN: u64, const MAX: u64>(&mut self, v: u64) -> Result<()>
    where
        [(); calculate_bits_required(MIN as i128, MAX as i128)]:, // @Hack just like C++ hacks... shouldn't exist!
        [(); calculate_bits_required(MIN as i128, MAX as i128) - 32]: // @Hack just like C++ hacks... shouldn't exist!
    {
        if v < MIN || v > MAX { return Err(SerializationError::ValueOutOfRange); }
        let v = v - MIN;

        // @XXX this should be a compile-time conditional, but Rust still has no way to do it
        let bits_required = calculate_bits_required(MIN as i128, MAX as i128);
        if bits_required > 32 {
            let bytes = v.to_le_bytes();
            let low  = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let high = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            self.serialize::<32>(low)?;
            self.serialize::<{ calculate_bits_required(MIN as i128, MAX as i128) - 32 }>(high)?;
        } else {
            self.serialize::<{ calculate_bits_required(MIN as i128, MAX as i128) }>(v as u32)?;
        }
        Ok(())
    }

    fn serialize_packed_i128<const MIN: i128, const MAX: i128>(&mut self, _v: i128) -> Result<()> { unimplemented!(); }
    fn serialize_packed_u128<const MIN: u128, const MAX: u128>(&mut self, _v: u128) -> Result<()> { unimplemented!(); }
}

/*
// @XXX this seems to disable the early drop optimization, which locks the immutable ref buffer
//      until end of scope or manual drop
impl Drop for Serializer<'_> {
    fn drop(&mut self) {
        assert!(self.scratch_bits != 0, "Serializer was dropped without finishing");
    }
}
*/

pub trait Serialize {
    fn serialize(self, serializer: &mut Serializer) -> Result<()>;
}

macro_rules! impl_serialize {
    ($type:ty, $ser:ident) => {
        impl Serialize for $type {
            fn serialize(self, serializer: &mut Serializer) -> Result<()> { serializer.$ser(self) }
        }
    }
}

impl_serialize!(bool, serialize_bool);
impl_serialize!(i8, serialize_i8);
impl_serialize!(u8, serialize_u8);
impl_serialize!(i16, serialize_i16);
impl_serialize!(u16, serialize_u16);
impl_serialize!(i32, serialize_i32);
impl_serialize!(u32, serialize_u32);
impl_serialize!(i64, serialize_i64);
impl_serialize!(u64, serialize_u64);
impl_serialize!(i128, serialize_i128);
impl_serialize!(u128, serialize_u128);

// Deserialization

macro_rules! impl_deserialize_packed {
    ($func:ident, $type:ty, $bytes:ident, $to_type:expr) => {
        fn $func<const MIN: $type, const MAX: $type>(&mut self) -> Result<$type>
        where [(); calculate_bits_required(MIN as i128, MAX as i128)]: // @Hack just like C++ hacks... shouldn't exist!
        {
            let $bytes = self.deserialize::<{ calculate_bits_required(MIN as i128, MAX as i128) }>()?.to_le_bytes();
            let v = match <$type>::from_le_bytes($to_type).checked_add(MIN) {
                None => return Err(SerializationError::ValueOutOfRange),
                Some(v) => v,
            };

            if v > MAX { Err(SerializationError::ValueOutOfRange) }
            else { Ok(v) }
        }
    }
}

pub struct Deserializer<'a> {
    scratch: u64,
    scratch_bits: usize,
    word_index: usize,
    buffer: &'a [u32],
}

impl<'a> Deserializer<'a> {
    pub fn new(buffer: &'a [u32]) -> Self {
        Self {
            scratch: 0,
            scratch_bits: 0,
            word_index: 0,
            buffer,
        }
    }

    // Deserialize up to 32 bits
    fn deserialize<const N: usize>(&mut self) -> Result<u32> {
        // @Fix should be a compile-time assert, but it's not supported in Rust (only via hacks).
        if N > 32 {
            return Err(SerializationError::WordTooSmall);
        }

        if self.scratch_bits < N {
            if self.word_index >= self.buffer.len() {
                return Err(SerializationError::BufferTooSmall);
            }

            self.scratch |= (self.buffer[self.word_index] as u64) << self.scratch_bits;
            self.word_index += 1;
            self.scratch_bits += 32;
        }

        let v = (self.scratch & ((1u64 << N) - 1)) as u32;
        println!("v {} {:#032b} {}", v, self.scratch, self.scratch_bits);
        self.scratch >>= N;
        self.scratch_bits -= N;
        println!("p {} {:#032b} {}", v, self.scratch, self.scratch_bits);

        Ok(v)
    }

    // Full range serialization

    fn deserialize_bool(&mut self) -> Result<bool> { Ok(self.deserialize::<1>()? != 0) }

    fn deserialize_i8(&mut self) -> Result<i8> {
        let bytes = self.deserialize::<8>()?.to_le_bytes();
        Ok(i8::from_le_bytes([bytes[0]]))
    }

    fn deserialize_u8(&mut self) -> Result<u8> {
        let bytes = self.deserialize::<8>()?.to_le_bytes();
        Ok(u8::from_le_bytes([bytes[0]]))
    }

    fn deserialize_i16(&mut self) -> Result<i16> {
        let bytes = self.deserialize::<16>()?.to_le_bytes();
        Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn deserialize_u16(&mut self) -> Result<u16> {
        let bytes = self.deserialize::<16>()?.to_le_bytes();
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn deserialize_i32(&mut self) -> Result<i32> {
        let bytes = self.deserialize::<32>()?.to_le_bytes();
        Ok(i32::from_le_bytes(bytes))
    }

    fn deserialize_u32(&mut self) -> Result<u32> {
        self.deserialize::<32>()
    }

    fn deserialize_i64(&mut self) -> Result<i64> {
        let low_bytes  = self.deserialize::<32>()?.to_le_bytes();
        let high_bytes = self.deserialize::<32>()?.to_le_bytes();
        Ok(
            i64::from_le_bytes(
                [
                    low_bytes[0],  low_bytes[1],  low_bytes[2],  low_bytes[3],
                    high_bytes[0], high_bytes[1], high_bytes[2], high_bytes[3],
                ]
            )
        )
    }

    fn deserialize_u64(&mut self) -> Result<u64> {
        let low_bytes  = self.deserialize::<32>()?.to_le_bytes();
        let high_bytes = self.deserialize::<32>()?.to_le_bytes();
        Ok(
            u64::from_le_bytes(
                [
                    low_bytes[0],  low_bytes[1],  low_bytes[2],  low_bytes[3],
                    high_bytes[0], high_bytes[1], high_bytes[2], high_bytes[3],
                ]
            )
        )
    }

    fn deserialize_i128(&mut self) -> Result<i128> {
        let low_low_bytes   = self.deserialize::<32>()?.to_le_bytes();
        let low_high_bytes  = self.deserialize::<32>()?.to_le_bytes();
        let high_low_bytes  = self.deserialize::<32>()?.to_le_bytes();
        let high_high_bytes = self.deserialize::<32>()?.to_le_bytes();
        Ok(
            i128::from_le_bytes(
                [
                    low_low_bytes[0],   low_low_bytes[1],   low_low_bytes[2],   low_low_bytes[3],
                    low_high_bytes[0],  low_high_bytes[1],  low_high_bytes[2],  low_high_bytes[3],
                    high_low_bytes[0],  high_low_bytes[1],  high_low_bytes[2],  high_low_bytes[3],
                    high_high_bytes[0], high_high_bytes[1], high_high_bytes[2], high_high_bytes[3],
                ]
            )
        )
    }

    fn deserialize_u128(&mut self) -> Result<u128> {
        let low_low_bytes   = self.deserialize::<32>()?.to_le_bytes();
        let low_high_bytes  = self.deserialize::<32>()?.to_le_bytes();
        let high_low_bytes  = self.deserialize::<32>()?.to_le_bytes();
        let high_high_bytes = self.deserialize::<32>()?.to_le_bytes();
        Ok(
            u128::from_le_bytes(
                [
                    low_low_bytes[0],   low_low_bytes[1],   low_low_bytes[2],   low_low_bytes[3],
                    low_high_bytes[0],  low_high_bytes[1],  low_high_bytes[2],  low_high_bytes[3],
                    high_low_bytes[0],  high_low_bytes[1],  high_low_bytes[2],  high_low_bytes[3],
                    high_high_bytes[0], high_high_bytes[1], high_high_bytes[2], high_high_bytes[3],
                ]
            )
        )
    }

    // Bit packing serialization

    impl_deserialize_packed!(deserialize_packed_i8, i8, bytes, [bytes[0]]);
    impl_deserialize_packed!(deserialize_packed_u8, u8, bytes, [bytes[0]]);
    impl_deserialize_packed!(deserialize_packed_i16, i16, bytes, [bytes[0], bytes[1]]);
    impl_deserialize_packed!(deserialize_packed_u16, u16, bytes, [bytes[0], bytes[1]]);
    impl_deserialize_packed!(deserialize_packed_i32, i32, bytes, bytes);
    impl_deserialize_packed!(deserialize_packed_u32, u32, bytes, bytes);

    fn deserialize_packed_i64<const MIN: i64, const MAX: i64>(&mut self) -> Result<i64>
    where
        [(); calculate_bits_required(MIN as i128, MAX as i128)]:, // @Hack just like C++ hacks... shouldn't exist!
        [(); calculate_bits_required(MIN as i128, MAX as i128) - 32]: // @Hack just like C++ hacks... shouldn't exist!
    {
        let bits_required = calculate_bits_required(MIN as i128, MAX as i128);

        let v = if bits_required > 32 {
            let low_bytes  = self.deserialize::<32>()?.to_le_bytes();
            let high_bytes = self.deserialize::<{
                calculate_bits_required(MIN as i128, MAX as i128) - 32
            }>()?.to_le_bytes();
            i64::from_le_bytes([
                low_bytes[0], low_bytes[1], low_bytes[2], low_bytes[3],
                high_bytes[0], high_bytes[1], high_bytes[2], high_bytes[3]
            ])
        } else {
            let bytes = self.deserialize::<{
                calculate_bits_required(MIN as i128, MAX as i128)
            }>()?.to_le_bytes();
            i64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], 0, 0, 0, 0])
        };

        let v = match v.checked_add(MIN) {
            None => return Err(SerializationError::ValueOutOfRange),
            Some(v) => v,
        };

        if v > MAX { Err(SerializationError::ValueOutOfRange) }
        else { Ok(v) }
    }

    fn deserialize_packed_u64<const MIN: u64, const MAX: u64>(&mut self) -> Result<u64>
    where
        [(); calculate_bits_required(MIN as i128, MAX as i128)]:, // @Hack just like C++ hacks... shouldn't exist!
        [(); calculate_bits_required(MIN as i128, MAX as i128) - 32]:, // @Hack just like C++ hacks... shouldn't exist!
    {
        let bits_required = calculate_bits_required(MIN as i128, MAX as i128);

        let v = if bits_required > 32 {
            let low_bytes  = self.deserialize::<32>()?.to_le_bytes();
            let high_bytes = self.deserialize::<{
                calculate_bits_required(MIN as i128, MAX as i128) - 32
            }>()?.to_le_bytes();

            u64::from_le_bytes([
                low_bytes[0], low_bytes[1], low_bytes[2], low_bytes[3],
                high_bytes[0], high_bytes[1], high_bytes[2], high_bytes[3]
            ])
        } else {
            let bytes = self.deserialize::<{
                calculate_bits_required(MIN as i128, MAX as i128)
            }>()?.to_le_bytes();
            u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], 0, 0, 0, 0])
        };

        let v = match v.checked_add(MIN) {
            None => return Err(SerializationError::ValueOutOfRange),
            Some(v) => v,
        };

        if v > MAX { Err(SerializationError::ValueOutOfRange) }
        else { Ok(v) }
    }

    fn deserialize_packed_i128<const MIN: i128, const MAX: i128>(&mut self) -> Result<i128> { unimplemented!(); }
    fn deserialize_packed_u128<const MIN: u128, const MAX: u128>(&mut self) -> Result<u128> { unimplemented!(); }
}

pub trait Deserialize {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> where Self: Sized;
}

macro_rules! impl_deserialize {
    ($type:ty, $deser:ident) => {
        impl Deserialize for $type {
            fn deserialize(deserializer: &mut Deserializer) -> Result<$type> { deserializer.$deser() }
        }
    }
}

impl_deserialize!(bool, deserialize_bool);
impl_deserialize!(i8, deserialize_i8);
impl_deserialize!(u8, deserialize_u8);
impl_deserialize!(i16, deserialize_i16);
impl_deserialize!(u16, deserialize_u16);
impl_deserialize!(i32, deserialize_i32);
impl_deserialize!(u32, deserialize_u32);
impl_deserialize!(i64, deserialize_i64);
impl_deserialize!(u64, deserialize_u64);
impl_deserialize!(i128, deserialize_i128);
impl_deserialize!(u128, deserialize_u128);

const fn calculate_bits_required(min: i128, max: i128) -> usize {
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
