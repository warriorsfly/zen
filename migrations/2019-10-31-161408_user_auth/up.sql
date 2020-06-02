-- 登陆账户
CREATE TABLE user_auth
(
    id SERIAL PRIMARY KEY,
    uid INT NOT NULL,
    -- '1.手机号 2.邮箱(unuse) 3.微信 4.头条
    identity_type INT DEFAULT 1 NOT NULL CHECK(identity_type IN (1,2,3,4,5)),
    --
    identifier TEXT NOT NULL DEFAULT '',
    certificate TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)