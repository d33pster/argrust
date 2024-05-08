use argrust;

#[test]
fn arguments() {
    let args = argrust::Arguments::new(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(args.get_arg_by_number(1), "a".to_string());
}