-- 登陆账户
CREATE TABLE user_auth
(
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    uid VARCHAR(36) NOT NULL,
    -- '1.手机号 2.邮箱(unuse) 3.微信 4.头条
    identity_type INT DEFAULT 1 NOT NULL CHECK(identity_type IN (1,2,3,4,5)),
    --
    identifier VARCHAR(50) NOT NULL DEFAULT '',
    certificate VARCHAR(64) NOT NULL DEFAULT '',
    login_session VARCHAR(256) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)