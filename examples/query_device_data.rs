use std::time::Duration;

use ambient_weather_progenitor::Client;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use textplots::{Chart, ColorPlot, Shape, TickDisplayBuilder};

#[derive(Serialize, Deserialize)]
struct Config {
    api_key: String,
    application_key: String,
}

fn farenheit_to_celcius(temperature: f32) -> f32 {
    (temperature - 32.0) * 5.0 / 9.0
}

#[tokio::main]
async fn main() {
    let client = Client::new("https://rt.ambientweather.net");
    let config: Config = toml_env::initialize(toml_env::Args {
        auto_map_env: Some(toml_env::AutoMapEnvArgs {
            prefix: Some("AMBIENT"),
            ..toml_env::AutoMapEnvArgs::default()
        }),
        ..toml_env::Args::default()
    })
    .unwrap()
    .expect("No configuration provided");

    let api_key = &config.api_key;
    let application_key = &config.application_key;

    let response = client
        .list_users_devices(api_key, application_key)
        .await
        .unwrap();
    println!("list_users_devices response: {response:#?}");

    let device_macs: Vec<_> = response
        .into_inner()
        .into_iter()
        .filter_map(|item| item.mac_address)
        .collect();

    let end_date = Utc::now();

    let graph_date = move |date: DateTime<Utc>| {
        ((date - end_date).num_seconds() as f32) / (60.0 * 60.0)
    };

    for device_mac in &device_macs {
        tokio::time::sleep(Duration::from_millis(1500)).await;
        let response = client
            .query_device_data(device_mac, api_key, application_key, Some(&end_date), None)
            .await
            .unwrap()
            .into_inner();

        println!("query_device_data response: {response:#?}");

        let tempf: Vec<_> = response
            .iter()
            .filter_map(|item| {
                Some((
                    graph_date(item.date?),
                    farenheit_to_celcius(item.tempf? as f32),
                ))
            })
            .collect();

        if !tempf.is_empty() {
            let min = tempf
                .iter()
                .map(|(x, _)| *x)
                .reduce(f32::min)
                .expect("At least one x value");

            println!("Temperature (Celcius):");
            Chart::new(180, 60, min, 0.0)
                .linecolorplot(&Shape::Lines(&tempf), (0, 0, 255).into())
                .display();
        }

        let windspeed: Vec<_> = response
            .iter()
            .filter_map(|item| {
                Some((
                    graph_date(item.date?),
                    (item.windspeedmph? * 1.60934) as f32,
                ))
            })
            .collect();
        
        if !windspeed.is_empty() {
            let min = windspeed
                .iter()
                .map(|(x, _)| *x)
                .reduce(f32::min)
                .expect("At least one x value");

            println!("Wind Speed (km/h):");
            Chart::new(180, 60, min, 0.0)
                .linecolorplot(&Shape::Lines(&windspeed), (255, 0, 0).into())
                .display();
        }
        
        let winddir: Vec<_> = response
            .iter()
            .filter_map(|item| {
                Some((
                    graph_date(item.date?),
                    item.winddir_avg10m? as f32,
                ))
            })
            .collect();
        
        if !winddir.is_empty() {
            let min = winddir
                .iter()
                .map(|(x, _)| *x)
                .reduce(f32::min)
                .expect("At least one x value");

            println!("Wind Direction (Degrees):");
            Chart::new_with_y_range(180, 60, min, 0.0, 0.0, 360.0)
                .y_tick_display(textplots::TickDisplay::Dense)
                .linecolorplot(&Shape::Lines(&winddir), (0, 255, 0).into())
                .display();
        }
    }
}
