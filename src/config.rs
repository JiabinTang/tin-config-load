use notify::{Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

pub struct ConfigLoader {
    builder: config::ConfigBuilder<config::builder::DefaultState>,
    path: String,
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self {
            builder: config::Config::builder(),
            path: String::from("./config/default"),
        }
    }
}

impl ConfigLoader {
    /// 添加文件source
    pub fn file(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self.builder = self.builder.add_source(config::File::with_name(&self.path));
        self
    }

    /// 添加环境变量source
    pub fn env(mut self, prefix: &str) -> Self {
        self.builder = self
            .builder
            .add_source(config::Environment::with_prefix(prefix));
        self
    }

    /// 添加默认文件source（可选）
    pub fn default_file(mut self) -> Self {
        self.builder = self
            .builder
            .add_source(config::File::with_name(&self.path).required(false));
        self
    }

    /// 默认环境变量
    pub fn default_env(mut self) -> Self {
        self.builder = self
            .builder
            .add_source(config::Environment::with_prefix("APP"));
        self
    }

    /// 构建并反序列化为目标类型
    pub fn build<T: serde::de::DeserializeOwned>(self) -> Result<T, config::ConfigError> {
        let s = self.builder.build()?;
        s.try_deserialize()
    }

    pub fn watch<F, Fut>(self, mut f: F) -> Self
    where
        F: FnMut(config::Config) -> Fut + Send + 'static,
        Fut: futures::Future<Output = ()> + Send + 'static,
    {
        let builder = Arc::new(Mutex::new(self.builder.clone()));
        let path = self.path.clone();

        let builder_clone = builder.clone();
        let mut watcher = RecommendedWatcher::new(
            move |result: Result<Event, Error>| {
                let event = result.unwrap();

                if event.kind.is_modify() {
                    let builder = builder_clone.lock().unwrap();
                    builder
                        .clone()
                        .build()
                        .map(|config| {
                            tokio::spawn(f(config));
                        })
                        .unwrap_or_else(|e| {
                            log::error!("Failed to reload config: {e}");
                        });
                }
            },
            notify::Config::default(),
        )
        .unwrap();

        watcher
            .watch(Path::new(&path), RecursiveMode::Recursive)
            .unwrap();
        self
    }
}
