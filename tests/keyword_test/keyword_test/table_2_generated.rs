// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum Table2Offset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Table2<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Table2<'a> {
  type Inner = Table2<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: unsafe { flatbuffers::Table::new(buf, loc) } }
  }
}

impl<'a> Table2<'a> {
  pub const VT_TYPE_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_TYPE_: flatbuffers::VOffsetT = 6;

  pub const fn get_fully_qualified_name() -> &'static str {
    "KeywordTest.Table2"
  }

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Table2 { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args Table2Args
  ) -> flatbuffers::WIPOffset<Table2<'bldr>> {
    let mut builder = Table2Builder::new(_fbb);
    if let Some(x) = args.type_ { builder.add_type_(x); }
    builder.add_type_type(args.type_type);
    builder.finish()
  }

  pub fn unpack(&self) -> Table2T {
    let type_ = match self.type_type() {
      KeywordsInUnion::NONE => KeywordsInUnionT::NONE,
      KeywordsInUnion::static_ => KeywordsInUnionT::Static_(Box::new(
        self.type__as_static_()
            .expect("Invalid union table, expected `KeywordsInUnion::static_`.")
            .unpack()
      )),
      KeywordsInUnion::internal => KeywordsInUnionT::Internal(Box::new(
        self.type__as_internal()
            .expect("Invalid union table, expected `KeywordsInUnion::internal`.")
            .unpack()
      )),
      _ => KeywordsInUnionT::NONE,
    };
    Table2T {
      type_,
    }
  }

  #[inline]
  pub fn type_type(&self) -> KeywordsInUnion {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<KeywordsInUnion>(Table2::VT_TYPE_TYPE, Some(KeywordsInUnion::NONE)).unwrap()}
  }
  #[inline]
  pub fn type_(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Table2::VT_TYPE_, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn type__as_static_(&self) -> Option<KeywordsInTable<'a>> {
    if self.type_type() == KeywordsInUnion::static_ {
      self.type_().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { KeywordsInTable::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn type__as_internal(&self) -> Option<KeywordsInTable<'a>> {
    if self.type_type() == KeywordsInUnion::internal {
      self.type_().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { KeywordsInTable::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Table2<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<KeywordsInUnion, _>("type_type", Self::VT_TYPE_TYPE, "type_", Self::VT_TYPE_, false, |key, v, pos| {
        match key {
          KeywordsInUnion::static_ => v.verify_union_variant::<flatbuffers::ForwardsUOffset<KeywordsInTable>>("KeywordsInUnion::static_", pos),
          KeywordsInUnion::internal => v.verify_union_variant::<flatbuffers::ForwardsUOffset<KeywordsInTable>>("KeywordsInUnion::internal", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct Table2Args {
    pub type_type: KeywordsInUnion,
    pub type_: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for Table2Args {
  #[inline]
  fn default() -> Self {
    Table2Args {
      type_type: KeywordsInUnion::NONE,
      type_: None,
    }
  }
}

pub struct Table2Builder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> Table2Builder<'a, 'b, A> {
  #[inline]
  pub fn add_type_type(&mut self, type_type: KeywordsInUnion) {
    self.fbb_.push_slot::<KeywordsInUnion>(Table2::VT_TYPE_TYPE, type_type, KeywordsInUnion::NONE);
  }
  #[inline]
  pub fn add_type_(&mut self, type_: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Table2::VT_TYPE_, type_);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> Table2Builder<'a, 'b, A> {
    let start = _fbb.start_table();
    Table2Builder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Table2<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Table2<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Table2");
      ds.field("type_type", &self.type_type());
      match self.type_type() {
        KeywordsInUnion::static_ => {
          if let Some(x) = self.type__as_static_() {
            ds.field("type_", &x)
          } else {
            ds.field("type_", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        KeywordsInUnion::internal => {
          if let Some(x) = self.type__as_internal() {
            ds.field("type_", &x)
          } else {
            ds.field("type_", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("type_", &x)
        },
      };
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Table2T {
  pub type_: KeywordsInUnionT,
}
impl Default for Table2T {
  fn default() -> Self {
    Self {
      type_: KeywordsInUnionT::NONE,
    }
  }
}
impl Table2T {
  pub fn pack<'b, A: flatbuffers::Allocator + 'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b, A>
  ) -> flatbuffers::WIPOffset<Table2<'b>> {
    let type_type = self.type_.keywords_in_union_type();
    let type_ = self.type_.pack(_fbb);
    Table2::create(_fbb, &Table2Args{
      type_type,
      type_,
    })
  }
}
