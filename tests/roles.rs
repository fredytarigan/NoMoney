// pub mod setup;

// use reqwest::StatusCode;
// use serde_json::Value;
// use setup::{setup_client, APP_HOST};

// #[test]
// fn test_get_roles() {
//     /*
//        Setup Section
//     */
//     let client = setup_client();

//     /*
//        Test Section
//     */
//     // response test
//     let response = client.get(format!("{}/roles", APP_HOST)).send().unwrap();

//     // response should be 200
//     assert_eq!(response.status(), StatusCode::OK);

//     // data test
//     let roles: Value = response.json().unwrap();

//     // total row must be 3
//     assert_eq!(roles["data"].as_array().unwrap().len(), 3);
// }

// #[test]
// fn test_get_admin_role() {
//     /*
//        Setup Section
//     */
//     let client = setup_client();

//     /*
//        Test Section
//     */
//     // response test
//     let response = client
//         .get(format!("{}/roles/search?name=admin", APP_HOST))
//         .send()
//         .unwrap();

//     // response should be 200
//     assert_eq!(response.status(), StatusCode::OK);

//     // data test
//     let roles: Value = response.json().unwrap();

//     // left side must matched right side
//     assert_eq!(
//         roles["data"].as_object().unwrap().get("name").unwrap(),
//         "admin"
//     );
//     assert_eq!(
//         roles["data"]
//             .as_object()
//             .unwrap()
//             .get("description")
//             .unwrap(),
//         "Administrator Role"
//     )
// }

// #[test]
// fn test_get_editor_role() {
//     /*
//        Setup Section
//     */
//     let client = setup_client();

//     /*
//        Test Section
//     */
//     // response test
//     let response = client
//         .get(format!("{}/roles/search?name=editor", APP_HOST))
//         .send()
//         .unwrap();

//     // response should be 200
//     assert_eq!(response.status(), StatusCode::OK);

//     // data test
//     let roles: Value = response.json().unwrap();

//     // left side must matched right side
//     assert_eq!(
//         roles["data"].as_object().unwrap().get("name").unwrap(),
//         "editor"
//     );
//     assert_eq!(
//         roles["data"]
//             .as_object()
//             .unwrap()
//             .get("description")
//             .unwrap(),
//         "Editor Role"
//     )
// }

// #[test]
// fn test_get_viewer_role() {
//     /*
//        Setup Section
//     */
//     let client = setup_client();

//     /*
//        Test Section
//     */
//     // response test
//     let response = client
//         .get(format!("{}/roles/search?name=viewer", APP_HOST))
//         .send()
//         .unwrap();

//     // response should be 200
//     assert_eq!(response.status(), StatusCode::OK);

//     // data test
//     let roles: Value = response.json().unwrap();

//     // left side must matched right side
//     assert_eq!(
//         roles["data"].as_object().unwrap().get("name").unwrap(),
//         "viewer"
//     );
//     assert_eq!(
//         roles["data"]
//             .as_object()
//             .unwrap()
//             .get("description")
//             .unwrap(),
//         "Viewer Role"
//     )
// }
