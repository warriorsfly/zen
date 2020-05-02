-- Your SQL goes here

-- ----------------------------
-- Table structure for sys_express
-- ----------------------------
DROP TABLE IF EXISTS sys_express;
CREATE TABLE sys_express (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  create_by bigint(20) DEFAULT NULL ,-- COMMENT '创建人',
  create_time datetime DEFAULT NULL ,-- COMMENT '创建时间/注册时间',
  modify_by bigint(20) DEFAULT NULL ,-- COMMENT '最后更新人',
  modify_at datetime DEFAULT NULL ,-- COMMENT '最后更新时间',
  code varchar(16) DEFAULT NULL ,-- COMMENT '公司编码',
  disabled tinyint(4) DEFAULT NULL ,-- COMMENT '是否禁用',
  name varchar(32) DEFAULT NULL ,-- COMMENT '公司名称',
  sort int(11) DEFAULT NULL -- COMMENT '排序'
);--  ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8 ,-- COMMENT='物流公司';

-- ----------------------------
-- Records of sys_express
-- ----------------------------
INSERT INTO sys_express VALUES ('1', null, null, null, null, 'SF', '0', '顺丰快递', '1');
INSERT INTO sys_express VALUES ('2', null, null, null, null, 'STO', '0', '申通快递', '2');
INSERT INTO sys_express VALUES ('3', null, null, null, null, 'YTO', '0', '圆通快递', '3');
INSERT INTO sys_express VALUES ('4', null, null, null, null, 'ZTO', '0', '中通快递', '4');
INSERT INTO sys_express VALUES ('5', null, null, null, null, 'YUNDA', '0', '韵达快递', '5');
INSERT INTO sys_express VALUES ('6', null, null, null, null, 'RUFENG', '1', '如风达快递', '6');