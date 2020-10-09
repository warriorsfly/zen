CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- CREATE TABLE users (
--     id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
--     username TEXT NOT NULL,
--     email VARCHAR(254) NOT NULL,
--     UNIQUE (username, email),
--     password TEXT NOT NULL,
--     bio TEXT,
--     avatar TEXT,
--     created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
--     updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
-- );

-- 用户基本信息
CREATE TABLE users
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- 1.正常用户 2.禁言用户 3.虚拟用户 4.运营
    role INT NOT NULL DEFAULT 1 CHECK(user_role IN(1,2,3,4)),
    -- 注册来源：1.手机号 2.邮箱 3.微信 4.头条
    register_source INT NOT NULL DEFAULT 2 CHECK(register_source BETWEEN 1 AND 6),
    -- 昵称
    nick_name TEXT NOT NULL DEFAULT '',
    -- 性别
    gender INT NOT NULL DEFAULT 0 CHECK (gender BETWEEN 0 AND 2),
    -- 生日
    birthday TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    -- 签名
    signature TEXT NOT NULL DEFAULT '',
    -- 手机号，唯一
    mobile TEXT,
    mobile_bind_time TIMESTAMP WITH TIME ZONE,
    UNIQUE(mobile),
    -- 邮箱，唯一
    email TEXT,
    email_bind_time TIMESTAMP WITH TIME ZONE,
    UNIQUE(email),
    -- 头像 
    avatar TEXT NOT NULL DEFAULT '',
    -- 头像 200*200*80
    avatar200 TEXT NOT NULL DEFAULT '',
    -- 原始头像
    avatar_source TEXT NOT NULL DEFAULT '',

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');

-- 登陆账户
CREATE TABLE accounts
(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    uid UUID NOT NULL,
    -- 1.手机号 2.邮箱 3.微信 4.头条
    identity_type INT NOT NULL CHECK(identity_type IN (1,2,3,4)),
    -- 账户
    identifier TEXT NOT NULL,
    -- 密码
    certificate TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('accounts');

-- 用户扩展信息
CREATE TABLE profiles
(
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  vendor TEXT NOT NULL DEFAULT '',-- COMMENT '手机厂商：apple|htc|samsung，很少用',
  client_name TEXT NOT NULL DEFAULT '',-- COMMENT '客户端名称，如hjskang',
  client_version TEXT NOT NULL DEFAULT '',-- COMMENT '客户端版本号，如7.0.1',
  os_name TEXT NOT NULL DEFAULT '',-- COMMENT '设备号:android|ios',
  os_version TEXT NOT NULL DEFAULT '',-- COMMENT '系统版本号:2.2|2.3|4.0|5.1',
  device_name TEXT NOT NULL DEFAULT '',-- COMMENT '设备型号，如:iphone6s、u880、u8800',
  device_id TEXT NOT NULL DEFAULT '',-- COMMENT '设备ID',
  idfa TEXT NOT NULL DEFAULT '',-- COMMENT '苹果设备的IDFA',
  idfv TEXT NOT NULL DEFAULT '',-- COMMENT '苹果设备的IDFV',
  market TEXT NOT NULL DEFAULT '',-- COMMENT '来源',
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),-- COMMENT '添加时间',
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),-- COMMENT '更新时间',
  extend1 TEXT NOT NULL DEFAULT '',-- COMMENT '扩展字段1',
  extend2 TEXT NOT NULL DEFAULT '',-- COMMENT '扩展字段2',
  extend3 TEXT NOT NULL DEFAULT ''
  -- COMMENT '扩展字段3'
);

SELECT diesel_manage_updated_at('profiles');
