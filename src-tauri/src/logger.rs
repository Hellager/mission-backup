use log4rs::{
    append::{
        console::{ ConsoleAppender, Target },
        rolling_file::{
            RollingFileAppender,
            policy::compound::{
                CompoundPolicy,
                roll::fixed_window::FixedWindowRoller,
                // roll::delete::DeleteRoller,
                trigger::size::SizeTrigger,
            }
        }
    },
    config::{ Appender, Config, Root },
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use log::{ debug, LevelFilter };
use path_absolutize::*;
use std::path::Path;

/**
 * Initialize programe logger
 * to specific log pattern, see https://docs.rs/log4rs/latest/log4rs/encode/pattern/
 */
pub fn initialize_logger(log_file_path: &str) {
    // define log level filter
    let global_log_level = LevelFilter::Debug;
    let stdout_log_level = LevelFilter::Debug;
    let logfile_log_level = LevelFilter::Info;
    let strong_level_log_level = LevelFilter::Warn;

    // define log pattern
    // let log_pattern = "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} | {({l}):5.5} | {f} {m}{n}";
    let log_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {m}{n}";

    // define roller trigger size
    let trigger_size = byte_unit::n_mb_bytes!(6) as u64;
    let trigger = SizeTrigger::new(trigger_size);

    // rolling update log, when trigger, it will split to new file
    let roller_pattern = "logs/log - {}.log"; // new log file format which means will generate files 1.log 2.log ...
    let roller_count = 3; // total log files, after reach count, it will use the earliest created one to log, like use 1.log after 3.log has reached trigger size
    let roller_base = 1;  // log file index, default is 0
    let roller = FixedWindowRoller::builder()
                .base(roller_base)
                .build(roller_pattern, roller_count).unwrap();

    let strong_level_roller = FixedWindowRoller::builder()
                .base(roller_base)
                .build(roller_pattern, roller_count).unwrap();

    // delete update log, when trigger, it will delete the old file and create an empty new one
    // let delete_roller = DeleteRoller::new();

    // define different plicies
    let log_file_compound_policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
    let strong_level_compound_policy = CompoundPolicy::new(Box::new(trigger), Box::new(strong_level_roller));
    // let compound_policy = CompoundPolicy::new(Box::new(trigger), Box::new(delete_roller));

    // output logs to specific path, if not exists, it will automatically create
    let log_file = RollingFileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(log_pattern)))
                    .build(log_file_path, Box::new(log_file_compound_policy))
                    .unwrap();

    // output warns and errors to a different path
    let absolute_path = Path::new(log_file_path).absolutize().unwrap();
    let parent_path = absolute_path.parent().unwrap();
    let strong_level_path = parent_path.join("strong_level.log").into_os_string().into_string().unwrap();
    let strong_level_log_file = RollingFileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(log_pattern)))
                    .build(strong_level_path, Box::new(strong_level_compound_policy))
                    .unwrap();

    // output logs to stdout
    let stdout = ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(log_pattern)))
                    .target(Target::Stdout)
                    .build();
    
    // set up log4rs configuration
    let config = Config::builder()
                    .appender(
                        Appender::builder()
                            .filter(Box::new(ThresholdFilter::new(logfile_log_level)))
                            .build("log_file", Box::new(log_file)))
                    .appender(
                        Appender::builder()
                        .filter(Box::new(ThresholdFilter::new(strong_level_log_level)))
                            .build("strong_level", Box::new(strong_level_log_file))
                    )
                    .appender(
                        Appender::builder()
                            .filter(Box::new(ThresholdFilter::new(stdout_log_level)))
                            .build("stdout", Box::new(stdout)),
                    )
                    .build(
                        Root::builder()
                            .appender("stdout")
                            .appender("log_file")
                            .appender("strong_level")
                            .build(global_log_level))
                    .unwrap();

    let _log_handler = log4rs::init_config(config).unwrap();

    debug!("Programe logger initialize success");  
}