use crate::prelude::*;

pub struct CratesIo {
    client: crates_io_api::AsyncClient,
    rate_limit : std::time::Duration,
}

impl CratesIo {
    pub fn new() -> Self {
        Self::new_with_rate_limit(200)
    }
    pub fn new_with_rate_limit(rate_limit: u64) -> Self {
        let client = crates_io_api::AsyncClient::new(
            "cargo-emanate (info@aspectron.com)",
            std::time::Duration::from_millis(0),
        )
        .unwrap_or_else(|err| panic!("Unable to instantiate crates_io_api::AsyncClient: `{err}`"));

        CratesIo { client, rate_limit : std::time::Duration::from_millis(rate_limit) }
    }

    pub async fn get_latest_version(&self, name: &str) -> Result<Version> {
        let crt = self.client.get_crate(name).await?;
        let mut versions = crt
            .versions
            .iter()
            .map(|v| {
                v.num.parse::<Version>().unwrap_or_else(|err| {
                    panic!(
                        "Unable to parse version for crate `{name}` - `{}`: {err}",
                        v.num
                    );
                })
            })
            .collect::<Vec<_>>();
        // remove all versions containing suffixes
        versions.retain(|v|v.suffix.is_none());
        versions.sort_by(|a, b| b.cmp(a));
        let version = versions
            .first()
            .unwrap_or_else(|| panic!("No versions present for crate {name}"))
            .to_owned();

        std::thread::sleep(self.rate_limit);

        Ok(version)
    }
}
