
-- ----------------------------
-- Table structure for sys_file_info
-- ----------------------------
DROP TABLE IF EXISTS sys_file_info;
CREATE TABLE sys_file_info (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  created_by VARCHAR(36) ,-- COMMENT '创建人',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '创建时间/注册时间',
  updated_by VARCHAR(36) ,-- COMMENT '最后更新人',
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '最后更新时间',
  original_file_name varchar(255) ,
  real_file_name varchar(255) 
);-- ENGINE=InnoDB AUTO_INCREMENT=162 DEFAULT CHARSET=utf8 ,-- COMMENT='文件';

-- ----------------------------
-- Records of sys_file_info
-- ----------------------------
INSERT INTO sys_file_info VALUES ('1', '1', '1588602065', '1', '1588602065', 'test.png', '7e9ebc08-b194-4f85-8997-d97ccb0d2c2d.png');
INSERT INTO sys_file_info VALUES ('2', '1', '1588602065', '1', '1588602065', 'banner2.png', '7143c463-8c37-4e24-a6c9-3150f7a7dae9.jpg');
INSERT INTO sys_file_info VALUES ('3', '1', '1588602065', '1', '1588602065', 'test1.jpg', 'b651cfeb-ce41-452a-87ff-e2706880af63.jpg');
INSERT INTO sys_file_info VALUES ('31', '1', '1588602065', null, 1588602065, 'banner_mobile1.jpg', '5f4d38b6-3d4e-43c5-869d-bd9ea51340f2.jpg');
INSERT INTO sys_file_info VALUES ('32', '1', '1588602065', null, 1588602065, 'banner_mobile2.jpg', '4306017b-1c30-4fd0-b5e4-3ddf0e3b8e5c.jpg');
INSERT INTO sys_file_info VALUES ('33', '1', '1588602065', null, 1588602065, 'banner_mobile3.jpg', '2928da26-f3a7-4109-b2cf-7d9b7f68c354.jpg');
INSERT INTO sys_file_info VALUES ('34', '1', '1588602065', null, 1588602065, 'banner_notebook1.jpg', '2ceb5539-7a66-4c4c-835d-b383dd85cab6.jpg');
INSERT INTO sys_file_info VALUES ('36', '1', '1588602065', null, 1588602065, '红米1.jpg', '5ad73a65-3ae1-4a62-8d90-3cd4f5a48ef1.jpg');
INSERT INTO sys_file_info VALUES ('37', '1', '1588602065', null, 1588602065, '红米2.jpg', 'd05c4bd7-2c16-4ca9-9ab9-fc31f7000795.jpg');
INSERT INTO sys_file_info VALUES ('38', '1', '1588602065', null, 1588602065, '红米3.jpg', 'b64493ab-ab97-4b9c-84d6-5c0516d5e218.jpg');
INSERT INTO sys_file_info VALUES ('39', '1', '1588602065', null, 1588602065, '红米4.jpg', '95a1e558-cc2f-430b-b023-dac6659f4315.jpg');
INSERT INTO sys_file_info VALUES ('40', '1', '1588602065', null, 1588602065, '黑鲨1.jpg', '7322230b-b1bf-46a0-85ab-87a1e2a6d83a.jpg');
