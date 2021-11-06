use super::*;

pub trait Deserialize {
    fn deserialize(deserializer: &mut Deserializer) -> std::result::Result<Self, SerializationError> where Self: Sized;
    fn parse(data: &[u8]) -> std::result::Result<Self, SerializationError> where Self: Sized {
        let data = unsafe {
            std::mem::transmute::<&[u8], &[u32]>(data)
        };

        let mut deserializer = Deserializer::new(data);
        Self::deserialize(&mut deserializer)
    }
}

macro_rules! impl_deserialize_packed {
    ($func:ident, $type:ty, $bytes:ident, $to_type:expr) => {
        pub fn $func<const MIN: $type, const MAX: $type>(&mut self) -> Result<$type>
        where [(); calculate_bits_required(MIN as i128, MAX as i128)]: // @Hack just like C++ hacks... shouldn't exist!
        {
            let $bytes = self.deserialize_bits::<{ calculate_bits_required(MIN as i128, MAX as i128) }>()?.to_le_bytes();
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

    pub fn deserialize<D: Deserialize>(&mut self) -> Result<D> { D::deserialize(self) }

    // Deserialize up to 32 bits
    fn deserialize_bits<const N: usize>(&mut self) -> Result<u32> {
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
        self.scratch >>= N;
        self.scratch_bits -= N;

        Ok(v)
    }

    // Full range serialization

    pub fn deserialize_bool(&mut self) -> Result<bool> { Ok(self.deserialize_bits::<1>()? != 0) }

    pub fn deserialize_i8(&mut self) -> Result<i8> {
        let bytes = self.deserialize_bits::<8>()?.to_le_bytes();
        Ok(i8::from_le_bytes([bytes[0]]))
    }

    pub fn deserialize_u8(&mut self) -> Result<u8> {
        let bytes = self.deserialize_bits::<8>()?.to_le_bytes();
        Ok(u8::from_le_bytes([bytes[0]]))
    }

    pub fn deserialize_i16(&mut self) -> Result<i16> {
        let bytes = self.deserialize_bits::<16>()?.to_le_bytes();
        Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn deserialize_u16(&mut self) -> Result<u16> {
        let bytes = self.deserialize_bits::<16>()?.to_le_bytes();
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn deserialize_i32(&mut self) -> Result<i32> {
        let bytes = self.deserialize_bits::<32>()?.to_le_bytes();
        Ok(i32::from_le_bytes(bytes))
    }

    pub fn deserialize_u32(&mut self) -> Result<u32> {
        self.deserialize_bits::<32>()
    }

    pub fn deserialize_i64(&mut self) -> Result<i64> {
        let low_bytes  = self.deserialize_bits::<32>()?.to_le_bytes();
        let high_bytes = self.deserialize_bits::<32>()?.to_le_bytes();
        Ok(
            i64::from_le_bytes(
                [
                    low_bytes[0],  low_bytes[1],  low_bytes[2],  low_bytes[3],
                    high_bytes[0], high_bytes[1], high_bytes[2], high_bytes[3],
                ]
            )
        )
    }

    pub fn deserialize_u64(&mut self) -> Result<u64> {
        let low_bytes  = self.deserialize_bits::<32>()?.to_le_bytes();
        let high_bytes = self.deserialize_bits::<32>()?.to_le_bytes();
        Ok(
            u64::from_le_bytes(
                [
                    low_bytes[0],  low_bytes[1],  low_bytes[2],  low_bytes[3],
                    high_bytes[0], high_bytes[1], high_bytes[2], high_bytes[3],
                ]
            )
        )
    }

    pub fn deserialize_i128(&mut self) -> Result<i128> {
        let low_low_bytes   = self.deserialize_bits::<32>()?.to_le_bytes();
        let low_high_bytes  = self.deserialize_bits::<32>()?.to_le_bytes();
        let high_low_bytes  = self.deserialize_bits::<32>()?.to_le_bytes();
        let high_high_bytes = self.deserialize_bits::<32>()?.to_le_bytes();
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

    pub fn deserialize_u128(&mut self) -> Result<u128> {
        let low_low_bytes   = self.deserialize_bits::<32>()?.to_le_bytes();
        let low_high_bytes  = self.deserialize_bits::<32>()?.to_le_bytes();
        let high_low_bytes  = self.deserialize_bits::<32>()?.to_le_bytes();
        let high_high_bytes = self.deserialize_bits::<32>()?.to_le_bytes();
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

    pub fn deserialize_packed_i64<const MIN: i64, const MAX: i64>(&mut self) -> Result<i64>
    where
        [(); calculate_bits_required(MIN as i128, MAX as i128)]:, // @Hack just like C++ hacks... shouldn't exist!
        [(); calculate_bits_required(MIN as i128, MAX as i128) - 32]: // @Hack just like C++ hacks... shouldn't exist!
    {
        let bits_required = calculate_bits_required(MIN as i128, MAX as i128);

        let v = if bits_required > 32 {
            let low_bytes  = self.deserialize_bits::<32>()?.to_le_bytes();
            let high_bytes = self.deserialize_bits::<{
                calculate_bits_required(MIN as i128, MAX as i128) - 32
            }>()?.to_le_bytes();
            i64::from_le_bytes([
                low_bytes[0], low_bytes[1], low_bytes[2], low_bytes[3],
                high_bytes[0], high_bytes[1], high_bytes[2], high_bytes[3]
            ])
        } else {
            let bytes = self.deserialize_bits::<{
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

    pub fn deserialize_packed_u64<const MIN: u64, const MAX: u64>(&mut self) -> Result<u64>
    where
        [(); calculate_bits_required(MIN as i128, MAX as i128)]:, // @Hack just like C++ hacks... shouldn't exist!
        [(); calculate_bits_required(MIN as i128, MAX as i128) - 32]:, // @Hack just like C++ hacks... shouldn't exist!
    {
        let bits_required = calculate_bits_required(MIN as i128, MAX as i128);

        let v = if bits_required > 32 {
            let low_bytes  = self.deserialize_bits::<32>()?.to_le_bytes();
            let high_bytes = self.deserialize_bits::<{
                calculate_bits_required(MIN as i128, MAX as i128) - 32
            }>()?.to_le_bytes();

            u64::from_le_bytes([
                low_bytes[0], low_bytes[1], low_bytes[2], low_bytes[3],
                high_bytes[0], high_bytes[1], high_bytes[2], high_bytes[3]
            ])
        } else {
            let bytes = self.deserialize_bits::<{
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

    pub fn deserialize_packed_i128<const MIN: i128, const MAX: i128>(&mut self) -> Result<i128> { unimplemented!(); }
    pub fn deserialize_packed_u128<const MIN: u128, const MAX: u128>(&mut self) -> Result<u128> { unimplemented!(); }
}

impl Deserialize for () {
    fn deserialize(_: &mut Deserializer) -> Result<()> { Ok(()) }
}

impl<A: Deserialize, B: Deserialize> Deserialize for (A, B) {
    fn deserialize(deserializer: &mut Deserializer) -> Result<(A, B)> {
        Ok((A::deserialize(deserializer)?, B::deserialize(deserializer)?))
    }
}

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Option<T>> {
        match bool::deserialize(deserializer)? {
            true  => Ok(Some(T::deserialize(deserializer)?)),
            false => Ok(None),
        }
    }
}

impl<T: Deserialize, const N: usize> Deserialize for [T; N] {
    fn deserialize(deserializer: &mut Deserializer) -> Result<[T; N]> {
        let arr = {
            let mut data: [std::mem::MaybeUninit<T>; N] = std::mem::MaybeUninit::uninit_array();

            for elem in &mut data[..] {
                let v = T::deserialize(deserializer)?;
                unsafe { std::ptr::write(elem.as_mut_ptr(), v); }
            }

            unsafe { std::mem::MaybeUninit::array_assume_init(data) }
        };

        Ok(arr)
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Vec<T>> {
        let len = u32::deserialize(deserializer)? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::deserialize(deserializer)?);
        }
        Ok(vec)
    }
}

macro_rules! impl_deserialize {
    ($type:ty, $deser:ident) => {
        impl Deserialize for $type {
            fn deserialize(deserializer: &mut Deserializer) -> Result<Self> { deserializer.$deser() }
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
