use super::{TypeVariant, TypeData};
use ::ir::compilation_unit::TypePath;

macro_rules! default_resolve_child_name_impl {
    () => {
        fn resolve_child_name(&self, data: &TypeData, _name: &str)
                              -> Result<WeakTypeContainer> {
            bail!("attempted to access child of unsupported type {:?}",
                  self.get_type(data));
        }
    }
}
macro_rules! default_has_property_impl {
    () => {
        fn has_spec_property(&self, _data: &TypeData, prop_name: &str)
                             -> Result<Option<WeakTypeSpecContainer>> {
            bail!("variant has no property '{}'", prop_name);
        }
    }
}

mod union;
pub use self::union::{UnionVariant, UnionVariantBuilder};

mod container;
pub use self::container::{ContainerVariant, ContainerField, ContainerVariantBuilder, ContainerFieldType};

mod array;
pub use self::array::ArrayVariant;

mod sized_buffer;
pub use self::sized_buffer::SizedBufferVariant;

mod terminated_buffer;
pub use self::terminated_buffer::TerminatedBufferVariant;

mod simple_scalar;
pub use self::simple_scalar::SimpleScalarVariant;

#[derive(Debug)]
pub enum Variant {
    // Composite
    Container(ContainerVariant),
    Array(ArrayVariant),
    Union(UnionVariant),

    // Strings/Data buffers
    SizedBuffer(SizedBufferVariant),
    TerminatedBuffer(TerminatedBufferVariant),

    // Simple
    SimpleScalar(SimpleScalarVariant),
}

impl Variant {
    pub fn to_variant<'a>(&'a self) -> &'a TypeVariant {
        match *self {
            Variant::Container(ref inner) => inner,
            Variant::Array(ref inner) => inner,
            Variant::Union(ref inner) => inner,
            Variant::SizedBuffer(ref inner) => inner,
            Variant::TerminatedBuffer(ref inner) => inner,
            Variant::SimpleScalar(ref inner) => inner,
        }
    }

    pub fn to_variant_mut<'a>(&'a mut self) -> &'a mut TypeVariant {
        match *self {
            Variant::Container(ref mut inner) => inner,
            Variant::Array(ref mut inner) => inner,
            Variant::Union(ref mut inner) => inner,
            Variant::SizedBuffer(ref mut inner) => inner,
            Variant::TerminatedBuffer(ref mut inner) => inner,
            Variant::SimpleScalar(ref mut inner) => inner,
        }
    }

    pub fn get_type(&self, data: &TypeData) -> VariantType {
        match *self {
            Variant::Container(_) => VariantType::Container,
            Variant::Array(_) => VariantType::Array,
            Variant::Union(_) => VariantType::Union,
            Variant::SizedBuffer(_) => VariantType::SizedBuffer,
            Variant::TerminatedBuffer(_) => VariantType::TerminatedBuffer,
            Variant::SimpleScalar(_) =>
                VariantType::SimpleScalar(data.name.clone()),
        }
    }

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VariantType {
    Container,
    Array,
    Union,
    SizedBuffer,
    TerminatedBuffer,
    SimpleScalar(TypePath),
}