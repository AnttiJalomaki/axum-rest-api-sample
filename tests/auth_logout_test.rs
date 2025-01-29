use reqwest::StatusCode;
use serial_test::serial;

pub mod common;
use common::{
    auth,
    constants::{TEST_ADMIN_PASSWORD_HASH, TEST_ADMIN_USERNAME},
    route, utils,
};

#[tokio::test]
#[serial]
async fn logout_test() {
    // Start the api server.
    utils::run_app().await;

    let config = utils::config();

    // Assert that revoked options are enabled.
    assert!(config.jwt_enable_revoked_tokens);

    // Try unauthorized access to the root handler.
    assert_eq!(
        route::fetch_root("").await.unwrap(),
        StatusCode::UNAUTHORIZED
    );

    // Login as an admin.
    let (status, result) = auth::login(TEST_ADMIN_USERNAME, TEST_ADMIN_PASSWORD_HASH)
        .await
        .unwrap();
    assert_eq!(status, StatusCode::OK);
    let (access_token, refresh_token) = result.unwrap();

    // Access to the root handler.
    assert_eq!(
        route::fetch_root(&access_token).await.unwrap(),
        StatusCode::OK
    );

    // Logout.
    assert_eq!(auth::logout(&refresh_token).await.unwrap(), StatusCode::OK);

    // Try access to the root handler after logout.
    assert_eq!(
        route::fetch_root(&access_token).await.unwrap(),
        StatusCode::UNAUTHORIZED
    );
}
