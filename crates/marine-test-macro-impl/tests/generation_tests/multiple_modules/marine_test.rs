fn empty_string(greeting_m: marine_test_env::greeting::ModuleInterface, call_parameters_m: marine_test_env::call_parameters::ModuleInterface) {
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
