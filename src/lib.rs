#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate serde_json;
extern crate chrono;

#[allow(dead_code)]
mod http_cache_semantics {
    use std::collections::{HashMap, HashSet};
    use std::string::String;
    use serde_json::{json, Value};
    use chrono::prelude::*;

    lazy_static! {
        static ref STATUS_CODE_CACHEABLE_BY_DEFAULT: HashSet<i32> = {
            let mut set = HashSet::new();
            set.insert(200);
            set.insert(203);
            set.insert(204);
            set.insert(206);
            set.insert(300);
            set.insert(301);
            set.insert(404);
            set.insert(405);
            set.insert(410);
            set.insert(414);
            set.insert(501);

            return set;
        };
    }

    lazy_static! {
        static ref UNDERSTOOD_STATUSES: HashSet<i32> = {
            let mut set = HashSet::new();
            set.insert(200);
            set.insert(203);
            set.insert(204);
            set.insert(300);
            set.insert(301);
            set.insert(302);
            set.insert(303);
            set.insert(307);
            set.insert(308);
            set.insert(404);
            set.insert(405);
            set.insert(410);
            set.insert(414);
            set.insert(501);

            return set;
        };
    }

    lazy_static! {
        static ref HOP_BY_HOP_HEADERS: HashSet<&'static str> = {
            let mut set = HashSet::new();
            set.insert("date");
            set.insert("connection");
            set.insert("keep-alive");
            set.insert("proxy-authentication");
            set.insert("proxy-authorization");
            set.insert("te");
            set.insert("trailer");
            set.insert("transfer-encoding");
            set.insert("upgrade");

            return set;
        };
    }

    lazy_static! {
        static ref EXCLUDED_FROM_REVALIDATION_UPDATE: HashSet<&'static str> = {
            let mut set = HashSet::new(); 
            set.insert("content-length");
            set.insert("content-encoding");
            set.insert("transfer-encoding");
            set.insert("content-range");

            return set;
        };
    }

    fn parse_cache_control() -> () {
        unimplemented!();
    }

    fn format_cache_control() -> () {
        unimplemented!();
    }

    #[derive(Debug)]
    pub struct CachePolicy {
        request: Value,
        response: Value,

        // optionals
        shared: Option<bool>,
        ignore_cargo_cult: Option<bool>,
        trust_server_date: Option<bool>,
        cache_heuristic: Option<bool>,
        immutable_min_time_to_live: Option<i32>,

        // tbd
        // response_time: String,
        // status: i32,
        // response_cache_control: String,
        // method: String,
        // url: String,
        // host: String,
        // no_authorization: bool,
        // request_cache_control: String,
    }

    impl CachePolicy {
        pub fn new(request: Value, response: Value) -> Self {
            CachePolicy {
                request: request,
                response: response,
                shared: None,
                cache_heuristic: None,
                immutable_min_time_to_live: None,
                ignore_cargo_cult: None,
                trust_server_date: None,
            }
        }

        pub fn with_shared(mut self, value: bool) -> CachePolicy {
            self.shared = Some(value);
            self
        }

        pub fn with_ignored_cargo_cult(mut self, value: bool) -> CachePolicy {
            self.ignore_cargo_cult = Some(value);
            self
        }

        pub fn with_trust_server_date(mut self, value: bool) -> CachePolicy {
            self.trust_server_date = Some(value);
            self
        }

        pub fn now() -> String {
            return Utc::now().to_string();
        }

        pub fn is_storable(&self) -> bool {
            unimplemented!();
        }

        fn has_explicit_expiration(&self) {
            unimplemented!();
        }

        fn assert_request_has_headers(&self, request: String) {
            unimplemented!();
        }

        pub fn satisfies_without_revalidation(&self, request: Value) -> bool {
            unimplemented!();
        }

        fn request_matches(&self, request: String, allow_head_method: bool) {
            unimplemented!();
        }

        fn allows_storing_authenticated(&self) {
            unimplemented!();
        }

        fn vary_matches(&self, request: String) -> bool {
            unimplemented!();
        }

        fn copy_without_hop_by_hop_headers(&self, in_headers: HashMap<String, String>) -> HashMap<String, String> {
            unimplemented!();
        }

        pub fn response_headers(&self) -> Value {
            unimplemented!();
        }

        pub fn date(&self) {
            unimplemented!();
        }

        fn server_date(&self) {
            unimplemented!();
        }

        pub fn age(&self) -> i32 {
            unimplemented!();
        }

        fn age_value(&self) {
            unimplemented!();
        }
        
        pub fn max_age(&self) -> i32 {
            unimplemented!();
        }

        pub fn time_to_live(&self) -> i32 {
            unimplemented!();
        }

        pub fn is_stale(&self) -> bool {
            unimplemented!();
        }

        pub fn from_object(object: HashMap<String, String>) -> CachePolicy {
            unimplemented!();
        }

        pub fn to_object(&self) -> HashMap<String, String> {
            unimplemented!();
        }

        pub fn revalidation_headers(&self, incoming_request: String) -> HashMap<String, String> {
            unimplemented!();
        }

        pub fn revalidated_policy(&self, request: String, response: HashMap<String, String>) -> HashMap<String, String> {
            unimplemented!();
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use crate::http_cache_semantics::CachePolicy;
    use serde_json::json;
    use chrono::prelude::*;
    use std::string::String;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use super::*;
    
    fn assert_cached(should_put: bool, response_code: i32) {
        let expected_response_code = response_code;

        let mut response = json!({
            "headers": {
                "last-modified": format_date(-105, 1),
                "expires": format_date(1, 3600),
                "www.authenticate": "challenge"
            },
            "status": response_code,
            "body": "ABCDE",
        });

        if 407 == response {
            response["headers"]["proxy-authenticate"] = json!("Basic realm=\"protected area\"");
        } else if 401 == response_code {
            response["headers"]["www-authenticate"] = json!("Basic realm=\"protected area\"");
        } else if 204 == response_code || 205 == response_code {
            response = json!({"body": ""});
        }

        let mut request = json!({
            "url": "/",
            "headers": {}
        });

        let policy = CachePolicy::new(
            request,
            response
        ).with_shared(false);

        assert_eq!(should_put, policy.is_storable());
    }
    
    #[test]
    fn test_ok_http_response_caching_by_response_code() {
        assert_cached(false, 100);
        assert_cached(false, 101);
        assert_cached(false, 102);
        assert_cached(true, 200);
        assert_cached(false, 201);
        assert_cached(false, 202);
        assert_cached(true, 203);
        assert_cached(true, 204);
        assert_cached(false, 205);
        // 206: electing to not cache partial responses
        assert_cached(false, 206); 
        assert_cached(false, 207);
        assert_cached(true, 300);
        assert_cached(true, 301);
        assert_cached(true, 302);
        assert_cached(false, 303);
        assert_cached(false, 304);
        assert_cached(false, 305);
        assert_cached(false, 306);
        assert_cached(true, 307);
        assert_cached(true, 308);
        assert_cached(false, 400);
        assert_cached(false, 401);
        assert_cached(false, 402);
        assert_cached(false, 403);
        assert_cached(true, 404);
        assert_cached(true, 405);
        assert_cached(false, 406);
        assert_cached(false, 408);
        assert_cached(false, 409);
        // 410: the HTTP spec permits caching 410s, but the RI doesn't
        assert_cached(true, 410); 
        assert_cached(false, 411);
        assert_cached(false, 412);
        assert_cached(false, 413);
        assert_cached(true, 414);
        assert_cached(false, 415);
        assert_cached(false, 416);
        assert_cached(false, 417);
        assert_cached(false, 418);
        assert_cached(false, 429);
        assert_cached(false, 500);
        assert_cached(true, 501);
        assert_cached(false, 502);
        assert_cached(false, 503);
        assert_cached(false, 504);
        assert_cached(false, 505);
        assert_cached(false, 506);
    }
    
    #[test]
    fn test_default_expiration_date_fully_cached_for_less_than_24_hours() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "last-modified": format_date(-105, 1),
                    "date": format_date(-5, 1),
                },
                "body": "A"
            })
        ).with_shared(false);   

        assert!(policy.time_to_live() > 4000);
    }
    
    #[test]
    fn test_default_expiration_date_fully_cached_for_more_than_24_hours() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "last-modified": format_date(-105, 3600 * 24),
                    "date": format_date(-5, 3600 * 24),
                },
                "body": "A"
            })
        ).with_shared(false);

        assert!(policy.max_age() >= 10 * 3600 * 24);
        assert!(policy.time_to_live() + 1000 >= 5 * 3600 * 24);
    }
    
    #[test]
    fn test_max_age_in_the_past_with_date_header_but_no_last_modified_header() {
        // Chrome interprets max-age relative to the local clock. Both our cache
        // and Firefox both use the earlier of the local and server's clock.
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "date": format_date(-120, 1),
                    "cache-control": "max-age=60",
                }
            })
        ).with_shared(false);

        assert!(policy.is_stale());
    }
    
    #[test]
    fn test_max_age_preferred_over_lower_shared_max_age() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "date": format_date(-2, 60),
                    "cache-control": "s-maxage=60, max-age=180",
                }
            })
        ).with_shared(false);

        assert_eq!(policy.max_age(), 180);
    }
    
    #[test]
    fn test_max_age_preferred_over_higher_max_age() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "date": format_date(-3, 60),
                    "cache-control": "s-maxage=60, max-age=180",
                }
            })
        ).with_shared(false);

        assert!(policy.is_stale());
    }
    
    fn request_method_not_cached(method: String) {
        // 1. seed the cache (potentially)
        // 2. expect a cache hit or miss
        let policy = CachePolicy::new(
            json!({
                "method": method, 
                "headers": {}
            }),
            json!({
                "headers": {
                    "expires": format_date(1, 3600),
                }
            })
        ).with_shared(false);

        assert!(policy.is_stale());
    }
    
    #[test]
    fn test_request_method_options_is_not_cached() {
        request_method_not_cached("OPTIONS".to_string());
    }
    
    #[test]
    fn test_request_method_put_is_not_cached() {
        request_method_not_cached("PUT".to_string());
    }
    
    #[test]
    fn test_request_method_delete_is_not_cached() {
        request_method_not_cached("DELETE".to_string());
    }
    
    #[test]
    fn test_request_method_trace_is_not_cached() {
        request_method_not_cached("TRACE".to_string());
    }
    
    #[test]
    fn test_etag_and_expiration_date_in_the_future() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "etag": "v1",
                    "last-modified": format_date(-2, 3600),
                    "expires": format_date(1, 3600),
                }
            })
        ).with_shared(false);

        assert!(policy.time_to_live() > 0);
    }
    
    #[test]
    fn test_client_side_no_store() {
        let policy = CachePolicy::new(
            json!({
                "headers": {
                    "cache-control": "no-store",
                }
            }),
            json!({
                "headers": {
                    "cache-control": "max-age=60",
                }
            })
        ).with_shared(false);

        assert_eq!(policy.is_storable(), false);
    }
    
    #[test]
    fn test_request_max_age() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "last-modified": format_date(-2, 3600),
                    "date": format_date(-1, 60),
                    "expires": format_date(1, 3600),
                }
            })
        ).with_shared(false);

        assert_eq!(policy.is_stale(), false);
        assert!(policy.age() >= 60);

        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "max-age=90",
                },
            })), true
        );

        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "max-age=30",
                },
            })), false
        );
    }
    
    #[test]
    fn test_request_min_fresh() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "cache-control": "max-age=60",
                }
            })
        ).with_shared(false);

        assert_eq!(policy.is_stale(), false);

        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "min-fresh=10",
                },
            })), true
        );

        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "min-fresh=120",
                },
            })), false
        );
    }
    
    #[test]
    fn test_request_max_stale() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "cache-control": "max-age=120",
                    "date": format_date(-4, 60),
                }
            })
        ).with_shared(false);

        assert!(policy.is_stale());

        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "max-stale=180",
                },
            })), true
        );
    
        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "max-stale",
                },
            })), true
        );

        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "max-stale=10",
                },
            })), false
        );
    }
    
    #[test]
    fn test_request_max_stale_not_honored_with_must_revalidate() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "cache-control": "max-age=120, must-revalidate",
                    "date": format_date(-4, 60),
                }
            })
        ).with_shared(false);

        assert!(policy.is_stale());

        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "max-stale=180",
                },
            })), false
        );

        assert_eq!(
            policy.satisfies_without_revalidation(json!({
                "headers": {
                    "cache-control": "max-stale",
                },
            })), false
        );
    }
    
    #[test]
    fn test_get_headers_deletes_cached_100_level_warnings() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "headers": {
                    "warning": "199 test danger, 200 ok ok",
                }
            })
        );

        assert_eq!("200 ok ok", policy.response_headers()["warning"]);
    }
    
    #[test]
    fn test_do_not_cache_partial_response() {
        let policy = CachePolicy::new(
            json!({"headers": {}}),
            json!({
                "status": 206,
                "headers": {
                    "content-range": "bytes 100-100/200",
                    "cache-control": "max-age=60",
                }
            })
        );

        assert_eq!(policy.is_storable(), false);
    }
    
    fn format_date(delta: i64, unit: i64) -> String {
        let now: DateTime<Utc> = Utc::now();
        let result = now.timestamp_nanos() + delta * unit * 1000;

        return result.to_string();
    }
    
    #[test]
    fn test_no_store_kills_cache() {
        assert!(false);
    }
    
    #[test]
    fn test_post_not_cacheable_by_default() {
        assert!(false);
    }
    
    #[test]
    fn test_post_cacheable_explicitly() {
        assert!(false);
    }
    
    #[test]
    fn test_public_cacheable_auth_is_ok() {
        assert!(false);
    }
    
    #[test]
    fn test_proxy_cacheable_auth_is_ok() {
        assert!(false);
    }
    
    #[test]
    fn test_private_auth_is_ok() {
        assert!(false);
    }
    
    #[test]
    fn test_revalidate_auth_is_ok() {
        assert!(false);
    }
    
    #[test]
    fn test_auth_prevents_caching_by_default() {
        assert!(false);
    }
    
    #[test]
    fn test_simple_miss() {
        assert!(false);
    }
    
    #[test]
    fn test_simple_hit() {
        assert!(false);
    }
    
    #[test]
    fn test_weird_syntax() {
        assert!(false);
    }
    
    #[test]
    fn test_quoted_syntax() {
        assert!(false);
    }
    
    #[test]
    fn test_iis() {
        assert!(false);
    }
    
    #[test]
    fn test_pre_check_tolerated() {
        assert!(false);
    }
    
    #[test]
    fn test_pre_check_poison() {
        assert!(false);
    }
    
    #[test]
    fn test_pre_check_poison_undefined_header() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_with_expires() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_with_expires_always_relative_to_date() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_expires_no_date() {
        assert!(false);
    }
    
    #[test]
    fn test_ages() {
        assert!(false);
    }
    
    #[test]
    fn test_age_can_make_stale() {
        assert!(false);
    }
    
    #[test]
    fn test_age_not_always_stale() {
        assert!(false);
    }
    
    #[test]
    fn test_bogus_age_ignored() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_old_files() {
        assert!(false);
    }
    
    #[test]
    fn test_immutable_simple_hit() {
        assert!(false);
    }
    
    #[test]
    fn test_immutable_can_expire() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_immutable_files() {
        assert!(false);
    }
    
    #[test]
    fn test_immutable_can_be_off() {
        assert!(false);
    }
    
    #[test]
    fn test_pragma_no_cache() {
        assert!(false);
    }
    
    #[test]
    fn test_blank_cache_control_and_pragma_no_cache() {
        assert!(false);
    }
    
    #[test]
    fn test_no_store() {
        assert!(false);
    }
    
    #[test]
    fn test_observe_private_cache() {
        assert!(false);
    }
    
    #[test]
    fn test_do_not_share_cookies() {
        assert!(false);
    }
    
    #[test]
    fn test_do_share_cookies_if_immutable() {
        assert!(false);
    }
    
    #[test]
    fn test_cache_explicitly_public_cookie() {
        assert!(false);
    }
    
    #[test]
    fn test_miss_max_age_equals_zero() {
        assert!(false);
    }
    
    #[test]
    fn test_uncacheable_503() {
        assert!(false);
    }
    
    #[test]
    fn test_cacheable_301() {
        assert!(false);
    }
    
    #[test]
    fn test_uncacheable_303() {
        assert!(false);
    }
    
    #[test]
    fn test_cacheable_303() {
        assert!(false);
    }
    
    #[test]
    fn test_uncacheable_412() {
        assert!(false);
    }
    
    #[test]
    fn test_expired_expires_cache_with_max_age() {
        assert!(false);
    }
    
    #[test]
    fn test_expired_expires_cached_with_s_maxage() {
        assert!(false);
    }
    
    #[test]
    fn test_max_age_wins_over_future_expires() {
        assert!(false);
    }
    
    #[test]
    fn test_remove_hop_headers() {
        assert!(false);
    }    
    
    fn assert_headers_passed() {
        assert!(false);
    }
    
    fn assert_no_validators() {
        assert!(false);
    }
    
    #[test]
    fn test_ok_if_method_changes_to_head() {
        assert!(false);
    }
    
    #[test]
    fn test_not_if_method_mismatch_other_than_head() {
        assert!(false);
    }
    
    #[test]
    fn test_not_if_url_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_not_if_host_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_not_if_vary_fields_prevent() {
        assert!(false);
    }
    
    #[test]
    fn test_when_entity_tag_validator_is_present() {
        assert!(false);
    }
    
    #[test]
    fn test_skips_weak_validators_on_post_2() {
        assert!(false);
    }
    
    #[test]
    fn test_merges_validators() {
        assert!(false);
    }
    
    #[test]
    fn test_when_last_modified_validator_is_present() {
        assert!(false);
    }
    
    #[test]
    fn test_not_without_validators() {
        assert!(false);
    }
    
    #[test]
    fn test_113_added() {
        assert!(false);
    }
    
    #[test]
    fn test_removes_warnings() {
        assert!(false);
    }
    
    #[test]
    fn test_must_contain_any_etag() {
        assert!(false);
    }
    
    #[test]
    fn test_merges_etags() {
        assert!(false);
    }
    
    #[test]
    fn test_should_send_the_last_modified_value() {
        assert!(false);
    }
    
    #[test]
    fn test_should_not_send_the_last_modified_value_for_post() {
        assert!(false);
    }
    
    #[test]
    fn test_should_not_send_the_last_modified_value_for_range_request() {
        assert!(false);
    }
    
    #[test]
    fn test_when_urls_match() {
        assert!(false);
    }
    
    #[test]
    fn test_when_expires_is_present() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_urls_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_when_methods_match() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_hosts_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_when_methods_match_head() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_methods_mismatch() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_methods_mismatch_head() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_proxy_revalidating() {
        assert!(false);
    }
    
    #[test]
    fn test_when_not_a_proxy_revalidating() {
        assert!(false);
    }
    
    #[test]
    fn test_not_when_no_cache_requesting() {
        assert!(false);
    }
    
    fn not_modified_response_headers() {
        assert!(false);
    }
    
    fn assert_updates() {
        assert!(false);
    }
    
    #[test]
    fn test_matching_etags_are_updated() {
        assert!(false);
    }
    
    #[test]
    fn test_matching_weak_etags_are_updated() {
        assert!(false);
    }
    
    #[test]
    fn test_matching_last_mod_are_updated() {
        assert!(false);
    }
    
    #[test]
    fn test_both_matching_are_updated() {
        assert!(false);
    }
    
    #[test]
    fn test_check_status() {
        assert!(false);
    }
    
    #[test]
    fn test_last_mod_ignored_if_etag_is_wrong() {
        assert!(false);
    }
    
    #[test]
    fn test_ignored_if_validator_is_missing() {
        assert!(false);
    }
    
    #[test]
    fn test_skips_update_of_content_length() {
        assert!(false);
    }
    
    #[test]
    fn test_ignored_if_validator_is_different() {
        assert!(false);
    }
    
    #[test]
    fn test_ignored_if_validator_does_not_match() {
        assert!(false);
    }
    
    #[test]
    fn test_vary_basic() {
        assert!(false);
    }
    
    #[test]
    fn test_asterisks_does_not_match() {
        assert!(false);
    }
    
    #[test]
    fn test_asterisks_is_stale() {
        assert!(false);
    }
    
    #[test]
    fn test_values_are_case_sensitive() {
        assert!(false);
    }
    
    #[test]
    fn test_irrelevant_headers_ignored() {
        assert!(false);
    }
    
    #[test]
    fn test_absence_is_meaningful() {
        assert!(false);
    }
    
    #[test]
    fn test_all_values_must_match() {
        assert!(false);
    }
    
    #[test]
    fn test_whitespace_is_okay() {
        assert!(false);
    }
    
    #[test]
    fn test_order_is_irrelevant() {
        assert!(false);
    }
    
    #[test]
    fn test_thaw_wrong_object() {
        assert!(false);
    }
    
    #[test]
    fn test_missing_headers() {
        assert!(false);
    }
    
    #[test]
    fn test_github_response_with_small_clock_skew() {
        assert!(false);
    }
    
}    
