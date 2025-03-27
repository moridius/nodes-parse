use nodes_parse::NodesJSON;

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
