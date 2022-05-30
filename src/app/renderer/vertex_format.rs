use super::*;

// It's very annoying that *type* is a keyword in Rust...
#[derive(Copy, Clone, Debug, ImDraw)]
pub(super) enum AttributeVariant {
    Float,
    Float2,
    Float3,
    Float4,
    /*
    Byte4,
    UByte4,
    Short2,
    UShort2,
    Short4,
    UShort4,
    None,
    */
}

impl AttributeVariant {
    pub(super) fn components_count(self) -> u32 {
        match self {
            AttributeVariant::Float  => 1,
            AttributeVariant::Float2 => 2,
            AttributeVariant::Float3 => 3,
            AttributeVariant::Float4 => 4,
        }
    }

    pub(super) fn size(self) -> u32 {
        self.components_count() * std::mem::size_of::<GLfloat>() as u32
    }
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub(super) struct VertexAttribute {
    pub(super) location:   u8,
    pub(super) variant:    AttributeVariant,
    pub(super) normalized: bool,
}

#[derive(Clone, Debug, ImDraw)]
pub(super) struct VertexFormat {
    attributes: Vec<VertexAttribute>,
    size: u32,
}

impl VertexFormat {
    pub(super) fn new(attributes: Vec<VertexAttribute>) -> Self {
        let size = attributes.iter()
            .fold(0, |acc, attrib| acc + attrib.variant.size());

        VertexFormat {
            attributes,
            size,
        }
    }

    pub(super) fn attributes(&self) -> &Vec<VertexAttribute> { &self.attributes }
    pub(super) fn size(&self) -> u32 { self.size }
}
