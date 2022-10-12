#![allow(clippy::default_trait_access)]

use std::path::PathBuf;

use bpaf::{construct, Bpaf, Parser};
use tracing_subscriber::filter::LevelFilter;

use crate::units::*;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, private, version)]
#[allow(unused, clippy::default_trait_access)]
pub(crate) struct Cli {
    #[bpaf(external)]
    pub index: Vec<usize>,

    /// Prints the limits and range of each field
    pub limits: bool,

    /// Writes log to stdout instead of a file
    pub stdout: bool,

    #[bpaf(argument("unit"), fallback(Default::default()))]
    pub unit_amperage: AmperageUnit,

    #[bpaf(argument("unit"), fallback(Default::default()))]
    pub unit_flags: FlagUnit,

    #[bpaf(argument("unit"), fallback(Default::default()))]
    pub unit_frame_time: FrameTimeUnit,

    #[bpaf(argument("unit"), fallback(Default::default()))]
    pub unit_height: HeightUnit,

    #[bpaf(argument("unit"), fallback(Default::default()))]
    pub unit_rotation: RotationUnit,

    #[bpaf(argument("unit"), fallback(Default::default()))]
    pub unit_acceleration: AccelerationUnit,

    #[bpaf(argument("unit"), fallback(Default::default()))]
    pub unit_gps_speed: GpsSpeedUnit,

    #[bpaf(argument("unit"), fallback(Default::default()))]
    pub unit_vbat: VBatUnit,

    #[bpaf(external)]
    pub altitude_offset: i16,

    /// Merges GPS data into the main CSV file instead of writing it
    /// separately
    pub merge_gps: bool,

    #[bpaf(external)]
    pub current_meter: Option<CurrentMeter>,

    // TODO: alias: simulate-imu
    #[bpaf(external)]
    pub imu: Option<Imu>,

    // TODO
    // #[arg(long)]
    // /// Set magnetic declination in degrees.minutes format (e.g. -12.58 for New York)
    // declination: (),
    //
    // TODO
    // #[arg(long)]
    // /// Set magnetic declination in decimal degrees (e.g. -12.97 for New York)
    // declination_dec: (),
    #[bpaf(external)]
    pub verbosity: LevelFilter,

    // TODO: accept - for stdin
    // TODO: complete file paths
    /// One or more logs to parse
    #[bpaf(
        positional("file"),
        guard(at_least_one, "at least one log file is required")
    )]
    pub logs: Vec<PathBuf>,
}

fn index() -> impl Parser<Vec<usize>> {
    let help = "Chooses which log(s) should be decoded or omit to decode all\n(applies to all \
                logs & can be repeated)";

    bpaf::short('i')
        .long("index")
        .help(help)
        .argument::<usize>("index")
        .many()
        .map(|mut v| {
            v.sort_unstable();
            v.dedup();
            v
        })
}

fn at_least_one(logs: &Vec<PathBuf>) -> bool {
    !logs.is_empty()
}

fn altitude_offset() -> impl Parser<i16> {
    let old = bpaf::long("alt-offset").argument::<i16>("").hide();
    let new = bpaf::long("altitude-offset")
        .help("Sets the altitude offset in meters")
        .argument::<i16>("offset");

    construct!([new, old]).fallback(0)
}

#[derive(Debug, Clone, Default)]
#[allow(unused)]
pub struct CurrentMeter {
    scale: Option<i16>,
    offset: Option<i16>,
}

fn current_meter() -> impl Parser<Option<CurrentMeter>> {
    let meter = {
        let help = "Simulates a virtual current meter using throttle data";

        let old = bpaf::long("simulate-current-meter").switch().hide();
        let new = bpaf::long("current-meter").help(help).switch();

        construct!([new, old])
    };

    let scale = {
        let help =
            "Overrides the current meter scale for the simulation\n(implies --current-meter)";

        let old = bpaf::long("sim-current-meter-scale")
            .argument::<i16>("")
            .hide();
        let new = bpaf::long("current-scale")
            .help(help)
            .argument::<i16>("scale");

        construct!([new, old]).map(Some).fallback(None)
    };

    let offset = {
        let help =
            "Overrides the current meter offset for the simulation\n(implies --current-meter)";

        let old = bpaf::long("sim-current-meter-offset")
            .argument::<i16>("")
            .hide();
        let new = bpaf::long("current-offset")
            .help(help)
            .argument::<i16>("offset");

        construct!([new, old]).map(Some).fallback(None)
    };

    construct!(meter, scale, offset).map(|(meter, scale, offset)| {
        (meter || scale.is_some() || offset.is_some()).then_some(CurrentMeter { scale, offset })
    })
}

