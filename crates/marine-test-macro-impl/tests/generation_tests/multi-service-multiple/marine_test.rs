fn test() {
    let mut greeting = marine_test_env::greeting::ServiceInterface::new();
    let _ = greeting.download("duckduckgo.com");
}
