#[test]
fn empty_test() {
    pub mod marine_test_env {
        pub mod empty_func {
            pub mod modules {
                pub mod greeting {
                    #[derive(
                        Clone,
                        Debug,
                        marine_rs_sdk_test :: internal :: serde :: Serialize,
                        marine_rs_sdk_test :: internal :: serde :: Deserialize
                    )]
                    #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                    pub struct CallParameters {
                        pub init_peer_id: String,
                        pub service_id: String,
                        pub service_creator_peer_id: String,
                        pub host_id: String,
                        pub particle_id: String,
                        pub tetraplets: Vec<Vec<SecurityTetraplet>>
                    }
                    #[derive(
                        Clone,
                        Debug,
                        marine_rs_sdk_test :: internal :: serde :: Serialize,
                        marine_rs_sdk_test :: internal :: serde :: Deserialize
                    )]
                    #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                    pub struct MountedBinaryResult {
                        pub ret_code: i32,
                        pub error: String,
                        pub stdout: Vec<u8>,
                        pub stderr: Vec<u8>
                    }
                    #[derive(
                        Clone,
                        Debug,
                        marine_rs_sdk_test :: internal :: serde :: Serialize,
                        marine_rs_sdk_test :: internal :: serde :: Deserialize
                    )]
                    #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                    pub struct MountedBinaryStringResult {
                        pub ret_code: i32,
                        pub error: String,
                        pub stdout: String,
                        pub stderr: String
                    }
                    #[derive(
                        Clone,
                        Debug,
                        marine_rs_sdk_test :: internal :: serde :: Serialize,
                        marine_rs_sdk_test :: internal :: serde :: Deserialize
                    )]
                    #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                    pub struct SecurityTetraplet {
                        pub peer_pk: String,
                        pub service_id: String,
                        pub function_name: String,
                        pub json_path: String
                    }
                    pub struct ModuleInterface {
                        marine: std::rc::Rc<
                            std::cell::RefCell<marine_rs_sdk_test::internal::AppService>,
                        >,
                    }
                    impl ModuleInterface {
                        pub fn new(
                            marine: std::rc::Rc<
                                std::cell::RefCell<marine_rs_sdk_test::internal::AppService>,
                            >
                        ) -> Self {
                            Self { marine }
                        }
                    }
                    impl ModuleInterface {}
                }
            }
            pub mod __facade_override {
                #[derive(
                    Clone,
                    Debug,
                    marine_rs_sdk_test :: internal :: serde :: Serialize,
                    marine_rs_sdk_test :: internal :: serde :: Deserialize
                )]
                #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                pub struct CallParameters {
                    pub init_peer_id: String,
                    pub service_id: String,
                    pub service_creator_peer_id: String,
                    pub host_id: String,
                    pub particle_id: String,
                    pub tetraplets: Vec<Vec<SecurityTetraplet>>
                }
                #[derive(
                    Clone,
                    Debug,
                    marine_rs_sdk_test :: internal :: serde :: Serialize,
                    marine_rs_sdk_test :: internal :: serde :: Deserialize
                )]
                #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                pub struct MountedBinaryResult {
                    pub ret_code: i32,
                    pub error: String,
                    pub stdout: Vec<u8>,
                    pub stderr: Vec<u8>
                }
                #[derive(
                    Clone,
                    Debug,
                    marine_rs_sdk_test :: internal :: serde :: Serialize,
                    marine_rs_sdk_test :: internal :: serde :: Deserialize
                )]
                #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                pub struct MountedBinaryStringResult {
                    pub ret_code: i32,
                    pub error: String,
                    pub stdout: String,
                    pub stderr: String
                }
                #[derive(
                    Clone,
                    Debug,
                    marine_rs_sdk_test :: internal :: serde :: Serialize,
                    marine_rs_sdk_test :: internal :: serde :: Deserialize
                )]
                #[serde(crate = "marine_rs_sdk_test::internal::serde")]
                pub struct SecurityTetraplet {
                    pub peer_pk: String,
                    pub service_id: String,
                    pub function_name: String,
                    pub json_path: String
                }
                pub struct ModuleInterface {
                    marine:
                        std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>,
                    >,
                }
                impl ModuleInterface {
                    pub fn new(
                        marine: std::rc::Rc<
                            std::cell::RefCell<marine_rs_sdk_test::internal::AppService>,
                        >
                    ) -> Self {
                        Self { marine }
                    }
                }
                impl ModuleInterface {}
            }
            pub use __facade_override::CallParameters;
            pub use __facade_override::MountedBinaryResult;
            pub use __facade_override::MountedBinaryStringResult;
            pub use __facade_override::SecurityTetraplet;
            pub struct __GeneratedModules {
                pub greeting: modules::greeting::ModuleInterface,
            }
            impl __GeneratedModules {
                fn new(
                    marine: std::rc::Rc<
                        std::cell::RefCell<marine_rs_sdk_test::internal::AppService>,
                    >
                ) -> Self {
                    Self {
                        greeting: modules::greeting::ModuleInterface::new(marine.clone()),
                    }
                }
            }
            pub struct ServiceInterface {
                pub modules: __GeneratedModules,
                __facade: __facade_override::ModuleInterface,
                marine: std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >
            }
            impl ServiceInterface {
                pub fn new() -> Self {
                    let tmp_dir = std::env::temp_dir();
                    let service_id = marine_rs_sdk_test::internal::Uuid::new_v4().to_string();
                    let tmp_dir = tmp_dir.join(&service_id);
                    let tmp_dir = tmp_dir.to_string_lossy().to_string();
                    std::fs::create_dir(&tmp_dir)
                        .expect("can't create a directory for service in tmp");
                    let mut module_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                    let mut file_path = std::path::Path::new(file!()).components();
                    let mut truncated_file_path = Vec::new();
                    loop {
                        if module_path.ends_with(file_path.as_path()) {
                            break;
                        }
                        let (file_path_, remainder) =
                            match file_path.next_back().and_then(|p| match p {
                                std::path::Component::Normal(_)
                                | std::path::Component::CurDir
                                | std::path::Component::ParentDir => Some((file_path, p)),
                                _ => None,
                            }) {
                                Some(t) => t,
                                None => break,
                            };
                        file_path = file_path_;
                        truncated_file_path.push(remainder);
                    }
                    for path in truncated_file_path.iter().rev() {
                        module_path.push(path);
                    }
                    let _ = module_path.pop();
                    let config_path = module_path.join("empty_func/Config.toml");
                    let modules_dir = module_path.join("empty_func/artifacts");
                    let modules_dir = modules_dir
                        .to_str()
                        .expect("modules_dir contains invalid UTF8 string");
                    let mut __m_generated_marine_config =
                        marine_rs_sdk_test::internal::TomlAppServiceConfig::load(&config_path)
                            .unwrap_or_else(|e|
                                panic!(
                                    "app service config located at `{:?}` can't be loaded: {}",
                                    config_path, e
                                )
                            );
                    __m_generated_marine_config.service_base_dir = Some(tmp_dir);
                    __m_generated_marine_config.toml_faas_config.modules_dir =
                        Some(modules_dir.to_string());
                    let marine = marine_rs_sdk_test::internal::AppService::new_with_empty_facade(
                        __m_generated_marine_config,
                        service_id,
                        std::collections::HashMap::new()
                    )
                    .unwrap_or_else(|e| panic!("app service can't be created: {}", e));
                    let marine = std::rc::Rc::new(std::cell::RefCell::new(marine));
                    let modules = __GeneratedModules::new(marine.clone());
                    let __facade = __facade_override::ModuleInterface::new(marine.clone());
                    Self {
                        marine,
                        modules,
                        __facade
                    }
                }
            }
        }
    }
    fn test_func() {
        {}
    }
    test_func()
}
