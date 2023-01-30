// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

#[derive(Clone, Debug)]
pub struct KindsOfOffsets {
    /// The major/minor version of the GDEF table
    pub version: MajorMinor,
    /// A normal offset
    pub nonnullable: OffsetMarker<Dummy>,
    /// An offset that is nullable, but always present
    pub nullable: NullableOffsetMarker<Dummy>,
    /// An offset to an array:
    pub array: OffsetMarker<Vec<u16>>,
    /// An offset to an array of records
    pub record_array: OffsetMarker<Vec<Shmecord>>,
    /// A nullable, versioned offset to an array of records
    pub versioned_nullable_record_array: NullableOffsetMarker<Vec<Shmecord>>,
    /// A normal offset that is versioned
    pub versioned_nonnullable: Option<OffsetMarker<Dummy>>,
    /// An offset that is nullable and versioned
    pub versioned_nullable: NullableOffsetMarker<Dummy, WIDTH_32>,
}

impl Default for KindsOfOffsets {
    fn default() -> Self {
        Self {
            version: MajorMinor::VERSION_1_1,
            nonnullable: Default::default(),
            nullable: Default::default(),
            array: Default::default(),
            record_array: Default::default(),
            versioned_nullable_record_array: Default::default(),
            versioned_nonnullable: Default::default(),
            versioned_nullable: Default::default(),
        }
    }
}

