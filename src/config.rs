pub struct ConfigLoader {
    builder: config::ConfigBuilder<config::builder::DefaultState>,
    path: String,
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self {
            builder: config::Config::builder()
                .add_source(config::File::with_name("./config/default").required(false))
                .add_source(config::Environment::with_prefix("APP")),
            path: String::from("./config/default"),
        }
    }
}

impl ConfigLoader {
    /// 添加文件source
    pub fn file(mut self, path: &str) -> Self {
        log::info!("添加配置文件路径: {path}");
        self.path = path.to_string();
        self.builder = self.builder.add_source(config::File::with_name(&self.path));
        self
    }

    /// 添加环境变量source
    pub fn env(mut self, prefix: &str) -> Self {
        log::info!("添加环境变量前缀: {prefix}");
        self.builder = self
            .builder
            .add_source(config::Environment::with_prefix(prefix));
        self
    }

    /// 添加默认文件source（可选）
    pub fn default_file(mut self) -> Self {
        log::info!("添加默认文件路径: ./config/default");
        self.builder = self
            .builder
            .add_source(config::File::with_name("./config/default").required(false));
        self
    }

    /// 默认环境变量
    pub fn default_env(mut self) -> Self {
        log::info!("添加默认环境变量前缀: APP");
        self.builder = self
            .builder
            .add_source(config::Environment::with_prefix("APP"));
        self
    }

    /// 构建并反序列化为目标类型
    pub fn build<T: serde::de::DeserializeOwned>(self) -> Result<T, config::ConfigError> {
        log::info!("开始构建配置并反序列化为目标类型");
        let s = self.builder.build()?;
        match s.try_deserialize() {
            Ok(cfg) => {
                log::info!("配置反序列化成功");
                Ok(cfg)
            }
            Err(e) => {
                log::error!("配置反序列化失败: {e}");
                Err(e)
            }
        }
    }
}
