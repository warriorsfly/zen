-- 域名
CREATE TABLE domain
(
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  shop_id  VARCHAR(36) NOT NULL DEFAULT '',-- COMMENT '商铺id',
  host VARCHAR(64) NOT NULL DEFAULT '',-- COMMENT '服务器明成',
  ssl_enabled bool  NOT NULL DEFAULT false,---- COMMENT '是否启用ssl',
  url VARCHAR(512)  NOT NULL DEFAULT '',---- COMMENT 'url'
)