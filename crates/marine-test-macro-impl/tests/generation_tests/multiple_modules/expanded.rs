#[test]
fn empty_string() {
    pub mod marine_test_env {
        pub mod greeting {
            pub use records::*;
            pub mod records {
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
                        std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >,
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
                impl ModuleInterface {
                    pub fn greeting(&mut self, name: String) -> String {
                        use std::ops::DerefMut;
                        let arguments = marine_rs_sdk_test::internal::serde_json::json!([name]);
                        let result = self
                            .marine
                            .as_ref()
                            .borrow_mut()
                            .call_module("greeting", "greeting", arguments, <_>::default())
                            .expect("call to Marine failed");
                        let result: String =
                            marine_rs_sdk_test::internal::serde_json::from_value(result)
                                .expect("the default deserializer shouldn't fail");
                        result
                    }
                    pub fn greeting_cp(
                        &mut self,
                        name: String,
                        cp: marine_rs_sdk_test::CallParameters
                    ) -> String {
                        use std::ops::DerefMut;
                        let arguments = marine_rs_sdk_test::internal::serde_json::json!([name]);
                        let result = self
                            .marine
                            .as_ref()
                            .borrow_mut()
                            .call_module("greeting", "greeting", arguments, cp)
                            .expect("call to Marine failed");
                        let result: String =
                            marine_rs_sdk_test::internal::serde_json::from_value(result)
                                .expect("the default deserializer shouldn't fail");
                        result
                    }
                }
            }
        }
        pub mod call_parameters {
            pub use records::*;
            pub mod records {
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
                pub struct SecurityTetraplet {
                    pub peer_pk: String,
                    pub service_id: String,
                    pub function_name: String,
                    pub json_path: String
                }
                pub struct ModuleInterface {
                    marine:
                        std::rc::Rc<std::cell::RefCell<marine_rs_sdk_test::internal::AppService>, >,
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
                impl ModuleInterface {
                    pub fn call_parameters(&mut self, ) -> String {
                        use std::ops::DerefMut;
                        let arguments = marine_rs_sdk_test::internal::serde_json::json!([]);
                        let result = self
                            .marine
                            .as_ref()
                            .borrow_mut()
                            .call_module(
                                "call_parameters",
                                "call_parameters",
                                arguments,
                                <_>::default()
                            )
                            .expect("call to Marine failed");
                        let result: String =
                            marine_rs_sdk_test::internal::serde_json::from_value(result)
                                .expect("the default deserializer shouldn't fail");
                        result
                    }
                    pub fn call_parameters_cp(
                        &mut self,
                        cp: marine_rs_sdk_test::CallParameters
                    ) -> String {
                        use std::ops::DerefMut;
                        let arguments = marine_rs_sdk_test::internal::serde_json::json!([]);
                        let result = self
                            .marine
                            .as_ref()
                            .borrow_mut()
                            .call_module("call_parameters", "call_parameters", arguments, cp)
                            .expect("call to Marine failed");
                        let result: String =
                            marine_rs_sdk_test::internal::serde_json::from_value(result)
                                .expect("the default deserializer shouldn't fail");
                        result
                    }
                    pub fn return_string(&mut self,) -> String {
                        use std::ops::DerefMut;
                        let arguments = marine_rs_sdk_test::internal::serde_json::json!([]);
                        let result = self
                            .marine
                            .as_ref()
                            .borrow_mut()
                            .call_module(
                                "call_parameters",
                                "return_string",
                                arguments,
                                <_>::default()
                            )
                            .expect("call to Marine failed");
                        let result: String =
                            marine_rs_sdk_test::internal::serde_json::from_value(result)
                                .expect("the default deserializer shouldn't fail");
                        result
                    }
                    pub fn return_string_cp(
                        &mut self,
                        cp: marine_rs_sdk_test::CallParameters
                    ) -> String {
                        use std::ops::DerefMut;
                        let arguments = marine_rs_sdk_test::internal::serde_json::json!([]);
                        let result = self
                            .marine
                            .as_ref()
                            .borrow_mut()
                            .call_module("call_parameters", "return_string", arguments, cp)
                            .expect("call to Marine failed");
                        let result: String =
                            marine_rs_sdk_test::internal::serde_json::from_value(result)
                                .expect("the default deserializer shouldn't fail");
                        result
                    }
                    pub fn test_array_refs(&mut self,) -> Vec<String> {
                        use std::ops::DerefMut;
                        let arguments = marine_rs_sdk_test::internal::serde_json::json!([]);
                        let result = self
                            .marine
                            .as_ref()
                            .borrow_mut()
                            .call_module(
                                "call_parameters",
                                "test_array_refs",
                                arguments,
                                <_>::default()
                            )
                            .expect("call to Marine failed");
                        let result: Vec<String> =
                            marine_rs_sdk_test::internal::serde_json::from_value(result)
                                .expect("the default deserializer shouldn't fail");
                        result
                    }
                    pub fn test_array_refs_cp(
                        &mut self,
                        cp: marine_rs_sdk_test::CallParameters
                    ) -> Vec<String> {
                        use std::ops::DerefMut;
                        let arguments = marine_rs_sdk_test::internal::serde_json::json!([]);
                        let result = self
                            .marine
                            .as_ref()
                            .borrow_mut()
                            .call_module("call_parameters", "test_array_refs", arguments, cp)
                            .expect("call to Marine failed");
                        let result: Vec<String> =
                            marine_rs_sdk_test::internal::serde_json::from_value(result)
                                .expect("the default deserializer shouldn't fail");
                        result
                    }
                }
            }
        }
    }
    let tmp_dir = std::env::temp_dir();
    let service_id = marine_rs_sdk_test::internal::Uuid::new_v4().to_string();
    let tmp_dir = tmp_dir.join(&service_id);
    let tmp_dir = tmp_dir.to_string_lossy().to_string();
    std::fs::create_dir(&tmp_dir).expect("can't create a directory for service in tmp");
    let mut module_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut file_path = std::path::Path::new(file!()).components();
    let mut truncated_file_path = Vec::new();
    loop {
        if module_path.ends_with(file_path.as_path()) {
            break;
        }
        let (file_path_, remainder) = match file_path.next_back().and_then(|p| match p {
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
    let config_path = module_path.join("Config.toml");
    let modules_dir = module_path.join("artifacts");
    let modules_dir = modules_dir
        .to_str()
        .expect("modules_dir contains invalid UTF8 string");
    let mut __m_generated_marine_config = marine_rs_sdk_test::internal::TomlAppServiceConfig::load(
        &config_path
    )
    .unwrap_or_else(|e|
        panic!(
            "app service config located at `{:?}` can't be loaded: {}",
            config_path, e
        )
    );
    __m_generated_marine_config.service_base_dir = Some(tmp_dir);
    __m_generated_marine_config.toml_faas_config.modules_dir = Some(modules_dir.to_string());
    let marine = marine_rs_sdk_test::internal::AppService::new_with_empty_facade(
        __m_generated_marine_config,
        service_id,
        std::collections::HashMap::new()
    )
    .unwrap_or_else(|e| panic!("app service can't be created: {}", e));
    let marine = std::rc::Rc::new(std::cell::RefCell::new(marine));
    let mut greeting_m = marine_test_env::greeting::ModuleInterface::new(marine.clone());
    let mut call_parameters_m =
        marine_test_env::call_parameters::ModuleInterface::new(marine.clone());
    fn test_func(
        greeting_m: marine_test_env::greeting::ModuleInterface,
        call_parameters_m: marine_test_env::call_parameters::ModuleInterface
    ) {
        let mut greeting_m = greeting_m;
        let mut call_parameters_m = call_parameters_m;
        {
            let init_peer_id = "init_peer_id";
            let service_id = "service_id";
            let service_creator_peer_id = "service_creator_peer_id";
            let host_id = "host_id";
            let particle_id = "particle_id";
            let greeting = greeting_m.greeting("asd");
            let mut tetraplet = SecurityTetraplet::default();
            tetraplet.function_name = "some_func_name".to_string();
            tetraplet.json_path = "some_json_path".to_string();
            let tetraplets = vec![vec![tetraplet]];
            let cp = CallParameters {
                init_peer_id: init_peer_id.to_string(),
                service_id: service_id.to_string(),
                service_creator_peer_id: service_creator_peer_id.to_string(),
                host_id: host_id.to_string(),
                particle_id: particle_id.to_string(),
                tetraplets: tetraplets.clone(),
            };
            let actual = call_parameters_m.call_parameters_cp(cp);
            let expected = format!(
                "{}\n{}\n{}\n{}\n{}\n{:?}",
                init_peer_id, service_id, service_creator_peer_id, host_id, particle_id, tetraplets
            );
            assert_eq!(actual, expected);
        }
    }
    test_func(greeting_m, call_parameters_m, )
}