impl FontWrite for KindsOfOffsets {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.version;
        version.write_into(writer);
        self.nonnullable.write_into(writer);
        self.nullable.write_into(writer);
        (array_len(&self.array).unwrap() as u16).write_into(writer);
        self.array.write_into(writer);
        self.record_array.write_into(writer);
        version
            .compatible((1, 1))
            .then(|| self.versioned_nullable_record_array.write_into(writer));
        version.compatible((1, 1)).then(|| {
            self.versioned_nonnullable
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version
            .compatible((1, 1))
            .then(|| self.versioned_nullable.write_into(writer));
    }
}

impl Validate for KindsOfOffsets {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("KindsOfOffsets", |ctx| {
            let version = self.version;
            ctx.in_field("nonnullable", |ctx| {
                self.nonnullable.validate_impl(ctx);
            });
            ctx.in_field("nullable", |ctx| {
                self.nullable.validate_impl(ctx);
            });
            ctx.in_field("record_array", |ctx| {
                self.record_array.validate_impl(ctx);
            });
            ctx.in_field("versioned_nullable_record_array", |ctx| {
                self.versioned_nullable_record_array.validate_impl(ctx);
            });
            ctx.in_field("versioned_nonnullable", |ctx| {
                if version.compatible((1, 1)) && self.versioned_nonnullable.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
                self.versioned_nonnullable.validate_impl(ctx);
            });
            ctx.in_field("versioned_nullable", |ctx| {
                self.versioned_nullable.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::codegen_test::offsets_arrays::KindsOfOffsets<'a>>
    for KindsOfOffsets
{
    fn from_obj_ref(
        obj: &read_fonts::codegen_test::offsets_arrays::KindsOfOffsets<'a>,
        _: FontData,
    ) -> Self {
        let offset_data = obj.offset_data();
        KindsOfOffsets {
            version: obj.version(),
            nonnullable: obj.nonnullable().to_owned_table(),
            nullable: obj.nullable().to_owned_table(),
            array: obj.array().to_owned_obj(offset_data),
            record_array: obj.record_array().to_owned_obj(offset_data),
            versioned_nullable_record_array: obj
                .versioned_nullable_record_array()
                .to_owned_obj(offset_data),
            versioned_nonnullable: obj.versioned_nonnullable().to_owned_table(),
            versioned_nullable: obj.versioned_nullable().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::codegen_test::offsets_arrays::KindsOfOffsets<'a>>
    for KindsOfOffsets
{
}

impl<'a> FontRead<'a> for KindsOfOffsets {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::codegen_test::offsets_arrays::KindsOfOffsets as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug, Default)]
pub struct KindsOfArraysOfOffsets {
    /// A normal array offset
    pub nonnullables: Vec<OffsetMarker<Dummy>>,
    /// An offset that is nullable, but always present
    pub nullables: Vec<NullableOffsetMarker<Dummy>>,
    /// A normal offset that is versioned
    pub versioned_nonnullables: Option<Vec<OffsetMarker<Dummy>>>,
    /// An offset that is nullable and versioned
    pub versioned_nullables: Option<Vec<NullableOffsetMarker<Dummy>>>,
}

impl FontWrite for KindsOfArraysOfOffsets {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = MajorMinor::VERSION_1_1 as MajorMinor;
        version.write_into(writer);
        (array_len(&self.nonnullables).unwrap() as u16).write_into(writer);
        self.nonnullables.write_into(writer);
        self.nullables.write_into(writer);
        version.compatible((1, 1)).then(|| {
            self.versioned_nonnullables
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible((1, 1)).then(|| {
            self.versioned_nullables
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
    }
}

impl Validate for KindsOfArraysOfOffsets {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("KindsOfArraysOfOffsets", |ctx| {
            let version: MajorMinor = MajorMinor::VERSION_1_1;
            ctx.in_field("nonnullables", |ctx| {
                if self.nonnullables.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.nonnullables.validate_impl(ctx);
            });
            ctx.in_field("nullables", |ctx| {
                if self.nullables.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.nullables.validate_impl(ctx);
            });
            ctx.in_field("versioned_nonnullables", |ctx| {
                if version.compatible((1, 1)) && self.versioned_nonnullables.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
                if self.versioned_nonnullables.is_some()
                    && self.versioned_nonnullables.as_ref().unwrap().len() > (u16::MAX as usize)
                {
                    ctx.report("array exceeds max length");
                }
                self.versioned_nonnullables.validate_impl(ctx);
            });
            ctx.in_field("versioned_nullables", |ctx| {
                if version.compatible((1, 1)) && self.versioned_nullables.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
                if self.versioned_nullables.is_some()
                    && self.versioned_nullables.as_ref().unwrap().len() > (u16::MAX as usize)
                {
                    ctx.report("array exceeds max length");
                }
                self.versioned_nullables.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::codegen_test::offsets_arrays::KindsOfArraysOfOffsets<'a>>
    for KindsOfArraysOfOffsets
{
    fn from_obj_ref(
        obj: &read_fonts::codegen_test::offsets_arrays::KindsOfArraysOfOffsets<'a>,
        _: FontData,
    ) -> Self {
        KindsOfArraysOfOffsets {
            nonnullables: obj.nonnullables().map(|x| x.to_owned_table()).collect(),
            nullables: obj.nullables().map(|x| x.to_owned_table()).collect(),
            versioned_nonnullables: obj
                .versioned_nonnullables()
                .map(|obj| obj.map(|x| x.to_owned_table()).collect()),
            versioned_nullables: obj
                .versioned_nullables()
                .map(|obj| obj.map(|x| x.to_owned_table()).collect()),
        }
    }
}

impl<'a> FromTableRef<read_fonts::codegen_test::offsets_arrays::KindsOfArraysOfOffsets<'a>>
    for KindsOfArraysOfOffsets
{
}

impl<'a> FontRead<'a> for KindsOfArraysOfOffsets {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::codegen_test::offsets_arrays::KindsOfArraysOfOffsets as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug)]
pub struct KindsOfArrays {
    pub version: u16,
    /// an array of scalars
    pub scalars: Vec<u16>,
    /// an array of records
    pub records: Vec<Shmecord>,
    /// a versioned array of scalars
    pub versioned_scalars: Option<Vec<u16>>,
    /// a versioned array of scalars
    pub versioned_records: Option<Vec<Shmecord>>,
}

impl Default for KindsOfArrays {
    fn default() -> Self {
        Self {
            version: 1,
            scalars: Default::default(),
            records: Default::default(),
            versioned_scalars: Default::default(),
            versioned_records: Default::default(),
        }
    }
}

impl FontWrite for KindsOfArrays {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.version;
        version.write_into(writer);
        (array_len(&self.scalars).unwrap() as u16).write_into(writer);
        self.scalars.write_into(writer);
        self.records.write_into(writer);
        version.compatible(1).then(|| {
            self.versioned_scalars
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(1).then(|| {
            self.versioned_records
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
    }
}

impl Validate for KindsOfArrays {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("KindsOfArrays", |ctx| {
            let version = self.version;
            ctx.in_field("scalars", |ctx| {
                if self.scalars.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
            ctx.in_field("records", |ctx| {
                if self.records.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.records.validate_impl(ctx);
            });
            ctx.in_field("versioned_scalars", |ctx| {
                if version.compatible(1) && self.versioned_scalars.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
                if self.versioned_scalars.is_some()
                    && self.versioned_scalars.as_ref().unwrap().len() > (u16::MAX as usize)
                {
                    ctx.report("array exceeds max length");
                }
            });
            ctx.in_field("versioned_records", |ctx| {
                if version.compatible(1) && self.versioned_records.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
                if self.versioned_records.is_some()
                    && self.versioned_records.as_ref().unwrap().len() > (u16::MAX as usize)
                {
                    ctx.report("array exceeds max length");
                }
                self.versioned_records.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::codegen_test::offsets_arrays::KindsOfArrays<'a>> for KindsOfArrays {
    fn from_obj_ref(
        obj: &read_fonts::codegen_test::offsets_arrays::KindsOfArrays<'a>,
        _: FontData,
    ) -> Self {
        let offset_data = obj.offset_data();
        KindsOfArrays {
            version: obj.version(),
            scalars: obj.scalars().to_owned_obj(offset_data),
            records: obj.records().to_owned_obj(offset_data),
            versioned_scalars: obj.versioned_scalars().to_owned_obj(offset_data),
            versioned_records: obj.versioned_records().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::codegen_test::offsets_arrays::KindsOfArrays<'a>>
    for KindsOfArrays
{
}

impl<'a> FontRead<'a> for KindsOfArrays {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::codegen_test::offsets_arrays::KindsOfArrays as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Dummy {
    pub value: u16,
}

impl FontWrite for Dummy {
    fn write_into(&self, writer: &mut TableWriter) {
        self.value.write_into(writer);
    }
}

impl Validate for Dummy {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::codegen_test::offsets_arrays::Dummy<'a>> for Dummy {
    fn from_obj_ref(
        obj: &read_fonts::codegen_test::offsets_arrays::Dummy<'a>,
        _: FontData,
    ) -> Self {
        Dummy { value: obj.value() }
    }
}

impl<'a> FromTableRef<read_fonts::codegen_test::offsets_arrays::Dummy<'a>> for Dummy {}

impl<'a> FontRead<'a> for Dummy {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::codegen_test::offsets_arrays::Dummy as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Shmecord {
    pub length: u16,
    pub breadth: u32,
}

impl FontWrite for Shmecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.length.write_into(writer);
        self.breadth.write_into(writer);
    }
}

impl Validate for Shmecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::codegen_test::offsets_arrays::Shmecord> for Shmecord {
    fn from_obj_ref(obj: &read_fonts::codegen_test::offsets_arrays::Shmecord, _: FontData) -> Self {
        Shmecord {
            length: obj.length(),
            breadth: obj.breadth(),
        }
    }
}
