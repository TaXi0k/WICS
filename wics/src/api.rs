use ureq;

pub enum QueryResult {
    ServerOk,
    ServerBad,
    Unknown
}

pub enum QueryError {
    Ureq(ureq::Error),
    Json(json::Error),
    ServerSideEntryNotFound,
}
impl From<ureq::Error> for QueryError {
    fn from(err: ureq::Error) -> Self {
        QueryError::Ureq(err)
    }
}
impl From<json::Error> for QueryError {
    fn from(err: json::Error) -> Self {
        QueryError::Json(err)
    }
}

pub fn query_modrinth_api(modid: &str) -> Result<QueryResult, QueryError> {       // the Ok(bool) is true when it's ok for server, and false if it's not ok for server :3
    let request_url = format!("https://api.modrinth.com/v2/project/{}", modid);

    let response: String = ureq::get(request_url)
        .header("User-Agent", format!("TaXi0k/WICS/{} (github.com/TaXi0k/WICS)", env!("CARGO_PKG_VERSION")))
        .call()?    // Call the api
        .body_mut()      // I have no clue wtf is that but it's needed
        .read_to_string()?;       // Convert response to string

    let parsed_response = json::parse(&response)?;

    let server_side = parsed_response["server_side"].as_str();

    if let Some(server_side) = server_side {
        if server_side == "unsupported" {   // If api returns "unsupported" then mod is not ok on server
            Ok(QueryResult::ServerBad)
        }
        else if server_side == "unknown" {  // If api returns "unknown", so we'll list it as unknown instead of OK nor BAD mods
            Ok(QueryResult::Unknown)
        }
        else {  // Everything else so: "required" and "optional" => it's ok on server
            Ok(QueryResult::ServerOk)
        }
    }
    else {
        Err(QueryError::ServerSideEntryNotFound)
    }
}