-- 登陆账户
CREATE TABLE user_auth
(
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    uid VARCHAR(36) NOT NULL,
    -- '1手机号 2邮箱 3用户名 4qq 5微信 6腾讯微博 7新浪微博'
    identity_type INT DEFAULT 1 NOT NULL CHECK(identity_type IN (1,2,3,4,5,6)),
    --
    identifier VARCHAR(50) NOT NULL DEFAULT '',
    certificate VARCHAR(64) NOT NULL DEFAULT '',
    login_session VARCHAR(256) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)