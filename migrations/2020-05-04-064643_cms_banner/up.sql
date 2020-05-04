
-- ----------------------------
-- Table structure for cms_banner
-- ----------------------------
CREATE TABLE cms_banner (

  id VARCHAR(36) NOT NULL PRIMARY KEY,
  created_by VARCHAR(36)  ,-- COMMENT '创建人',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '创建时间/注册时间',
  modify_by VARCHAR(36) ,-- COMMENT '最后更新人',
  modify_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '最后更新时间',
  id_file varchar(64)  ,-- COMMENT 'banner图id',
  page varchar(64)  ,-- COMMENT '界面',
  param varchar(128)  ,-- COMMENT '参数',
  title varchar(64)  ,-- COMMENT '标题',
  type varchar(32)  ,-- COMMENT '类型',
  url varchar(128)  -- COMMENT '点击banner跳转到url',
);-- ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8 ,-- COMMENT='文章';

-- ----------------------------
-- Records of cms_banner
-- ----------------------------
INSERT INTO cms_banner VALUES ('1', '1', '1588602065', null, 1588602065, '75b1e658-161e-4b12-83b0-abd2c1bead39.jpg', 'goods', '{\"id\":2}', '红米Rote8,打开外部链接', 'index', null);
INSERT INTO cms_banner VALUES ('2', '1', '1588602065', null, 1588602065, 'cfd733e0-4a8a-4b87-8f30-fb909025c647.jpg', 'goods', '{\"id\":1}', '红米8A', 'index', null);
INSERT INTO cms_banner VALUES ('3', '1', '1588602065', null, 1588602065, '2ba1e87f-f04e-40b5-8d99-63e035a9d752.jpg', 'https://microapp.gitee.io/linjiashop', null, '打开外部链接', 'index', null);
INSERT INTO cms_banner VALUES ('4', '1', '1588602065', null, 1588602065, '00950b78-0fc6-4e88-b663-07dc46a2b6df.jpg', 'goods', '{\"id\":15}', '不打开链接', 'product', null);
INSERT INTO cms_banner VALUES ('5', '1', '1588602065', null, 1588602065, '8974ee52-c261-440a-84d3-8f8c1bd43a6a.jpg', 'goods', '{\"id\":16}', '打打开站内链接', 'product', null);
INSERT INTO cms_banner VALUES ('6', '1', '1588602065', null, 1588602065, '14f9ce27-f133-4321-aeb5-aed470b794d6.jpg', 'goods', '{\"id\":11}', '打开外部链接', 'product', null);
INSERT INTO cms_banner VALUES ('7', '1', '1588602065', null, 1588602065, '0cbeb359-39de-42a9-9d19-96e9887a819e.jpg', 'http://flash-mobile.enilu.cn/#/index', null, '不打开链接', 'solution', null);
INSERT INTO cms_banner VALUES ('10', '1', '1588602065', null, 1588602065, '7e9ebc08-b194-4f85-8997-d97ccb0d2c2d.png', null, null, '不打开链接', 'case', null);