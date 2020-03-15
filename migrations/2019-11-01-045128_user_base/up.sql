-- 用户基本信息
CREATE TABLE user_base
(
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    -- 2.正常用户 3.禁言用户 4.虚拟用户 5.运营
    user_role INT NOT NULL DEFAULT 2 CHECK(user_role IN(2,3,4,5)),
    -- 注册来源：1.手机号 2.邮箱 3.微信 4.头条
    register_source INT NOT NULL DEFAULT 0 CHECK(register_source BETWEEN 1 AND 6),
    -- 昵称
    nick_name varchar(32) NOT NULL DEFAULT '',
    gender INT NOT NULL DEFAULT 0 CHECK (gender BETWEEN 0 AND 2),
    birthday TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    signature VARCHAR(255) NOT NULL DEFAULT '',
    -- 手机号，唯一
    mobile VARCHAR(16) NOT NULL DEFAULT '',
    mobile_bind_time TIMESTAMP,
    -- 邮箱，唯一
    email VARCHAR(16) NOT NULL DEFAULT '',
    email_bind_time TIMESTAMP,
    -- 头像 
    avatar VARCHAR(255) NOT NULL DEFAULT '',
    -- 头像 200*200*80
    avatar200 VARCHAR(255) NOT NULL DEFAULT '',
    -- 原始头像
    avatar_source VARCHAR(255) NOT NULL DEFAULT '',

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    push_token VARCHAR(50) NOT NULL DEFAULT ''
)