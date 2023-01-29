use std::{collections::VecDeque, thread, time::Duration};

use async_recursion::async_recursion;
use serde::{de::DeserializeOwned, Serialize};
use surf::Client;

pub struct DepsFetcher {
    client: Client,
    registry: String,
}

impl DepsFetcher {
    const MAX_CONCURRENCY: u8 = 10;
    const THROTTLE_SECONDS: u64 = 1;

    pub(super) fn new(registry: String) -> Self {
        Self {
            client: Client::new(),
            registry,
        }
    }

    pub(super) async fn fetch_all<D>(
        &self,
        path: &str,
        deps: VecDeque<String>,
    ) -> Vec<surf::Result<D>>
    where
        D: DeserializeOwned + Serialize,
    {
        let mut result = vec![];
        self.buffering_fetch_all(path, deps, &mut result).await;

        result
    }

    #[async_recursion(?Send)]
    async fn buffering_fetch_all<D>(
        &self,
        path: &str,
        mut deps: VecDeque<String>,
        result: &mut Vec<surf::Result<D>>,
    ) where
        D: DeserializeOwned + Serialize,
    {
        let mut futures = vec![];
        let mut concurrency = 0;
        while let Some(dep) = deps.pop_front() {
            if concurrency >= Self::MAX_CONCURRENCY {
                break;
            }
            println!("Fetching {dep} ...");
            futures.push(
                self.client
                    .recv_json(surf::get(format!("{}{}/{}", self.registry, path, dep))),
            );
            concurrency += 1;
        }
        result.append(&mut futures_util::future::join_all(futures.into_iter()).await);

        if !deps.is_empty() {
            let d = Duration::from_secs(Self::THROTTLE_SECONDS);
            thread::sleep(d);
            self.buffering_fetch_all(path, deps, result).await;
        }
    }
}
