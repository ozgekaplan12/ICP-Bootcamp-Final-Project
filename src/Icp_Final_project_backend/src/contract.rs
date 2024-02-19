#![cfg_attr(not(feature = "std"), no_std)]

use ic_cdk::export::candid::CandidType;
use ic_cdk::storage;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize)]
struct WeatherState {
    temperature: i32,
    pressure: i32,
    humidity: i32,
    wind_speed: i32,
}

#[ic_cdk_macros::query]
fn get_temperature() -> i32 {
    match storage::get::<WeatherState>("weather_state") {
        Some(state) => state.temperature,
        None => 0, // Return default value if state not found
    }
}

#[ic_cdk_macros::query]
fn get_pressure() -> i32 {
    match storage::get::<WeatherState>("weather_state") {
        Some(state) => state.pressure,
        None => 0, // Return default value if state not found
    }
}

#[ic_cdk_macros::query]
fn get_humidity() -> i32 {
    match storage::get::<WeatherState>("weather_state") {
        Some(state) => state.humidity,
        None => 0, // Return default value if state not found
    }
}

#[ic_cdk_macros::query]
fn get_wind_speed() -> i32 {
    match storage::get::<WeatherState>("weather_state") {
        Some(state) => state.wind_speed,
        None => 0, // Return default value if state not found
    }
}

#[ic_cdk_macros::update]
fn set_weather_data(temperature: i32, pressure: i32, humidity: i32, wind_speed: i32) {
    let state = WeatherState {
        temperature,
        pressure,
        humidity,
        wind_speed,
    };
    storage::set("weather_state", state);
}

#[ic_cdk_macros::update]
fn update_temperature(new_temperature: i32) {
    if let Some(mut state) = storage::get::<WeatherState>("weather_state") {
        state.temperature = new_temperature;
        storage::set("weather_state", state);
    }
}

#[ic_cdk_macros::update]
fn update_pressure(new_pressure: i32) {
    if let Some(mut state) = storage::get::<WeatherState>("weather_state") {
        state.pressure = new_pressure;
        storage::set("weather_state", state);
    }
}

#[ic_cdk_macros::update]
fn update_humidity(new_humidity: i32) {
    if let Some(mut state) = storage::get::<WeatherState>("weather_state") {
        state.humidity = new_humidity;
        storage::set("weather_state", state);
    }
}

#[ic_cdk_macros::update]
fn update_wind_speed(new_wind_speed: i32) {
    if let Some(mut state) = storage::get::<WeatherState>("weather_state") {
        state.wind_speed = new_wind_speed;
        storage::set("weather_state", state);
    }
}