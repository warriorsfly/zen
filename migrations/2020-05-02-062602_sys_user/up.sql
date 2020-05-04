-- Your SQL goes here
-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
CREATE TABLE sys_user (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  create_by VARCHAR(36) DEFAULT '' , -- COMMENT '创建人',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- COMMENT '创建时间/注册时间',
  updated_by VARCHAR(36)  , -- COMMENT '最后更新人',
  updated_at TIMESTAMP, -- COMMENT '最后更新时间',
  account varchar(255) ,
  avatar varchar(255) ,
  birthday TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deptid VARCHAR(36) ,
  email varchar(255) ,
  name varchar(255) ,
  password varchar(255) ,
  phone varchar(255) ,
  roleid varchar(255) ,
  salt varchar(255) ,
  gender int(11) ,
  status int(11) ,
  version int(11) 
);--  ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8 , -- COMMENT='账号';

-- ----------------------------
-- Records of sys_user
-- ----------------------------
-- INSERT INTO sys_user VALUES (1, null, null, null, null, 'system', null, null, null, null, '应用系统', null, null, null, null, null, null, null);
INSERT INTO sys_user VALUES (2, null, '1588602065', '1', '1588602065', 'admin', null, '1588602065', '27', '519056575@qq.com', '管理员', 'b5a51391f271f062867e5984e2fcffee', '15021222222', '1', '8pgby', '2', '1', '25');
INSERT INTO sys_user VALUES (3, null, '1588602065', '1', '1588602065', 'developer', null, '1588602065', '25', '519056575@qq.com', '网站管理员', 'fac36d5616fe9ebd460691264b28ee27', '15022222222', '2,', 'vscp9', '1', '1', null);