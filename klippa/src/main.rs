//! binary subset tool
//!
//! Takes a font file and a subset input which describes the desired subset, and output is a new
//! font file containing only the data specified in the input.
//!

use clap::Parser;
use klippa::{
    parse_drop_tables, parse_name_ids, parse_name_languages, parse_unicodes, populate_gids,
    subset_font, Plan, SubsetFlags,
};
use write_fonts::read::{collections::IntSet, types::NameId, FontRef};

#[derive(Parser, Debug)]
//Allow name_IDs, so we keep the option name consistent with HB and fonttools
#[allow(non_snake_case)]
#[command(version, about, long_about = None)]
struct Args {
    /// The input font file.
    #[arg(short, long)]
    path: std::path::PathBuf,

    /// List of glyph ids
    #[arg(short, long)]
    gids: Option<String>,

    /// List of unicode codepoints
    #[arg(short, long)]
    unicodes: Option<String>,

    /// The output font file
    #[arg(short, long)]
    output_file: std::path::PathBuf,

    /// Drop the specified tables.
    #[arg(long)]
    drop_tables: Option<String>,

    /// List of 'name' table entry nameIDs
    #[arg(long)]
    name_IDs: Option<String>,

    /// List of 'name' table entry langIDs
    #[arg(long)]
    name_languages: Option<String>,

    /// drop hints
    #[arg(long)]
    no_hinting: bool,

    /// If set don't renumber glyph ids in the subset.
    #[arg(long)]
    retain_gids: bool,

    /// Remove CFF/CFF2 use of subroutines
    #[arg(long)]
    desubroutinize: bool,

    /// Keep legacy (non-Unicode) 'name' table entries
    #[arg(long)]
    name_legacy: bool,

    /// Set the overlaps flag on each glyph
    #[arg(long)]
    set_overlaps_flag: bool,

    /// Keep the outline of .notdef glyph
    #[arg(long)]
    notdef_outline: bool,

    /// Don't change the 'OS/2 ulUnicodeRange*' bits
    #[arg(long)]
    no_prune_unicode_ranges: bool,

    /// Don't perform glyph closure for layout substitution (GSUB)
    #[arg(long)]
    no_layout_closure: bool,

    /// Keep PS glyph names in TT-flavored fonts
    #[arg(long)]
    glyph_names: bool,

    /// Do not drop tables that the tool does not know how to subset
    #[arg(long)]
    passthrough_tables: bool,

    /// Perform IUP delta optimization on the resulting gvar table's deltas
    #[arg(long)]
    optimize: bool,

    ///run subsetter N times
    #[arg(short, long)]
    num_iterations: Option<u32>,
}

fn main() {
    let args = Args::parse();

    let subset_flags = parse_subset_flags(&args);
    let gids = match populate_gids(&args.gids.unwrap_or_default()) {
        Ok(gids) => gids,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let unicodes = match parse_unicodes(&args.unicodes.unwrap_or_default()) {
        Ok(unicodes) => unicodes,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let font_bytes = std::fs::read(&args.path).expect("Invalid input font file found");
    let font = FontRef::new(&font_bytes).expect("Error reading font bytes");
    let drop_tables = match parse_drop_tables(&args.drop_tables.unwrap_or_default()) {
        Ok(drop_tables) => drop_tables,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let name_ids = match &args.name_IDs {
        Some(name_ids_input) => match parse_name_ids(name_ids_input) {
            Ok(name_ids) => name_ids,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        // default value: https://github.com/harfbuzz/harfbuzz/blob/main/src/hb-subset-input.cc#L43
        None => {
            let mut default_name_ids = IntSet::<NameId>::empty();
            default_name_ids.insert_range(NameId::from(0)..=NameId::from(6));
            default_name_ids
        }
    };

    let name_languages = match &args.name_languages {
        Some(name_languages_input) => match parse_name_languages(name_languages_input) {
            Ok(name_languages) => name_languages,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        // default value: https://github.com/harfbuzz/harfbuzz/blob/main/src/hb-subset-input.cc#L44
        None => {
            let mut default_name_languages = IntSet::<u16>::empty();
            default_name_languages.insert(0x0409);
            default_name_languages
        }
    };

    let mut output_bytes = Vec::new();
    for _ in 0..args.num_iterations.unwrap_or(1) {
        let plan = Plan::new(
            &gids,
            &unicodes,
            &font,
            subset_flags,
            &drop_tables,
            &name_ids,
            &name_languages,
        );
        match subset_font(&font, &plan) {
            Ok(out) => {
                output_bytes = out;
            }
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        };
    }
    std::fs::write(&args.output_file, output_bytes).unwrap();
}

fn parse_subset_flags(args: &Args) -> SubsetFlags {
    let mut flags = SubsetFlags::default();
    if args.no_hinting {
        flags |= SubsetFlags::SUBSET_FLAGS_NO_HINTING;
    }

    if args.retain_gids {
        flags |= SubsetFlags::SUBSET_FLAGS_RETAIN_GIDS;
    }

    if args.desubroutinize {
        flags |= SubsetFlags::SUBSET_FLAGS_DESUBROUTINIZE;
    }

    if args.name_legacy {
        flags |= SubsetFlags::SUBSET_FLAGS_NAME_LEGACY;
    }

    if args.set_overlaps_flag {
        flags |= SubsetFlags::SUBSET_FLAGS_SET_OVERLAPS_FLAG;
    }

    if args.notdef_outline {
        flags |= SubsetFlags::SUBSET_FLAGS_NOTDEF_OUTLINE;
    }

    if args.no_prune_unicode_ranges {
        flags |= SubsetFlags::SUBSET_FLAGS_NO_PRUNE_UNICODE_RANGES;
    }

    if args.no_layout_closure {
        flags |= SubsetFlags::SUBSET_FLAGS_NO_LAYOUT_CLOSURE;
    }

    if args.glyph_names {
        flags |= SubsetFlags::SUBSET_FLAGS_GLYPH_NAMES;
    }

    if args.passthrough_tables {
        flags |= SubsetFlags::SUBSET_FLAGS_PASSTHROUGH_UNRECOGNIZED;
    }

    if args.optimize {
        flags |= SubsetFlags::SUBSET_FLAGS_OPTIMIZE_IUP_DELTAS;
    }
    flags
}
