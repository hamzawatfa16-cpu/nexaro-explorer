use notify::{
    Config,
    Event,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};

use std::path::Path;


pub struct WatcherService;


impl WatcherService {
    pub fn new() -> Self {
        Self
    }

    pub fn watch<F>(
        &self,
        path: &Path,
        callback: F,
    ) -> notify::Result<RecommendedWatcher>
    where
        F: Fn(Event) + Send + 'static,
    {
        let mut watcher = RecommendedWatcher::new(
            move |result| {
                if let Ok(event) = result {
                    callback(event);
                }
            },
            Config::default(),
        )?;

        watcher.watch(
            path,
            RecursiveMode::Recursive,
        )?;

        Ok(watcher)
    }
}