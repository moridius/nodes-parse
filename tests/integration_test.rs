use nodes_parse::NodesJSON;
use std::path::PathBuf;

#[test]
fn test_arbitrary_static_v2() {
    use std::env;
    let mut path_buf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path_buf.push("resources/tests/nodes.json");
    let data = std::fs::read_to_string(path_buf).unwrap();
    let nodes_json = serde_json::from_str::<NodesJSON>(&data).unwrap();
    assert! {nodes_json.version==2}
}

#[test]
fn test_parse_current_v2() {
    let url = "https://harvester.ffh.zone/api/nodes.json";
    let response: String = ureq::get(url)
        .call().unwrap()
        .body_mut()
        .read_to_string().unwrap();
    let nodes_json: NodesJSON = serde_json::from_str(&response).unwrap();
    assert!{nodes_json.version==2}
}
