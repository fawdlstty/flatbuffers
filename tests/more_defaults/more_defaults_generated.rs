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
pub enum MoreDefaultsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct MoreDefaults<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for MoreDefaults<'a> {
  type Inner = MoreDefaults<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: unsafe { flatbuffers::Table::new(buf, loc) } }
  }
}

impl<'a> MoreDefaults<'a> {
  pub const VT_INTS: flatbuffers::VOffsetT = 4;
  pub const VT_FLOATS: flatbuffers::VOffsetT = 6;
  pub const VT_EMPTY_STRING: flatbuffers::VOffsetT = 8;
  pub const VT_SOME_STRING: flatbuffers::VOffsetT = 10;
  pub const VT_ABCS: flatbuffers::VOffsetT = 12;
  pub const VT_BOOLS: flatbuffers::VOffsetT = 14;

  pub const fn get_fully_qualified_name() -> &'static str {
    "MoreDefaults"
  }

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    MoreDefaults { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args MoreDefaultsArgs<'args>
  ) -> flatbuffers::WIPOffset<MoreDefaults<'bldr>> {
    let mut builder = MoreDefaultsBuilder::new(_fbb);
    if let Some(x) = args.bools { builder.add_bools(x); }
    if let Some(x) = args.abcs { builder.add_abcs(x); }
    if let Some(x) = args.some_string { builder.add_some_string(x); }
    if let Some(x) = args.empty_string { builder.add_empty_string(x); }
    if let Some(x) = args.floats { builder.add_floats(x); }
    if let Some(x) = args.ints { builder.add_ints(x); }
    builder.finish()
  }

  pub fn unpack(&self) -> MoreDefaultsT {
    let ints = {
      let x = self.ints();
      x.into_iter().collect()
    };
    let floats = {
      let x = self.floats();
      x.into_iter().collect()
    };
    let empty_string = {
      let x = self.empty_string();
      x.to_string()
    };
    let some_string = {
      let x = self.some_string();
      x.to_string()
    };
    let abcs = {
      let x = self.abcs();
      x.into_iter().collect()
    };
    let bools = {
      let x = self.bools();
      x.into_iter().collect()
    };
    MoreDefaultsT {
      ints,
      floats,
      empty_string,
      some_string,
      abcs,
      bools,
    }
  }

  #[inline]
  pub fn ints(&self) -> flatbuffers::Vector<'a, i32> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i32>>>(MoreDefaults::VT_INTS, Some(Default::default())).unwrap()}
  }
  #[inline]
  pub fn floats(&self) -> flatbuffers::Vector<'a, f32> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f32>>>(MoreDefaults::VT_FLOATS, Some(Default::default())).unwrap()}
  }
  #[inline]
  pub fn empty_string(&self) -> &'a str {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MoreDefaults::VT_EMPTY_STRING, Some(&"")).unwrap()}
  }
  #[inline]
  pub fn some_string(&self) -> &'a str {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(MoreDefaults::VT_SOME_STRING, Some(&"some")).unwrap()}
  }
  #[inline]
  pub fn abcs(&self) -> flatbuffers::Vector<'a, ABC> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, ABC>>>(MoreDefaults::VT_ABCS, Some(Default::default())).unwrap()}
  }
  #[inline]
  pub fn bools(&self) -> flatbuffers::Vector<'a, bool> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, bool>>>(MoreDefaults::VT_BOOLS, Some(Default::default())).unwrap()}
  }
}

impl flatbuffers::Verifiable for MoreDefaults<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, i32>>>("ints", Self::VT_INTS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, f32>>>("floats", Self::VT_FLOATS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("empty_string", Self::VT_EMPTY_STRING, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("some_string", Self::VT_SOME_STRING, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, ABC>>>("abcs", Self::VT_ABCS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, bool>>>("bools", Self::VT_BOOLS, false)?
     .finish();
    Ok(())
  }
}
pub struct MoreDefaultsArgs<'a> {
    pub ints: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i32>>>,
    pub floats: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f32>>>,
    pub empty_string: Option<flatbuffers::WIPOffset<&'a str>>,
    pub some_string: Option<flatbuffers::WIPOffset<&'a str>>,
    pub abcs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, ABC>>>,
    pub bools: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, bool>>>,
}
impl<'a> Default for MoreDefaultsArgs<'a> {
  #[inline]
  fn default() -> Self {
    MoreDefaultsArgs {
      ints: None,
      floats: None,
      empty_string: None,
      some_string: None,
      abcs: None,
      bools: None,
    }
  }
}

pub struct MoreDefaultsBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> MoreDefaultsBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_ints(&mut self, ints: flatbuffers::WIPOffset<flatbuffers::Vector<'b , i32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MoreDefaults::VT_INTS, ints);
  }
  #[inline]
  pub fn add_floats(&mut self, floats: flatbuffers::WIPOffset<flatbuffers::Vector<'b , f32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MoreDefaults::VT_FLOATS, floats);
  }
  #[inline]
  pub fn add_empty_string(&mut self, empty_string: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MoreDefaults::VT_EMPTY_STRING, empty_string);
  }
  #[inline]
  pub fn add_some_string(&mut self, some_string: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MoreDefaults::VT_SOME_STRING, some_string);
  }
  #[inline]
  pub fn add_abcs(&mut self, abcs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , ABC>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MoreDefaults::VT_ABCS, abcs);
  }
  #[inline]
  pub fn add_bools(&mut self, bools: flatbuffers::WIPOffset<flatbuffers::Vector<'b , bool>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(MoreDefaults::VT_BOOLS, bools);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> MoreDefaultsBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    MoreDefaultsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<MoreDefaults<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for MoreDefaults<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("MoreDefaults");
      ds.field("ints", &self.ints());
      ds.field("floats", &self.floats());
      ds.field("empty_string", &self.empty_string());
      ds.field("some_string", &self.some_string());
      ds.field("abcs", &self.abcs());
      ds.field("bools", &self.bools());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct MoreDefaultsT {
  pub ints: Vec<i32>,
  pub floats: Vec<f32>,
  pub empty_string: String,
  pub some_string: String,
  pub abcs: Vec<ABC>,
  pub bools: Vec<bool>,
}
impl Default for MoreDefaultsT {
  fn default() -> Self {
    Self {
      ints: Default::default(),
      floats: Default::default(),
      empty_string: "".to_string(),
      some_string: "some".to_string(),
      abcs: Default::default(),
      bools: Default::default(),
    }
  }
}
impl MoreDefaultsT {
  pub fn pack<'b, A: flatbuffers::Allocator + 'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b, A>
  ) -> flatbuffers::WIPOffset<MoreDefaults<'b>> {
    let ints = Some({
      let x = &self.ints;
      _fbb.create_vector(x)
    });
    let floats = Some({
      let x = &self.floats;
      _fbb.create_vector(x)
    });
    let empty_string = Some({
      let x = &self.empty_string;
      _fbb.create_string(x)
    });
    let some_string = Some({
      let x = &self.some_string;
      _fbb.create_string(x)
    });
    let abcs = Some({
      let x = &self.abcs;
      _fbb.create_vector(x)
    });
    let bools = Some({
      let x = &self.bools;
      _fbb.create_vector(x)
    });
    MoreDefaults::create(_fbb, &MoreDefaultsArgs{
      ints,
      floats,
      empty_string,
      some_string,
      abcs,
      bools,
    })
  }
}
