use serde_json::{json, Value};
use url::Url;

pub fn manifest_json_for_deploy(
    deployment_id: &str,
    service_id: &str,
    resolved_env: &Value,
) -> String {
    if let Some(m) = resolved_env.get("MUX_MANIFEST_JSON") {
        if m.is_object() {
            return serde_json::to_string(m).unwrap_or_else(|_| "{}".to_string());
        }
        if let Some(s) = m.as_str() {
            if let Ok(v) = serde_json::from_str::<Value>(s) {
                if v.is_object() {
                    return serde_json::to_string(&v).unwrap_or_else(|_| "{}".to_string());
                }
            }
        }
    }
    let Some(raw) = resolved_env
        .get("MUX_DEFAULT_UPSTREAM")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    else {
        return "{}".to_string();
    };
    let Ok(base) = Url::parse(raw) else {
        return "{}".to_string();
    };
    let Some(host) = base.host_str().map(str::to_string) else {
        return "{}".to_string();
    };
    let port_u = base.port().unwrap_or_else(|| {
        if base.scheme() == "https" || base.scheme() == "wss" {
            443
        } else {
            80
        }
    });
    let protocol = match base.scheme() {
        "wss" => "wss",
        "ws" => "ws",
        "https" => "https",
        _ => "http",
    };
    json!({
        "version": "1",
        "id": format!("{service_id}:{deployment_id}"),
        "listen": { "mode": "tcp", "host": "0.0.0.0", "port": 0 },
        "routes": [{
            "name": "mux-default-upstream",
            "match": { "type": "path_prefix", "prefix": "/" },
            "upstream": { "host": host, "port": port_u, "protocol": protocol }
        }]
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn empty_env_yields_empty_object_string() {
        let s = manifest_json_for_deploy("d1", "s1", &json!({}));
        assert_eq!(s, "{}");
    }

    #[test]
    fn default_upstream_builds_manifest() {
        let s = manifest_json_for_deploy(
            "dep",
            "svc",
            &json!({ "MUX_DEFAULT_UPSTREAM": "https://origin.example.com:8443/api" }),
        );
        let v: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["version"], "1");
        assert_eq!(v["routes"][0]["upstream"]["host"], "origin.example.com");
        assert_eq!(v["routes"][0]["upstream"]["port"], 8443);
        assert_eq!(v["routes"][0]["upstream"]["protocol"], "https");
    }

    #[test]
    fn mux_manifest_json_object_wins() {
        let custom = json!({
            "version": "1",
            "id": "x",
            "listen": { "mode": "tcp", "host": "0.0.0.0", "port": 1 },
            "routes": [{
                "name": "r",
                "match": { "type": "path_prefix", "prefix": "/z" },
                "upstream": { "host": "a.example", "port": 80, "protocol": "http" }
            }]
        });
        let s = manifest_json_for_deploy(
            "d",
            "s",
            &json!({
                "MUX_DEFAULT_UPSTREAM": "https://ignored.example",
                "MUX_MANIFEST_JSON": custom
            }),
        );
        let v: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["routes"][0]["match"]["prefix"], "/z");
    }
}
