use ic_kit::candid::candid_method;

#[ic_cdk_macros::query]
#[candid_method(query)]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk_macros::query]
#[candid_method(query)]
fn version() -> &'static str {
    "0.0.5"
}

// ---------------- CANDID -----------------------

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    include_str!("hello.did").to_string()
}