-- Your SQL goes here
-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
CREATE TABLE sys_user (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  create_by bigint(20) DEFAULT NULL , -- COMMENT '创建人',
  create_time datetime DEFAULT NULL , -- COMMENT '创建时间/注册时间',
  modify_by bigint(20) DEFAULT NULL , -- COMMENT '最后更新人',
  modify_at datetime DEFAULT NULL , -- COMMENT '最后更新时间',
  account varchar(255) DEFAULT NULL,
  avatar varchar(255) DEFAULT NULL,
  birthday datetime DEFAULT NULL,
  deptid VARCHAR(36) DEFAULT NULL,
  email varchar(255) DEFAULT NULL,
  name varchar(255) DEFAULT NULL,
  password varchar(255) DEFAULT NULL,
  phone varchar(255) DEFAULT NULL,
  roleid varchar(255) DEFAULT NULL,
  salt varchar(255) DEFAULT NULL,
  sex int(11) DEFAULT NULL,
  status int(11) DEFAULT NULL,
  version int(11) DEFAULT NULL
);--  ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8 , -- COMMENT='账号';

-- ----------------------------
-- Records of sys_user
-- ----------------------------
INSERT INTO sys_user VALUES (1, null, null, null, null, 'system', null, null, null, null, '应用系统', null, null, null, null, null, null, null);
INSERT INTO sys_user VALUES (2, null, '2016-01-29 08:49:53', '1', '2019-03-20 23:45:24', 'admin', null, '2017-05-05 00:00:00', '27', '519056575@qq.com', '管理员', 'b5a51391f271f062867e5984e2fcffee', '15021222222', '1', '8pgby', '2', '1', '25');
INSERT INTO sys_user VALUES (3, null, '2018-09-13 17:21:02', '1', '2019-01-09 23:05:51', 'developer', null, '2017-12-31 00:00:00', '25', '519056575@qq.com', '网站管理员', 'fac36d5616fe9ebd460691264b28ee27', '15022222222', '2,', 'vscp9', '1', '1', null);