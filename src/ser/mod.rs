use std::str;

pub mod impls;

///////////////////////////////////////////////////////////////////////////////

pub trait Serialize {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer;
}

///////////////////////////////////////////////////////////////////////////////

pub trait Serializer {
    type Error;

    fn visit_bool(&mut self, v: bool) -> Result<(), Self::Error>;

    #[inline]
    fn visit_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_i64(&mut self, v: i64) -> Result<(), Self::Error>;

    #[inline]
    fn visit_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        self.visit_u64(v as u64)
    }

    #[inline]
    fn visit_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.visit_u64(v as u64)
    }

    #[inline]
    fn visit_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        self.visit_u64(v as u64)
    }

    #[inline]
    fn visit_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        self.visit_u64(v as u64)
    }

    #[inline]
    fn visit_u64(&mut self, v: u64) -> Result<(), Self::Error>;

    #[inline]
    fn visit_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        self.visit_f64(v as f64)
    }

    fn visit_f64(&mut self, v: f64) -> Result<(), Self::Error>;

    /// `visit_char` serializes a character. By default it serializes it as a `&str` containing a
    /// single character.
    #[inline]
    fn visit_char(&mut self, v: char) -> Result<(), Self::Error> {
        // The unwraps in here should be safe.
        let mut s = &mut [0; 4];
        let len = v.encode_utf8(s).unwrap();
        self.visit_str(str::from_utf8(&s[..len]).unwrap())
    }

    /// `visit_str` serializes a `&str`.
    fn visit_str(&mut self, value: &str) -> Result<(), Self::Error>;

    /// `visit_bytes` is a hook that enables those serialization formats that support serializing
    /// byte slices separately from generic arrays. By default it serializes as a regular array. 
    #[inline]
    fn visit_bytes(&mut self, value: &[u8]) -> Result<(), Self::Error> {
        self.visit_seq(impls::SeqIteratorVisitor::new(value.iter(), Some(value.len())))
    }

    fn visit_unit(&mut self) -> Result<(), Self::Error>;

    #[inline]
    fn visit_named_unit(&mut self, _name: &str) -> Result<(), Self::Error> {
        self.visit_unit()
    }

    #[inline]
    fn visit_enum_unit(&mut self,
                       _name: &str,
                       _variant: &str) -> Result<(), Self::Error> {
        self.visit_unit()
    }

    fn visit_none(&mut self) -> Result<(), Self::Error>;

    fn visit_some<V>(&mut self, value: V) -> Result<(), Self::Error>
        where V: Serialize;

    fn visit_seq<V>(&mut self, visitor: V) -> Result<(), Self::Error>
        where V: SeqVisitor;

    #[inline]
    fn visit_named_seq<V>(&mut self,
                          _name: &'static str,
                          visitor: V) -> Result<(), Self::Error>
        where V: SeqVisitor,
    {
        self.visit_seq(visitor)
    }

    #[inline]
    fn visit_enum_seq<V>(&mut self,
                         _name: &'static str,
                         _variant: &'static str,
                         visitor: V) -> Result<(), Self::Error>
        where V: SeqVisitor,
    {
        self.visit_seq(visitor)
    }

    fn visit_seq_elt<T>(&mut self, value: T) -> Result<(), Self::Error>
        where T: Serialize;

    fn visit_map<V>(&mut self, visitor: V) -> Result<(), Self::Error>
        where V: MapVisitor;

    #[inline]
    fn visit_named_map<V>(&mut self,
                          _name: &'static str,
                          visitor: V) -> Result<(), Self::Error>
        where V: MapVisitor,
    {
        self.visit_map(visitor)
    }

    #[inline]
    fn visit_enum_map<V>(&mut self,
                          _name: &'static str,
                          _variant: &'static str,
                          visitor: V) -> Result<(), Self::Error>
        where V: MapVisitor,
    {
        self.visit_map(visitor)
    }

    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error>
        where K: Serialize,
              V: Serialize;
}

pub trait SeqVisitor {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: Serializer;

    /// Return the length of the sequence if known.
    #[inline]
    fn len(&self) -> Option<usize> {
        None
    }
}

pub trait MapVisitor {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: Serializer;

    /// Return the length of the map if known.
    #[inline]
    fn len(&self) -> Option<usize> {
        None
    }
}
