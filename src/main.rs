mod helpers;
mod jobs;
mod net;
mod packets;
mod plugins;
mod threads;

use helpers::unwrap_and_parse_or_default;
use plugins::LuaPluginInterface;

fn main() {
    let matches = clap::App::new("OpenNetBattle Server")
        .arg(
            clap::Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .default_value("8765")
                .required(true)
                .takes_value(true)
                .validator(|value| {
                    let error_message = "PORT must be > 0 and < 65535";
                    let port = value
                        .parse::<u16>()
                        .map_err(|_| String::from(error_message))?;

                    if port != 0 {
                        Ok(())
                    } else {
                        Err(String::from(error_message))
                    }
                }),
        )
        .arg(
            clap::Arg::with_name("log_connections")
                .long("log-connections")
                .help("Logs connects and disconnects"),
        )
        .arg(
            clap::Arg::with_name("log_packets")
                .long("log-packets")
                .help("Logs received packets (useful for debugging)"),
        )
        .arg(
            clap::Arg::with_name("max_payload_size")
                .long("max-payload-size")
                .help("Maximum data size a packet can carry, excluding UDP headers (reduce for lower packet drop rate)")
                .value_name("SIZE_IN_BYTES")
                .default_value("1400")
                .takes_value(true)
                .validator(|value| {
                    let error_message = "Invalid payload size";
                    let max_payload_size = value
                        .parse::<u16>()
                        .map_err(|_| String::from(error_message))?;

                    // max size defined by NetPlayConfig::MAX_BUFFER_LEN
                    if (100..=10240).contains(&max_payload_size) {
                        Ok(())
                    } else {
                        Err(String::from(error_message))
                    }
                }),
        )
        .arg(
            clap::Arg::with_name("resend_budget")
                .long("resend-budget")
                .help("Budget of bytes each client has for the server to spend on resending packets")
                .value_name("SIZE_IN_BYTES")
                .default_value("65536") // nearest power of a power of two to (test data / 2 skips / 2 for safety / 2 reliability types)
                .takes_value(true)
                .validator(|value| {
                    let error_message = "Invalid size";

                    let resend_budget = value.parse::<isize>().map_err(|_| String::from(error_message))?;

                    if resend_budget < 0 {
                        Err(String::from(error_message))
                    } else {
                        Ok(())
                    }
                }),
        )
        .arg(
            clap::Arg::with_name("player_asset_limit")
                .long("player-asset-limit")
                .help("Sets the file size limit for avatar files (in KiB)")
                .value_name("SIZE_IN_KiB")
                .default_value("50")
                .takes_value(true)
                .validator(|value| match value.parse::<usize>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(String::from("Invalid file size")),
                }),
        )
        .arg(
            clap::Arg::with_name("avatar_dimensions_limit")
                .long("avatar-dimensions-limit")
                .help("Sets the limit for dimensions of a single avatar frame")
                .value_name("SIDE_LENGTH")
                .default_value("80")
                .takes_value(true)
                .validator(|value| match value.parse::<u32>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(String::from("Invalid length")),
                }),
        )
        .arg(
            clap::Arg::with_name("worker_thread_count")
                .long("worker-threads")
                .help("Sets the amount of threads to spawn for handling special tasks")
                .default_value("2")
                .validator(|value| match value.parse::<u16>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(String::from("Invalid quantity")),
                }),
        )
        .get_matches();

    let config = net::ServerConfig {
        // validators makes this safe to unwrap
        port: matches.value_of("port").unwrap().parse().unwrap(),
        log_connections: matches.is_present("log_connections"),
        log_packets: matches.is_present("log_packets"),
        max_payload_size: unwrap_and_parse_or_default(matches.value_of("max_payload_size")),
        resend_budget: matches.value_of("port").unwrap().parse().unwrap(),
        player_asset_limit: unwrap_and_parse_or_default::<usize>(
            matches.value_of("player_asset_limit"),
        ) * 1024,
        avatar_dimensions_limit: unwrap_and_parse_or_default(
            matches.value_of("avatar_dimensions_limit"),
        ),
        worker_thread_count: unwrap_and_parse_or_default(matches.value_of("worker_thread_count")),
    };

    let mut server = net::Server::new(config);

    server.add_plugin_interface(Box::new(LuaPluginInterface::new()));

    if let Err(err) = server.start() {
        panic!("{}", err);
    }
}
