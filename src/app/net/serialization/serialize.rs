use super::*;

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
        pub fn $func<const MIN: $type, const MAX: $type>(&mut self, v: $type) -> Result<()>
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

    pub fn finish(&mut self) -> Result<usize> {
        if self.scratch_bits > 0 {
            if self.word_index >= self.buffer.len() {
                return Err(SerializationError::BufferTooSmall);
            }

            self.buffer[self.word_index] = self.scratch as u32;
            self.word_index += 1;
            self.scratch = 0;
            self.scratch_bits = 0;
        }

        Ok(self.word_index)
    }

    // Serialize up to 32 bits
    fn serialize<const N: usize>(&mut self, v: u32) -> Result<()> {
        // @Fix should be a compile-time assert, but it's not supported in Rust (only via hacks).
        if N > 32 { return Err(SerializationError::WordTooSmall); }
        if N == 0 { return Ok(()); }

        let v = v.to_le();
        //println!("[serialize] writing: {}", v);

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

    pub fn serialize_bool(&mut self, v: bool) -> Result<()> { self.serialize::<1>(v as u32) }

    pub fn serialize_i8 (&mut self, v:  i8) -> Result<()> { self.serialize::<8>(v.to_le_bytes()[0] as u32) }
    pub fn serialize_u8 (&mut self, v:  u8) -> Result<()> { self.serialize::<8>(v as u32) }
    pub fn serialize_i16(&mut self, v: i16) -> Result<()> { self.serialize::<16>(u16::from_le_bytes(v.to_le_bytes()) as u32) }
    pub fn serialize_u16(&mut self, v: u16) -> Result<()> { self.serialize::<16>(v as u32) }
    pub fn serialize_i32(&mut self, v: i32) -> Result<()> { self.serialize::<32>(v as u32) }
    pub fn serialize_u32(&mut self, v: u32) -> Result<()> { self.serialize::<32>(v) }

    pub fn serialize_i64(&mut self, v: i64) -> Result<()> { self.serialize_u64(u64::from_le_bytes(v.to_le_bytes())) }
    pub fn serialize_u64(&mut self, v: u64) -> Result<()> {
        let word = v as u32;
        self.serialize::<32>(word)?;
        let word = (v >> 32) as u32;
        self.serialize::<32>(word)?;
        Ok(())
    }

    pub fn serialize_i128(&mut self, v: i128) -> Result<()> { self.serialize_u128(u128::from_le_bytes(v.to_le_bytes())) }
    pub fn serialize_u128(&mut self, v: u128) -> Result<()> {
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

    pub fn serialize_packed_i64<const MIN: i64, const MAX: i64>(&mut self, v: i64) -> Result<()>
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

    pub fn serialize_packed_u64<const MIN: u64, const MAX: u64>(&mut self, v: u64) -> Result<()>
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

    pub fn serialize_packed_i128<const MIN: i128, const MAX: i128>(&mut self, _v: i128) -> Result<()> { unimplemented!(); }
    pub fn serialize_packed_u128<const MIN: u128, const MAX: u128>(&mut self, _v: u128) -> Result<()> { unimplemented!(); }
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
    fn serialize(&self, serializer: &mut Serializer) -> Result<()>;
}

impl Serialize for () {
    fn serialize(&self, _: &mut Serializer) -> Result<()> { Ok(()) }
}

impl<A: Serialize, B: Serialize> Serialize for (A, B) {
    fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
        self.0.serialize(serializer)?;
        self.1.serialize(serializer)?;
        Ok(())
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
        match &self {
            Some(v) => {
                true.serialize(serializer)?;
                v.serialize(serializer)?;
            },
            None => {
                false.serialize(serializer)?;
            }
        }
        Ok(())
    }
}

impl<T: Serialize, const N: usize> Serialize for [T; N] {
    fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
        for v in self.iter() {
            v.serialize(serializer)?;
        }
        Ok(())
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
        (self.len() as u32).serialize(serializer)?;
        for v in self.iter() {
            v.serialize(serializer)?;
        }
        Ok(())
    }
}

macro_rules! impl_serialize {
    ($type:ty, $ser:ident) => {
        impl Serialize for $type {
            fn serialize(&self, serializer: &mut Serializer) -> Result<()> { serializer.$ser(*self) }
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
