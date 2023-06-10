use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub region_code: Option<String>,
    pub ip: usize,
    pub postal_code: Option<String>,
    pub country_code: String,
    pub city: Option<String>,
    pub dma_code: Option<String>,

    #[serde(with = "date_format")]
    pub last_update: chrono::DateTime<Utc>,
    pub latitude: f32,
    pub tags: Vec<String>,
    pub area_code: Option<usize>,
    pub country_name: String,
    pub hostnames: Vec<String>,
    pub org: String,
    pub data: Vec<Data>,
    pub asn: String,
    pub isp: String,
    pub longitude: f32,
    pub country_code3: Option<String>,
    pub domains: Vec<String>,
    pub ip_str: String,
    pub os: Option<String>,
    pub ports: Vec<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    #[serde(rename = "_shodan")]
    pub shodan: Shodan,
    pub hash: isize,
    pub os: Option<String>,
    pub opts: Opts,
    pub ip: usize,
    pub isp: String,
    pub port: usize,
    pub hostnames: Vec<String>,
    pub location: Location,
    pub dns: Option<Dns>,

    #[serde(with = "date_format")]
    pub timestamp: chrono::DateTime<Utc>,
    pub domain: Option<Vec<String>>,
    pub org: String,
    pub data: String,
    pub asn: String,
    pub transport: String,
    pub ip_str: String,
}

#[derive(Debug, Deserialize)]
pub struct Shodan {
    pub id: String,
    pub options: Options,
    pub ptr: bool,
    pub module: String,
    pub crawler: String,
}

#[derive(Debug, Deserialize)]
pub struct Opts {
    pub raw: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub city: Option<String>,
    pub region_code: Option<String>,
    pub area_code: Option<usize>,
    pub longitude: f32,
    pub country_code3: Option<String>,
    pub postal_code: Option<usize>,
    pub dma_code: Option<String>,
    pub country_code: String,
    pub latitude: f32,
}

#[derive(Debug, Deserialize)]
pub struct Dns {
    pub resolver_hostname: Option<String>,
    pub recursive: bool,
    pub resolver_id: Option<String>,
    pub software: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Options {}

mod date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let datetime =
            chrono::NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;

        Ok(DateTime::<Utc>::from_utc(datetime, Utc))
    }
}
