-- Your SQL goes here


-- ----------------------------
-- Table structure for sys_dept
-- ----------------------------
CREATE TABLE sys_dept (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  create_by bigint(20) DEFAULT NULL ,-- COMMENT '创建人',
  create_at datetime DEFAULT NULL ,-- COMMENT '创建时间/注册时间',
  modify_by bigint(20) DEFAULT NULL ,-- COMMENT '最后更新人',
  modify_at datetime DEFAULT NULL ,-- COMMENT '最后更新时间',
  full_name varchar(255) DEFAULT NULL,
  num int(11) DEFAULT NULL,
  pid bigint(20) DEFAULT NULL,
  pids varchar(255) DEFAULT NULL,
  simple_name varchar(255) DEFAULT NULL,
  tips varchar(255) DEFAULT NULL,
  version int(11) DEFAULT NULL
);-- ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8 ,-- COMMENT='部门';

-- ----------------------------
-- Records of sys_dept
-- ----------------------------
INSERT INTO sys_dept VALUES ('24', null, null, null, null, '总公司', '1', '0', '[0],', '总公司', '', null);
INSERT INTO sys_dept VALUES ('25', null, null, null, null, '开发部', '2', '24', '[0],[24],', '开发部', '', null);
INSERT INTO sys_dept VALUES ('26', null, null, null, null, '运营部', '3', '24', '[0],[24],', '运营部', '', null);
INSERT INTO sys_dept VALUES ('27', null, null, null, null, '战略部', '4', '24', '[0],[24],', '战略部', '', null);
