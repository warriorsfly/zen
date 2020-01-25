-- 用户扩展信息
CREATE TABLE user_extra (
  uid UUID NOT NULL PRIMARY KEY,--'用户 ID',
  vendor VARCHAR(64) NOT NULL DEFAULT '',-- COMMENT '手机厂商：apple|htc|samsung，很少用',
  client_name VARCHAR(50) NOT NULL DEFAULT '',-- COMMENT '客户端名称，如hjskang',
  client_version VARCHAR(50) NOT NULL DEFAULT '',-- COMMENT '客户端版本号，如7.0.1',
  os_name VARCHAR(16) NOT NULL DEFAULT '',-- COMMENT '设备号:android|ios',
  os_version VARCHAR(16) NOT NULL DEFAULT '',-- COMMENT '系统版本号:2.2|2.3|4.0|5.1',
  device_name VARCHAR(32) NOT NULL DEFAULT '',-- COMMENT '设备型号，如:iphone6s、u880、u8800',
  device_id VARCHAR(128) NOT NULL DEFAULT '',-- COMMENT '设备ID',
  idfa VARCHAR(50) NOT NULL DEFAULT '',-- COMMENT '苹果设备的IDFA',
  idfv VARCHAR(50) NOT NULL DEFAULT '',-- COMMENT '苹果设备的IDFV',
  market VARCHAR(20) NOT NULL DEFAULT '',-- COMMENT '来源',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,-- COMMENT '添加时间',
  updated_at TIMESTAMP ,-- COMMENT '更新时间',
  extend1 VARCHAR(100) NOT NULL DEFAULT '',-- COMMENT '扩展字段1',
  extend2 VARCHAR(100) NOT NULL DEFAULT '',-- COMMENT '扩展字段2',
  extend3 VARCHAR(100) NOT NULL DEFAULT '' -- COMMENT '扩展字段3'
)