mod cli;
mod units;

use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;
use std::process::{ExitCode, Termination};

use blackbox_log::parser::Value;
use blackbox_log::Log;
use mimalloc::MiMalloc;
use rayon::prelude::*;

use self::cli::Cli;

#[global_allocator]
static ALLOC: MiMalloc = MiMalloc;

#[derive(Debug)]
enum QuietResult<T> {
    Ok(T),
    Err(ExitCode),
}

impl<T> From<exitcode::ExitCode> for QuietResult<T> {
    fn from(code: exitcode::ExitCode) -> Self {
        Self::Err(ExitCode::from(u8::try_from(code).unwrap()))
    }
}

impl Termination for QuietResult<()> {
    fn report(self) -> ExitCode {
        match self {
            Self::Ok(()) => ExitCode::SUCCESS,
            Self::Err(code) => code,
        }
    }
}

fn main() -> QuietResult<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(cli.verbosity)
        .init();

    if cli.logs.len() > 1 && cli.stdout {
        tracing::error!("cannot write multiple logs to stdout");
        return QuietResult::from(exitcode::USAGE);
    }

    let result = cli.logs.par_iter().try_for_each(|filename| {
        let span = tracing::info_span!("file", name = ?filename);
        let _span = span.enter();

        let data = read_log_file(filename).map_err(|error| {
            tracing::error!(%error, "failed to read log file");
            exitcode::IOERR
        })?;

        let file = blackbox_log::File::new(&data);

        let log_count = file.log_count();
        if cli.stdout && log_count > 1 && cli.index.len() != 1 {
            tracing::error!(
                "found {log_count} logs, choose exactly one to write to stdout with `--index`"
            );
            return Err(exitcode::USAGE);
        }

        (0..log_count).into_par_iter().try_for_each(|i| {
            let human_i = i + 1;

            let span = tracing::info_span!("log", index = human_i);
            let _span = span.enter();

            let mut log = file.parse_by_index(i).map_err(|_| exitcode::DATAERR)?;

            if let Some(ref filter) = cli.filter {
                log.set_filter(filter);
            }

            let mut out = match get_output(cli.stdout, filename, human_i) {
                Ok(out) => BufWriter::new(out),
                Err(error) => {
                    tracing::error!(%error, "failed to open output file");
                    return Err(exitcode::CANTCREAT);
                }
            };

            if let Err(error) = write_csv(&mut out, &log, &cli) {
                tracing::error!(%error, "failed to write csv");
                return Err(exitcode::IOERR);
            }

            Ok(())
        })
    });

    if let Err(code) = result {
        QuietResult::from(code)
    } else {
        QuietResult::Ok(())
    }
}

fn read_log_file(filename: &Path) -> io::Result<Vec<u8>> {
    let mut log = File::open(filename)?;
    let mut data = Vec::new();
    log.read_to_end(&mut data)?;
    Ok(data)
}

fn get_output(stdout: bool, filename: &Path, index: usize) -> io::Result<Box<dyn Write>> {
    if stdout {
        Ok(Box::new(io::stdout().lock()))
    } else {
        let mut out = filename.to_owned();
        out.set_extension(format!("{index:0>2}.csv"));
        tracing::info!("Writing log to '{}'", out.display());
        Ok(Box::new(File::create(out)?))
    }
}

fn write_csv(out: &mut impl Write, log: &Log, config: &Cli) -> io::Result<()> {
    write_csv_line(
        out,
        log.iter_fields().map(|(name, unit)| {
            let unit = config.get_unit(unit.into());
            if unit.is_raw() {
                name.to_owned()
            } else {
                format!("{name} ({unit})")
            }
        }),
    )?;

    for frame in log.iter_frames() {
        write_csv_line(
            out,
            frame.map(|value| match value {
                Value::FrameTime(t) => config.unit_frame_time.format(t),
                Value::Amperage(a) => config.unit_amperage.format(a),
                Value::Voltage(v) => config.unit_vbat.format(v),
                Value::Acceleration(a) => config.unit_acceleration.format(a),
                Value::Rotation(r) => config.unit_rotation.format(r),
                Value::FlightMode(m) => config.unit_flags.format(m),
                Value::State(s) => config.unit_flags.format(s),
                Value::FailsafePhase(p) => config.unit_flags.format(p),
                Value::Boolean(b) => config.unit_bool.format(b),
                Value::Unsigned(x) => x.to_string(),
                Value::Signed(x) => x.to_string(),
            }),
        )?;
    }

    out.flush()
}

fn write_csv_line(
    out: &mut impl Write,
    mut fields: impl Iterator<Item = String>,
) -> io::Result<()> {
    if let Some(first) = fields.next() {
        out.write_all(first.as_bytes())?;

        for s in fields {
            out.write_all(b",")?;
            out.write_all(s.as_bytes())?;
        }
    }

    out.write_all(b"\n")
}