#[derive(Debug, Clone, Default)]
#[allow(unused)]
pub struct Imu {
    deg_in_names: bool,
    ignore_mag: bool,
}

fn imu() -> impl Parser<Option<Imu>> {
    let help = "Computes tilt, roll, and heading information from gyro,\naccelerometer, and \
                magnetometer data";

    let imu = {
        let old = bpaf::long("simulate-imu").switch().hide();
        let new = bpaf::long("imu").help(help).switch();

        construct!([new, old])
    };

    let deg = {
        let old = bpaf::long("include-imu-degrees").switch().hide();
        let new = bpaf::long("imu-deg")
            .help("Includes (deg) in the tilt/roll/heading header (implies --imu)")
            .switch();

        construct!([new, old])
    };

    let ignore_mag = bpaf::long("imu-ignore-mag")
        .help("Ignores magnetometer when computing heading (implies --imu)")
        .switch();

    construct!(imu, deg, ignore_mag).map(|(imu, deg, ignore_mag)| {
        (imu || deg || ignore_mag).then_some(Imu {
            deg_in_names: deg,
            ignore_mag,
        })
    })
}

fn verbosity() -> impl Parser<LevelFilter> {
    const DEFAULT: usize = if cfg!(debug_assertions) { 4 } else { 3 };
    const LEVELS: [LevelFilter; 6] = [
        LevelFilter::OFF,
        LevelFilter::ERROR,
        LevelFilter::WARN,
        LevelFilter::INFO,
        LevelFilter::DEBUG,
        LevelFilter::TRACE,
    ];

    fn plural(x: usize) -> &'static str {
        if x == 1 { "" } else { "s" }
    }

    let debug = bpaf::long("debug").switch().hide();

    let max = DEFAULT;
    let help = format!("Reduces debug output up to {max} time{}", plural(max));
    let quiet = bpaf::short('q')
        .long("quiet")
        .help(help)
        .req_flag(())
        .many()
        .map(|v| v.len());

    let max = LEVELS.len() - DEFAULT - 1;
    let help = format!("Increases debug output up to {max} time{}", plural(max));
    let verbose = bpaf::short('v')
        .long("verbose")
        .help(help)
        .req_flag(())
        .many()
        .map(|v| v.len());

    construct!(debug, verbose, quiet).map(|(debug, v, q)| {
        if debug {
            LEVELS[LEVELS.len() - 1]
        } else {
            LEVELS[(v + DEFAULT).saturating_sub(q).min(LEVELS.len() - 1)]
        }
    })
}

impl Cli {
    pub fn parse() -> Self {
        cli()
            .usage("Usage: blackbox_decode [options] <log>...")
            .run()
    }

    pub fn get_unit(&self, unit: CliUnitKind) -> CliUnit {
        match unit {
            CliUnitKind::FrameTime => CliUnit::FrameTime(self.unit_frame_time),
            CliUnitKind::Amperage => CliUnit::Amperage(self.unit_amperage),
            CliUnitKind::VBat => CliUnit::VBat(self.unit_vbat),
            CliUnitKind::Acceleration => CliUnit::Acceleration(self.unit_acceleration),
            CliUnitKind::Rotation => CliUnit::Rotation(self.unit_rotation),
            CliUnitKind::Height => CliUnit::Height(self.unit_height),
            CliUnitKind::GpsSpeed => CliUnit::GpsSpeed(self.unit_gps_speed),
            CliUnitKind::Flag => CliUnit::Flag(self.unit_flags),
            CliUnitKind::Unitless => CliUnit::Unitless,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bpaf_invariants() {
        cli().check_invariants(true);
    }
}
