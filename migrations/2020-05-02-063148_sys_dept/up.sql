-- Your SQL goes here


-- ----------------------------
-- Table structure for sys_dept
-- ----------------------------
CREATE TABLE sys_dept (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  create_by VARCHAR(36),-- COMMENT '创建人',
  create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '创建时间/注册时间',
  modify_by VARCHAR(36),-- COMMENT '最后更新人',
  modify_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '最后更新时间',
  full_name varchar(255) ,
  num int(11) ,
  pid bigint(20) ,
  pids varchar(255) ,
  simple_name varchar(255) ,
  tips varchar(255) ,
  version int(11) 
);-- ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8 ,-- COMMENT='部门';

-- ----------------------------
-- Records of sys_dept
-- ----------------------------
INSERT INTO sys_dept VALUES ('24', null, 1588602065, null, 1588602065, '总公司', '1', '0', '[0],', '总公司', '', null);
INSERT INTO sys_dept VALUES ('25', null, 1588602065, null, 1588602065, '开发部', '2', '24', '[0],[24],', '开发部', '', null);
INSERT INTO sys_dept VALUES ('26', null, 1588602065, null, 1588602065, '运营部', '3', '24', '[0],[24],', '运营部', '', null);
INSERT INTO sys_dept VALUES ('27', null, 1588602065, null, 1588602065, '战略部', '4', '24', '[0],[24],', '战略部', '', null);
