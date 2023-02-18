#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "_serde", derive(serde::Serialize))]
pub enum DebugMode {
    /// `AC_CORRECTION` (Betaflight only)
    AcCorrection,
    /// `AC_ERROR` (Betaflight only)
    AcError,
    /// `ACC` (INAV only)
    Acc,
    /// `ACCELEROMETER` (Betaflight only)
    Accelerometer,
    /// `ACRO_TRAINER` (Betaflight only)
    AcroTrainer,
    /// `ADC_INTERNAL` (Betaflight only)
    AdcInternal,
    /// `AGL` (INAV only)
    Agl,
    /// `ALTITUDE`
    Altitude,
    /// `ALWAYS` (INAV only)
    Always,
    /// `ANGLERATE` (Betaflight only)
    Anglerate,
    /// `ANTI_GRAVITY` (Betaflight only)
    AntiGravity,
    /// `ATTITUDE` (Betaflight only)
    Attitude,
    /// `AUTOLEVEL` (INAV only)
    AutoLevel,
    /// `AUTOTRIM` (INAV only)
    AutoTrim,
    /// `AUTOTUNE` (INAV only)
    AutoTune,
    /// `BARO` (Betaflight only)
    Baro,
    /// `BATTERY` (Betaflight only)
    Battery,
    /// `BLACKBOX_OUTPUT` (Betaflight only)
    BlackboxOutput,
    /// `CRSF_LINK_STATISTICS_DOWN` (Betaflight only)
    CrsfLinkStatisticsDown,
    /// `CRSF_LINK_STATISTICS_PWR` (Betaflight only)
    CrsfLinkStatisticsPwr,
    /// `CRSF_LINK_STATISTICS_UPLINK` (Betaflight only)
    CrsfLinkStatisticsUplink,
    /// `CRUISE` (INAV only)
    Cruise,
    /// `CURRENT_SENSOR` (Betaflight only)
    CurrentSensor,
    /// `CYCLETIME` (Betaflight only)
    Cycletime,
    /// `D_LPF` (Betaflight only)
    DLpf,
    /// `D_MIN` (Betaflight only)
    DMin,
    /// `DSHOT_RPM_ERRORS` (Betaflight only)
    DshotRpmErrors,
    /// `DSHOT_RPM_TELEMETRY` (Betaflight only)
    DshotRpmTelemetry,
    /// `DUAL_GYRO_DIFF` (Betaflight only)
    DualGyroDiff,
    /// `DUAL_GYRO_RAW` (Betaflight only)
    DualGyroRaw,
    /// `DUAL_GYRO_SCALED` (Betaflight only)
    DualGyroScaled,
    /// `DYN_IDLE` (Betaflight only)
    DynIdle,
    /// `DYN_LPF` (Betaflight only)
    DynLpf,
    /// `DYNAMIC_GYRO_LPF` (INAV only)
    DynamicGyroLpf,
    /// `ESC_SENSOR` (Betaflight only)
    EscSensor,
    /// `ESC_SENSOR_RPM` (Betaflight only)
    EscSensorRpm,
    /// `ESC_SENSOR_TMP` (Betaflight only)
    EscSensorTmp,
    /// `FEEDFORWARD` (Betaflight only)
    Feedforward,
    /// `FEEDFORWARD_LIMIT` (Betaflight only)
    FeedforwardLimit,
    /// `FFT` (Betaflight only)
    Fft,
    /// `FFT_FREQ` (Betaflight only)
    FftFreq,
    /// `FFT_TIME` (Betaflight only)
    FftTime,
    /// `FLOW` (INAV only)
    Flow,
    /// `FLOW_RAW` (INAV only)
    FlowRaw,
    /// `FPORT` (Betaflight only)
    Fport,
    /// `GHST` (Betaflight only)
    Ghst,
    /// `GHST_MSP` (Betaflight only)
    GhstMsp,
    /// `GPS_RESCUE_HEADING` (Betaflight only)
    GpsRescueHeading,
    /// `GPS_RESCUE_THROTTLE_PID` (Betaflight only)
    GpsRescueThrottlePid,
    /// `GPS_RESCUE_TRACKING` (Betaflight only)
    GpsRescueTracking,
    /// `GPS_RESCUE_VELOCITY` (Betaflight only)
    GpsRescueVelocity,
    /// `GYRO_FILTERED` (Betaflight only)
    GyroFiltered,
    /// `GYRO_RAW` (Betaflight only)
    GyroRaw,
    /// `GYRO_SAMPLE` (Betaflight only)
    GyroSample,
    /// `GYRO_SCALED` (Betaflight only)
    GyroScaled,
    /// `ITERM_RELAX` (Betaflight only)
    ItermRelax,
    /// `LANDING` (INAV only)
    Landing,
    /// `LIDAR_TF` (Betaflight only)
    LidarTf,
    /// `MAX745_SIGNAL` (Betaflight only)
    Max745Signal,
    /// `MAX745_SPICLOCK` (Betaflight only)
    Max745Spiclock,
    /// `NAV_YAW` (INAV only)
    NavYaw,
    /// `NONE`
    None,
    /// `PCF8574` (INAV only)
    Pcf8574,
    /// `PIDLOOP` (Betaflight only)
    Pidloop,
    /// `RANGEFINDER` (Betaflight only)
    Rangefinder,
    /// `RANGEFINDER_QUALITY` (Betaflight only)
    RangefinderQuality,
    /// `RATE_DYNAMICS` (INAV only)
    RateDynamics,
    /// `RC_INTERPOLATION` (Betaflight only)
    RcInterpolation,
    /// `RC_SMOOTHING` (Betaflight only)
    RcSmoothing,
    /// `RC_SMOOTHING_RATE` (Betaflight only)
    RcSmoothingRate,
    /// `REM_FLIGHT_TIME` (INAV only)
    RemFlightTime,
    /// `RPM_FILTER` (Betaflight only)
    RpmFilter,
    /// `RTH` (Betaflight only)
    Rth,
    /// `RUNAWAY_TAKEOFF` (Betaflight only)
    RunawayTakeoff,
    /// `RX_EXPRESSLRS_PHASELOCK` (Betaflight only)
    RxExpresslrsPhaselock,
    /// `RX_EXPRESSLRS_SPI` (Betaflight only)
    RxExpresslrsSpi,
    /// `RX_FRSKY_SPI` (Betaflight only)
    RxFrskySpi,
    /// `RX_SFHSS_SPI` (Betaflight only)
    RxSfhssSpi,
    /// `RX_SIGNAL_LOSS` (Betaflight only)
    RxSignalLoss,
    /// `RX_SPEKTRUM_SPI` (Betaflight only)
    RxSpektrumSpi,
    /// `RX_STATE_TIME` (Betaflight only)
    RxStateTime,
    /// `RX_TIMING` (Betaflight only)
    RxTiming,
    /// `SAG_COMP_VOLTAGE` (INAV only)
    SagCompVoltage,
    /// `SBUS` (Betaflight only)
    Sbus,
    /// `SCHEDULER` (Betaflight only)
    Scheduler,
    /// `SCHEDULER_DETERMINISM` (Betaflight only)
    SchedulerDeterminism,
    /// `SDIO` (Betaflight only)
    Sdio,
    /// `SMARTAUDIO`
    Smartaudio,
    /// `STACK` (Betaflight only)
    Stack,
    /// `TIMING_ACCURACY` (Betaflight only)
    TimingAccuracy,
    /// `USB` (Betaflight only)
    Usb,
    /// `VIBE` (INAV only)
    Vibe,
    /// `VTX_MSP` (Betaflight only)
    VtxMsp,
    /// `VTX_TRAMP` (Betaflight only)
    VtxTramp,
}
#[allow(unused_qualifications)]
impl crate::units::Flag for DebugMode {
    fn as_name(&self) -> &'static str {
        match self {
            Self::AcCorrection => "AC_CORRECTION",
            Self::AcError => "AC_ERROR",
            Self::Acc => "ACC",
            Self::Accelerometer => "ACCELEROMETER",
            Self::AcroTrainer => "ACRO_TRAINER",
            Self::AdcInternal => "ADC_INTERNAL",
            Self::Agl => "AGL",
            Self::Altitude => "ALTITUDE",
            Self::Always => "ALWAYS",
            Self::Anglerate => "ANGLERATE",
            Self::AntiGravity => "ANTI_GRAVITY",
            Self::Attitude => "ATTITUDE",
            Self::AutoLevel => "AUTOLEVEL",
            Self::AutoTrim => "AUTOTRIM",
            Self::AutoTune => "AUTOTUNE",
            Self::Baro => "BARO",
            Self::Battery => "BATTERY",
            Self::BlackboxOutput => "BLACKBOX_OUTPUT",
            Self::CrsfLinkStatisticsDown => "CRSF_LINK_STATISTICS_DOWN",
            Self::CrsfLinkStatisticsPwr => "CRSF_LINK_STATISTICS_PWR",
            Self::CrsfLinkStatisticsUplink => "CRSF_LINK_STATISTICS_UPLINK",
            Self::Cruise => "CRUISE",
            Self::CurrentSensor => "CURRENT_SENSOR",
            Self::Cycletime => "CYCLETIME",
            Self::DLpf => "D_LPF",
            Self::DMin => "D_MIN",
            Self::DshotRpmErrors => "DSHOT_RPM_ERRORS",
            Self::DshotRpmTelemetry => "DSHOT_RPM_TELEMETRY",
            Self::DualGyroDiff => "DUAL_GYRO_DIFF",
            Self::DualGyroRaw => "DUAL_GYRO_RAW",
            Self::DualGyroScaled => "DUAL_GYRO_SCALED",
            Self::DynIdle => "DYN_IDLE",
            Self::DynLpf => "DYN_LPF",
            Self::DynamicGyroLpf => "DYNAMIC_GYRO_LPF",
            Self::EscSensor => "ESC_SENSOR",
            Self::EscSensorRpm => "ESC_SENSOR_RPM",
            Self::EscSensorTmp => "ESC_SENSOR_TMP",
            Self::Feedforward => "FEEDFORWARD",
            Self::FeedforwardLimit => "FEEDFORWARD_LIMIT",
            Self::Fft => "FFT",
            Self::FftFreq => "FFT_FREQ",
            Self::FftTime => "FFT_TIME",
            Self::Flow => "FLOW",
            Self::FlowRaw => "FLOW_RAW",
            Self::Fport => "FPORT",
            Self::Ghst => "GHST",
            Self::GhstMsp => "GHST_MSP",
            Self::GpsRescueHeading => "GPS_RESCUE_HEADING",
            Self::GpsRescueThrottlePid => "GPS_RESCUE_THROTTLE_PID",
            Self::GpsRescueTracking => "GPS_RESCUE_TRACKING",
            Self::GpsRescueVelocity => "GPS_RESCUE_VELOCITY",
            Self::GyroFiltered => "GYRO_FILTERED",
            Self::GyroRaw => "GYRO_RAW",
            Self::GyroSample => "GYRO_SAMPLE",
            Self::GyroScaled => "GYRO_SCALED",
            Self::ItermRelax => "ITERM_RELAX",
            Self::Landing => "LANDING",
            Self::LidarTf => "LIDAR_TF",
            Self::Max745Signal => "MAX745_SIGNAL",
            Self::Max745Spiclock => "MAX745_SPICLOCK",
            Self::NavYaw => "NAV_YAW",
            Self::None => "NONE",
            Self::Pcf8574 => "PCF8574",
            Self::Pidloop => "PIDLOOP",
            Self::Rangefinder => "RANGEFINDER",
            Self::RangefinderQuality => "RANGEFINDER_QUALITY",
            Self::RateDynamics => "RATE_DYNAMICS",
            Self::RcInterpolation => "RC_INTERPOLATION",
            Self::RcSmoothing => "RC_SMOOTHING",
            Self::RcSmoothingRate => "RC_SMOOTHING_RATE",
            Self::RemFlightTime => "REM_FLIGHT_TIME",
            Self::RpmFilter => "RPM_FILTER",
            Self::Rth => "RTH",
            Self::RunawayTakeoff => "RUNAWAY_TAKEOFF",
            Self::RxExpresslrsPhaselock => "RX_EXPRESSLRS_PHASELOCK",
            Self::RxExpresslrsSpi => "RX_EXPRESSLRS_SPI",
            Self::RxFrskySpi => "RX_FRSKY_SPI",
            Self::RxSfhssSpi => "RX_SFHSS_SPI",
            Self::RxSignalLoss => "RX_SIGNAL_LOSS",
            Self::RxSpektrumSpi => "RX_SPEKTRUM_SPI",
            Self::RxStateTime => "RX_STATE_TIME",
            Self::RxTiming => "RX_TIMING",
            Self::SagCompVoltage => "SAG_COMP_VOLTAGE",
            Self::Sbus => "SBUS",
            Self::Scheduler => "SCHEDULER",
            Self::SchedulerDeterminism => "SCHEDULER_DETERMINISM",
            Self::Sdio => "SDIO",
            Self::Smartaudio => "SMARTAUDIO",
            Self::Stack => "STACK",
            Self::TimingAccuracy => "TIMING_ACCURACY",
            Self::Usb => "USB",
            Self::Vibe => "VIBE",
            Self::VtxMsp => "VTX_MSP",
            Self::VtxTramp => "VTX_TRAMP",
        }
    }
}
impl ::core::fmt::Display for DebugMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let s = <Self as crate::units::Flag>::as_name(self);
        f.write_str(s)
    }
}
#[allow(
    unused_qualifications,
    clippy::match_same_arms,
    clippy::unseparated_literal_suffix
)]
impl DebugMode {
    pub(crate) fn new(raw: u32, fw: crate::headers::InternalFirmware) -> Option<Self> {
        match raw {
            0u32 => Some(Self::None),
            1u32 if fw.is_betaflight() => Some(Self::Cycletime),
            1u32 if fw.is_inav() => Some(Self::Agl),
            2u32 if fw.is_betaflight() => Some(Self::Battery),
            2u32 if fw.is_inav() => Some(Self::FlowRaw),
            3u32 if fw.is_betaflight() => Some(Self::GyroFiltered),
            3u32 if fw.is_inav() => Some(Self::Flow),
            4u32 if fw.is_betaflight() => Some(Self::Accelerometer),
            4u32 if fw.is_inav() => Some(Self::Always),
            5u32 if fw.is_betaflight() => Some(Self::Pidloop),
            5u32 if fw.is_inav() => Some(Self::SagCompVoltage),
            6u32 if fw.is_betaflight() => Some(Self::GyroScaled),
            6u32 if fw.is_inav() => Some(Self::Vibe),
            7u32 if fw.is_betaflight() => Some(Self::RcInterpolation),
            7u32 if fw.is_inav() => Some(Self::Cruise),
            8u32 if fw.is_betaflight() => Some(Self::Anglerate),
            8u32 if fw.is_inav() => Some(Self::RemFlightTime),
            9u32 if fw.is_betaflight() => Some(Self::EscSensor),
            9u32 if fw.is_inav() => Some(Self::Smartaudio),
            10u32 if fw.is_betaflight() => Some(Self::Scheduler),
            10u32 if fw.is_inav() => Some(Self::Acc),
            11u32 if fw.is_betaflight() => Some(Self::Stack),
            11u32 if fw.is_inav() => Some(Self::NavYaw),
            12u32 if fw.is_betaflight() => Some(Self::EscSensorRpm),
            12u32 if fw.is_inav() => Some(Self::Pcf8574),
            13u32 if fw.is_betaflight() => Some(Self::EscSensorTmp),
            13u32 if fw.is_inav() => Some(Self::DynamicGyroLpf),
            14u32 if fw.is_betaflight() => Some(Self::Altitude),
            14u32 if fw.is_inav() => Some(Self::AutoLevel),
            15u32 if fw.is_betaflight() => Some(Self::Fft),
            15u32 if fw.is_inav() => Some(Self::Altitude),
            16u32 if fw.is_betaflight() => Some(Self::FftTime),
            16u32 if fw.is_inav() => Some(Self::AutoTrim),
            17u32 if fw.is_betaflight() => Some(Self::FftFreq),
            17u32 if fw.is_inav() => Some(Self::AutoTune),
            18u32 if fw.is_betaflight() => Some(Self::RxFrskySpi),
            18u32 if fw.is_inav() => Some(Self::RateDynamics),
            19u32 if fw.is_betaflight() => Some(Self::RxSfhssSpi),
            19u32 if fw.is_inav() => Some(Self::Landing),
            20u32 if fw.is_betaflight() => Some(Self::GyroRaw),
            21u32 if fw.is_betaflight() => Some(Self::DualGyroRaw),
            22u32 if fw.is_betaflight() => Some(Self::DualGyroDiff),
            23u32 if fw.is_betaflight() => Some(Self::Max745Signal),
            24u32 if fw.is_betaflight() => Some(Self::Max745Spiclock),
            25u32 if fw.is_betaflight() => Some(Self::Sbus),
            26u32 if fw.is_betaflight() => Some(Self::Fport),
            27u32 if fw.is_betaflight() => Some(Self::Rangefinder),
            28u32 if fw.is_betaflight() => Some(Self::RangefinderQuality),
            29u32 if fw.is_betaflight() => Some(Self::LidarTf),
            30u32 if fw.is_betaflight() => Some(Self::AdcInternal),
            31u32 if fw.is_betaflight() => Some(Self::RunawayTakeoff),
            32u32 if fw.is_betaflight() => Some(Self::Sdio),
            33u32 if fw.is_betaflight() => Some(Self::CurrentSensor),
            34u32 if fw.is_betaflight() => Some(Self::Usb),
            35u32 if fw.is_betaflight() => Some(Self::Smartaudio),
            36u32 if fw.is_betaflight() => Some(Self::Rth),
            37u32 if fw.is_betaflight() => Some(Self::ItermRelax),
            38u32 if fw.is_betaflight() => Some(Self::AcroTrainer),
            39u32 if fw.is_betaflight() => Some(Self::RcSmoothing),
            40u32 if fw.is_betaflight() => Some(Self::RxSignalLoss),
            41u32 if fw.is_betaflight() => Some(Self::RcSmoothingRate),
            42u32 if fw.is_betaflight() => Some(Self::AntiGravity),
            43u32 if fw.is_betaflight() => Some(Self::DynLpf),
            44u32 if fw.is_betaflight() => Some(Self::RxSpektrumSpi),
            45u32 if fw.is_betaflight() => Some(Self::DshotRpmTelemetry),
            46u32 if fw.is_betaflight() => Some(Self::RpmFilter),
            47u32 if fw.is_betaflight() => Some(Self::DMin),
            48u32 if fw.is_betaflight() => Some(Self::AcCorrection),
            49u32 if fw.is_betaflight() => Some(Self::AcError),
            50u32 if fw.is_betaflight() => Some(Self::DualGyroScaled),
            51u32 if fw.is_betaflight() => Some(Self::DshotRpmErrors),
            52u32 if fw.is_betaflight() => Some(Self::CrsfLinkStatisticsUplink),
            53u32 if fw.is_betaflight() => Some(Self::CrsfLinkStatisticsPwr),
            54u32 if fw.is_betaflight() => Some(Self::CrsfLinkStatisticsDown),
            55u32 if fw.is_betaflight() => Some(Self::Baro),
            56u32 if fw.is_betaflight() => Some(Self::GpsRescueThrottlePid),
            57u32 if fw.is_betaflight() => Some(Self::DynIdle),
            58u32 if fw.is_betaflight() => Some(Self::FeedforwardLimit),
            59u32 if fw.is_betaflight() => Some(Self::Feedforward),
            60u32 if fw.is_betaflight() => Some(Self::BlackboxOutput),
            61u32 if fw.is_betaflight() => Some(Self::GyroSample),
            62u32 if fw.is_betaflight() => Some(Self::RxTiming),
            63u32 if fw.is_betaflight() => Some(Self::DLpf),
            64u32 if fw.is_betaflight() => Some(Self::VtxTramp),
            65u32 if fw.is_betaflight() => Some(Self::Ghst),
            66u32 if fw.is_betaflight() => Some(Self::GhstMsp),
            67u32 if fw.is_betaflight() => Some(Self::SchedulerDeterminism),
            68u32 if fw.is_betaflight() => Some(Self::TimingAccuracy),
            69u32 if fw.is_betaflight() => Some(Self::RxExpresslrsSpi),
            70u32 if fw.is_betaflight() => Some(Self::RxExpresslrsPhaselock),
            71u32 if fw.is_betaflight() => Some(Self::RxStateTime),
            72u32 if fw.is_betaflight() => Some(Self::GpsRescueVelocity),
            73u32 if fw.is_betaflight() => Some(Self::GpsRescueHeading),
            74u32 if fw.is_betaflight() => Some(Self::GpsRescueTracking),
            75u32 if fw.is_betaflight() => Some(Self::Attitude),
            76u32 if fw.is_betaflight() => Some(Self::VtxMsp),
            _ => {
                #[allow(clippy::redundant_closure_call)]
                (|raw| tracing::error!("invalid debug mode: {raw}"))(raw);
                None
            }
        }
    }
}
