
-- ----------------------------
-- Table structure for shop_user
-- ----------------------------
DROP TABLE IF EXISTS shop_user;
CREATE TABLE shop_user (

  id VARCHAR(36) NOT NULL PRIMARY KEY,
  avatar varchar(64)  ,-- COMMENT '头像',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '注册时间',
  gender varchar(18)  ,-- COMMENT '性别:male;female',
  last_login_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '最后登陆时间',
  mobile varchar(16)  ,-- COMMENT '手机号',
  nick_name varchar(32)  ,-- COMMENT '昵称',
  password varchar(32)  ,-- COMMENT '密码',
  salt varchar(32) -- COMMENT '密码盐',
);-- ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8 ,-- COMMENT='用户';

-- ----------------------------
-- Records of shop_user
-- ----------------------------
INSERT INTO shop_user VALUES ('1', '449c2a9d-a7f9-4dd7-b0b2-a176ff8bf25a.jpeg', '1588602065', 'male', '1588602065', '15011112222', '艾尼路', 'b5a51391f271f062867e5984e2fcffee', '8pgby');
INSERT INTO shop_user VALUES ('2', '184e85a7-41dd-4d1b-b67a-2632372ba257.jpg', '1588602065', 'male', 1588602065, '13581640280', '路飞', '154b20e371d69ccf7fab7402807a8b2d', 'q9i3r');
INSERT INTO shop_user VALUES ('3', '2c650bc2-666e-4621-92cd-36b4165d527c.jpeg', '1588602065', 'female', 1588602065, '13123234325', '娜美', 'd6d47d83b728917df24b45bf136f7155', 'bd6nu');
INSERT INTO shop_user VALUES ('4', '5422cb7c-015d-4c1b-a0d3-612c0d010d65.jpg', '1588602065', 'male', 1588602065, '13523921111', '香吉士', 'fa4ceda72c1fe22e062978a4282098f8', 'bzn8x');