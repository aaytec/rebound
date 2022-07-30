mod conf;
mod node;

use log::LevelFilter;
use log::info;
use log::error;
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;

use node::master::MasterNode;

fn main() {

    setup_logger();

    let conf_file = match
    std::env::var(conf::REBOUND_CONF_FILE) 
    {
        Ok(f) => f,
        Err(_e) => {
            
            error!("Rebound File not Specified in Env");
            std::process::exit(-1)
        },
    };

    info!("Reading Conf: {}", conf_file);
    let conf = conf::parser::parse(conf_file);
    info!("Conf: {:?}", conf);
    
    MasterNode::from(&conf)
    .unwrap()
    .run();
}

fn setup_logger() {
    
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%+)(utc)} [{f}:{L}] {h({l})} {M}:{m}{n}")))
        .build();
    
    let compound_policy = CompoundPolicy::new
    (
        Box::new(SizeTrigger::new(5 * 1024 * 1024)), // 5KB as max log file size to roll
        Box::new(FixedWindowRoller::builder().build("/tmp/rebound.{}.log", 3).unwrap())
    );

    let file_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
        .build(
            std::env::var(conf::REBOUND_LOG_FILE).unwrap(),Box::new(compound_policy))
        .unwrap();

     let config = Config::builder()
        .appender(

            Appender::builder().build("stdout", Box::new(stdout))

        )
        .appender(

            Appender::builder()
            .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
            .build("file_appender", Box::new(file_appender)))

        .build(

            Root::builder()
                    .appender("stdout")
                    .appender("file_appender")
                    .build(LevelFilter::Info)

        )
        .unwrap();

    log4rs::init_config(config).unwrap();
    
}