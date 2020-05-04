
-- ----------------------------
-- Table structure for cms_channel
-- ----------------------------carg
CREATE TABLE cms_channel (

  id VARCHAR(36) NOT NULL PRIMARY KEY,
  create_by bigint(20)  ,-- COMMENT '创建人',
  create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '创建时间/注册时间',
  modify_by bigint(20)  ,-- COMMENT '最后更新人',
  modify_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ,-- COMMENT '最后更新时间',
  code varchar(64)  ,-- COMMENT '编码',
  name varchar(64)  -- COMMENT '名称'
) ;--ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8 ,-- COMMENT='文章栏目';

-- ----------------------------
-- Records of cms_channel
-- ----------------------------
INSERT INTO cms_channel VALUES ('1', '1', '1588602065', '1', '1588602065', 'news', '动态资讯');
INSERT INTO cms_channel VALUES ('2', '1', '1588602065', '1', '1588602065', 'product', '产品服务');
INSERT INTO cms_channel VALUES ('3', '1', '1588602065', '1', '1588602065', 'solution', '解决方案');
INSERT INTO cms_channel VALUES ('4', '1', '1588602065', '1', '1588602065', 'case', '精选案例');