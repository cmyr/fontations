#![parse_module(read_fonts::codegen_test::conditions)]

#[skip_constructor] // because we don't use it, this avoids an unused warning
table MajorMinorVersion {
    #[version]
    #[default(MajorMinor::VERSION_1_1)]
    version: MajorMinor,
    always_present: u16,
    #[since_version(1.1)]
    if_11: u16,
    #[since_version(2.0)]
    if_20: u32,
}


flags u16 GotFlags {
    FOO = 0x0001,
    BAR = 0x0002,
}

table FlagDay {
    volume: u16,
    flags: GotFlags,
    #[if_flag($flags, GotFlags::FOO)]
    foo: u16,
    #[if_flag($flags, GotFlags::BAR)]
    bar: u16,
}
