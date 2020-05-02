
-- ----------------------------
-- Table structure for sys_file_info
-- ----------------------------
DROP TABLE IF EXISTS sys_file_info;
CREATE TABLE sys_file_info (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  create_by bigint(20) DEFAULT NULL ,-- COMMENT '创建人',
  create_at datetime DEFAULT NULL ,-- COMMENT '创建时间/注册时间',
  modify_by bigint(20) DEFAULT NULL ,-- COMMENT '最后更新人',
  modify_at datetime DEFAULT NULL ,-- COMMENT '最后更新时间',
  original_file_name varchar(255) DEFAULT NULL,
  real_file_name varchar(255) DEFAULT NULL
);-- ENGINE=InnoDB AUTO_INCREMENT=162 DEFAULT CHARSET=utf8 ,-- COMMENT='文件';

-- ----------------------------
-- Records of sys_file_info
-- ----------------------------
INSERT INTO sys_file_info VALUES ('1', '1', '2019-03-18 10:34:34', '1', '2019-03-18 10:34:34', 'test.png', '7e9ebc08-b194-4f85-8997-d97ccb0d2c2d.png');
INSERT INTO sys_file_info VALUES ('2', '1', '2019-03-18 10:54:04', '1', '2019-03-18 10:54:04', 'banner2.png', '7143c463-8c37-4e24-a6c9-3150f7a7dae9.jpg');
INSERT INTO sys_file_info VALUES ('3', '1', '2019-03-18 20:09:59', '1', '2019-03-18 20:09:59', 'test1.jpg', 'b651cfeb-ce41-452a-87ff-e2706880af63.jpg');
INSERT INTO sys_file_info VALUES ('31', '1', '2019-11-14 11:52:46', null, null, 'banner_mobile1.jpg', '5f4d38b6-3d4e-43c5-869d-bd9ea51340f2.jpg');
INSERT INTO sys_file_info VALUES ('32', '1', '2019-11-14 11:52:52', null, null, 'banner_mobile2.jpg', '4306017b-1c30-4fd0-b5e4-3ddf0e3b8e5c.jpg');
INSERT INTO sys_file_info VALUES ('33', '1', '2019-11-14 11:52:56', null, null, 'banner_mobile3.jpg', '2928da26-f3a7-4109-b2cf-7d9b7f68c354.jpg');
INSERT INTO sys_file_info VALUES ('34', '1', '2019-11-14 11:53:01', null, null, 'banner_notebook1.jpg', '2ceb5539-7a66-4c4c-835d-b383dd85cab6.jpg');
INSERT INTO sys_file_info VALUES ('36', '1', '2019-11-14 11:53:15', null, null, '红米1.jpg', '5ad73a65-3ae1-4a62-8d90-3cd4f5a48ef1.jpg');
INSERT INTO sys_file_info VALUES ('37', '1', '2019-11-14 11:53:18', null, null, '红米2.jpg', 'd05c4bd7-2c16-4ca9-9ab9-fc31f7000795.jpg');
INSERT INTO sys_file_info VALUES ('38', '1', '2019-11-14 11:53:21', null, null, '红米3.jpg', 'b64493ab-ab97-4b9c-84d6-5c0516d5e218.jpg');
INSERT INTO sys_file_info VALUES ('39', '1', '2019-11-14 11:53:25', null, null, '红米4.jpg', '95a1e558-cc2f-430b-b023-dac6659f4315.jpg');
INSERT INTO sys_file_info VALUES ('40', '1', '2019-11-14 11:53:28', null, null, '黑鲨1.jpg', '7322230b-b1bf-46a0-85ab-87a1e2a6d83a.jpg');
INSERT INTO sys_file_info VALUES ('41', '1', '2019-11-14 11:53:33', null, null, 'cc91.jpg', 'df14460f-6282-48ad-8a8d-d478deafaf73.jpg');
INSERT INTO sys_file_info VALUES ('42', '1', '2019-11-14 11:53:36', null, null, 'cc92.webp', 'c516fdca-a220-4af7-abcb-e4218cabb956.webp');
INSERT INTO sys_file_info VALUES ('43', '1', '2019-11-14 13:51:30', null, null, '电视43英寸.jpg', 'bcfd1fea-f484-4c85-b08d-46bbca55f07f.jpg');
INSERT INTO sys_file_info VALUES ('44', '1', '2019-11-14 13:51:38', null, null, '电视50英寸.jpg', 'b87fdaac-6925-4c6e-ba50-1f9d4a07440a.jpg');
INSERT INTO sys_file_info VALUES ('45', '1', '2019-11-14 13:51:41', null, null, '电视55英寸.jpg', '5d29c2e1-cb7b-4d66-9054-3afd56379894.jpg');
INSERT INTO sys_file_info VALUES ('46', '1', '2019-11-14 13:51:45', null, null, '电视58英寸.jpg', '774927e3-a98e-4936-87ad-9cf30c864331.jpg');
INSERT INTO sys_file_info VALUES ('47', '1', '2019-11-14 13:51:48', null, null, '电视65英寸.jpg', 'c2e2a916-ae91-4033-8996-db74e2d78872.jpg');
INSERT INTO sys_file_info VALUES ('48', '1', '2019-11-14 14:09:29', null, null, '笔记本air12.5.jpg', 'e15c2c57-4862-48fb-ac0e-ee7124fdf36f.5.jpg');
INSERT INTO sys_file_info VALUES ('49', '1', '2019-11-14 14:09:34', null, null, '笔记本air13.3.jpg', '43cd9071-1653-4e1e-8ae8-9cf25f0255f1.3.jpg');
INSERT INTO sys_file_info VALUES ('50', '1', '2019-11-14 14:09:38', null, null, '笔记本pro15.6.jpg', '59d2fb68-a8a4-4f35-b3a8-7f3bdcaf3ab9.6.jpg');
INSERT INTO sys_file_info VALUES ('51', '1', '2019-11-14 14:09:44', null, null, '笔记本pro15.6 MX250独显.jpg', '74ae1ec2-9e00-4189-bbfe-9ac916375506.6gtx.jpg');
INSERT INTO sys_file_info VALUES ('52', '1', '2019-11-14 14:09:49', null, null, '笔记本游戏本.jpg', '7c75ad57-4f1f-41cd-93c3-ec04b704e8bd.jpg');
INSERT INTO sys_file_info VALUES ('53', '1', '2019-11-14 14:23:02', null, null, '家电-互联网油烟机.jpg', 'c341af05-8ca4-410d-b17d-5bd40c48c151.jpg');
INSERT INTO sys_file_info VALUES ('54', '1', '2019-11-14 14:23:06', null, null, '家电-扁平滚筒洗衣机.jpg', '81296c2b-649a-48c3-b692-7b0d262f1f0b.jpg');
INSERT INTO sys_file_info VALUES ('55', '1', '2019-11-14 14:23:10', null, null, '家电-新风机.jpg', '1d06bb1f-cadb-49fa-a88c-dee33d48aa4f.jpg');
INSERT INTO sys_file_info VALUES ('56', '1', '2019-11-14 14:23:13', null, null, '家电-空调大1匹.jpg', 'b7d52cd5-6182-4e23-a468-0866997c2f06.jpg');
INSERT INTO sys_file_info VALUES ('57', '1', '2019-11-14 14:23:16', null, null, '家电-米家两门冰箱.jpg', '10188f57-915b-40c4-9eb9-9248895b1d78.jpg');
INSERT INTO sys_file_info VALUES ('58', '1', '2019-11-14 14:23:21', null, null, '家电-米家洗烘一体机.jpg', 'b96c12d5-444c-47fe-8ea9-e08ba18fe500.jpg');
INSERT INTO sys_file_info VALUES ('59', '1', '2019-11-14 14:23:25', null, null, '家电-风冷对开门冰箱.jpg', 'f5a918a8-c820-4d4b-b976-bf19348e24d4.jpg');
INSERT INTO sys_file_info VALUES ('60', '1', '2019-11-14 14:57:24', null, null, '小米CC9美图定制版1.jpg', '15516e0b-7787-4223-8c21-927d5cd3c1b2.jpg');
INSERT INTO sys_file_info VALUES ('61', '1', '2019-11-14 14:57:29', null, null, '小米CC9美图定制版2.jpg', '2c4fa9d5-3d4b-4964-bfb5-7a63f3e25aeb.jpg');
INSERT INTO sys_file_info VALUES ('62', '1', '2019-11-14 14:57:32', null, null, '小米CC9美图定制版3.jpg', '2db4fc3e-57da-42fb-a8e7-d66957318b04.jpg');
INSERT INTO sys_file_info VALUES ('63', '1', '2019-11-14 14:57:35', null, null, '小米CC9美图定制版4.jpg', '57a299c0-6c3f-4d2b-b5a1-8c22a6b1b625.jpg');
INSERT INTO sys_file_info VALUES ('64', '1', '2019-11-14 16:00:17', null, null, '黑鲨手机1.jpg', 'e0e358ac-9ff2-455c-a745-48a83d3f1fd6.jpg');
INSERT INTO sys_file_info VALUES ('65', '1', '2019-11-14 16:00:20', null, null, '黑鲨手机2.jpg', 'c9de090b-166c-4d23-a52e-a4959fa636d9.jpg');
INSERT INTO sys_file_info VALUES ('66', '1', '2019-11-14 16:00:24', null, null, '黑鲨手机3.jpg', 'd9d5ac78-8e14-4f6d-992f-b2ffc4eb4afd.jpg');
INSERT INTO sys_file_info VALUES ('67', '1', '2019-11-14 16:00:26', null, null, '黑鲨手机4.jpg', '1cd9b0b1-a12b-47ed-bdf6-98be9a899b26.jpg');
INSERT INTO sys_file_info VALUES ('68', '1', '2019-11-14 16:05:17', null, null, '红米K201.jpg', '37f5710e-1fe3-48f0-b063-838593fbb2b9.jpg');
INSERT INTO sys_file_info VALUES ('69', '1', '2019-11-14 16:05:20', null, null, '红米K202.jpg', '8f42bccb-0550-4e60-9b54-663c274669e9.jpg');
INSERT INTO sys_file_info VALUES ('70', '1', '2019-11-14 16:05:23', null, null, '红米K203.jpg', '709136ce-19aa-450f-a48f-ef502f07ed2e.jpg');
INSERT INTO sys_file_info VALUES ('71', '1', '2019-11-14 16:10:06', null, null, 'CC9Pro1.jpg', '924792ff-e20f-440c-837d-6e476c19e470.jpg');
INSERT INTO sys_file_info VALUES ('72', '1', '2019-11-14 16:10:09', null, null, 'CC9Pro2.jpg', 'f2cb462e-a381-4f99-b572-f63c45821d17.jpg');
INSERT INTO sys_file_info VALUES ('73', '1', '2019-11-14 16:10:12', null, null, 'CC9Pro3.jpg', '065ce7db-07a2-4331-858e-9ad6c4625c3f.jpg');
INSERT INTO sys_file_info VALUES ('74', '1', '2019-11-14 16:10:16', null, null, 'CC9Pro4.jpg', 'f7ba31c4-fce8-4d95-a8a0-e70f7f76063d.jpg');
INSERT INTO sys_file_info VALUES ('75', '1', '2019-11-14 16:10:19', null, null, 'CC9Pro5.jpg', '76101509-4ef4-4da1-946a-852a2f655cc2.jpg');
INSERT INTO sys_file_info VALUES ('76', '1', '2019-11-14 16:30:22', null, null, '红米81.jpg', 'b1b53d00-e7c7-460e-b953-64ac2a05768c.jpg');
INSERT INTO sys_file_info VALUES ('77', '1', '2019-11-14 16:30:25', null, null, '红米82.jpg', '65a7d4f0-fa1e-49b0-bc6e-517f0c0d5b9a.jpg');
INSERT INTO sys_file_info VALUES ('78', '1', '2019-11-14 16:30:28', null, null, '红米83.jpg', '0ff25d14-4a85-4af7-9556-8f461b7437c3.jpg');
INSERT INTO sys_file_info VALUES ('79', '1', '2019-11-14 16:30:31', null, null, '红米84.jpg', '154ccdaa-a112-434b-a56b-5e1ee1a10ee6.jpg');
INSERT INTO sys_file_info VALUES ('80', '1', '2019-11-14 16:30:35', null, null, '红米85.jpg', 'a6e96c16-b267-4a0c-9203-e8adbc239824.jpg');
INSERT INTO sys_file_info VALUES ('81', '1', '2019-11-14 16:57:34', null, null, '红米8A1.jpg', '1b129a47-cda2-4722-ab02-d8646552ffaa.jpg');
INSERT INTO sys_file_info VALUES ('82', '1', '2019-11-14 16:57:38', null, null, '红米8A2.jpg', '22119bd1-1326-4c56-916c-71608b457928.jpg');
INSERT INTO sys_file_info VALUES ('83', '1', '2019-11-14 16:57:40', null, null, '红米8A3.jpg', 'd0dc9531-94a1-4236-98c8-7f8fdcaa79c0.jpg');
INSERT INTO sys_file_info VALUES ('84', '1', '2019-11-14 16:57:44', null, null, '红米8A4.jpg', '7d4f5887-f243-4478-b892-9fff830d02d4.jpg');
INSERT INTO sys_file_info VALUES ('85', '1', '2019-11-14 17:06:46', null, null, '电视4A65英寸1.jpg', '95542191-fb1e-4afb-abcf-ef71c4364c3c.jpg');
INSERT INTO sys_file_info VALUES ('86', '1', '2019-11-14 21:23:54', null, null, '电视4A58英寸1.jpg', 'b2cfd917-50ea-4721-8a7b-092b2909b8eb.jpg');
INSERT INTO sys_file_info VALUES ('87', '1', '2019-11-14 21:24:01', null, null, '电视4A50英寸.jpg', 'd7680fd5-03e0-4c62-92b8-a154bca050cc.jpg');
INSERT INTO sys_file_info VALUES ('88', '1', '2019-11-14 21:24:07', null, null, '电视55英寸1.jpg', 'a54cf50a-a2fc-4bb5-bf3d-5a9edc54a9a8.jpg');
INSERT INTO sys_file_info VALUES ('89', '1', '2019-11-14 21:24:10', null, null, '电视55英寸2.jpg', '4e63b4d9-d21a-42cf-bd72-51f2e1adc7e9.jpg');
INSERT INTO sys_file_info VALUES ('90', '1', '2019-11-14 21:24:13', null, null, '电视55英寸3.jpg', 'f1ad08e7-ea7c-4ba7-9354-9d5eded26c62.jpg');
INSERT INTO sys_file_info VALUES ('91', '1', '2019-11-14 21:27:11', null, null, '电视43英寸.jpg', 'c5a5a9fc-ff98-4c46-8c95-39216be4bfcc.jpg');
INSERT INTO sys_file_info VALUES ('92', '1', '2019-11-14 21:46:18', null, null, '笔记本Air12.5英寸1.jpg', '4e7b78fa-9b24-4d6a-beaf-31cc644515ea.5英寸1.jpg');
INSERT INTO sys_file_info VALUES ('93', '1', '2019-11-14 21:46:21', null, null, '笔记本Air12.5英寸2.jpg', 'b9407f5a-9be3-4513-8136-a8c1dcc9e23f.5英寸2.jpg');
INSERT INTO sys_file_info VALUES ('94', '1', '2019-11-14 21:46:24', null, null, '笔记本Air12.5英寸3.jpg', 'a4e2a1e1-05e1-45a7-b2b3-acd099a8e130.5英寸3.jpg');
INSERT INTO sys_file_info VALUES ('95', '1', '2019-11-14 21:46:28', null, null, '笔记本Air12.5英寸4.jpg', 'f28dea0d-a50b-47c2-8792-10f66441a5d2.5英寸4.jpg');
INSERT INTO sys_file_info VALUES ('96', '1', '2019-11-14 21:52:16', null, null, '笔记本13.3英寸1.jpg', 'ea6ffb3a-6f10-4062-b71a-22f5e6f7375f.3英寸1.jpg');
INSERT INTO sys_file_info VALUES ('97', '1', '2019-11-14 21:52:19', null, null, '笔记本13.3英寸2.jpg', '956407d6-c9ba-46a0-804f-c744d7eb836e.3英寸2.jpg');
INSERT INTO sys_file_info VALUES ('98', '1', '2019-11-14 21:52:21', null, null, '笔记本13.3英寸3.jpg', '5822699c-35a1-4950-8670-fb4ca817dd62.3英寸3.jpg');
INSERT INTO sys_file_info VALUES ('99', '1', '2019-11-14 21:52:24', null, null, '笔记本13.3英寸4.jpg', '6af94ec5-2542-41b1-ab4b-2c07bf4d7774.3英寸4.jpg');
INSERT INTO sys_file_info VALUES ('100', '1', '2019-11-14 21:52:27', null, null, '笔记本13.3英寸5.jpg', '857c7096-9948-4106-90a4-70df5f2124c9.3英寸5.jpg');
INSERT INTO sys_file_info VALUES ('101', '1', '2019-11-14 21:52:32', null, null, '笔记本13.3英寸6.jpg', 'b7b60643-5989-4643-abf9-77ed0680c889.3英寸6.jpg');
INSERT INTO sys_file_info VALUES ('102', '1', '2019-11-14 21:58:06', null, null, '笔记本15.6英寸1.jpg', 'ea2d1f35-d825-433a-ba5e-a098214a106a.6英寸1.jpg');
INSERT INTO sys_file_info VALUES ('103', '1', '2019-11-14 21:58:09', null, null, '笔记本15.6英寸2.jpg', 'c384e530-2b6a-4637-82dc-022261c7c407.6英寸2.jpg');
INSERT INTO sys_file_info VALUES ('104', '1', '2019-11-14 21:58:12', null, null, '笔记本15.6英寸3.jpg', '0a6ebee6-42bf-4b20-a81e-7f516c7dae1e.6英寸3.jpg');
INSERT INTO sys_file_info VALUES ('105', '1', '2019-11-14 21:58:14', null, null, '笔记本15.6英寸4.jpg', '40ce1fe7-5f60-4a35-958c-96fe9fa5dcda.6英寸4.jpg');
INSERT INTO sys_file_info VALUES ('106', '1', '2019-11-14 22:10:04', null, null, '笔记本Pro15.6英寸1.jpg', '123b59b1-13e6-4729-8621-d2f3ad9ecb41.6英寸1.jpg');
INSERT INTO sys_file_info VALUES ('107', '1', '2019-11-14 22:10:07', null, null, '笔记本Pro15.6英寸2.jpg', '4afa34b3-d44f-4322-a969-8f6b88b6027d.6英寸2.jpg');
INSERT INTO sys_file_info VALUES ('108', '1', '2019-11-14 22:10:10', null, null, '笔记本Pro15.6英寸3.jpg', '0b71273e-ef45-4ccd-b146-459be1e793a7.6英寸3.jpg');
INSERT INTO sys_file_info VALUES ('109', '1', '2019-11-14 22:10:12', null, null, '笔记本Pro15.6英寸4.jpg', '75f54336-fe51-42ee-a4fe-534abe9f5833.6英寸4.jpg');
INSERT INTO sys_file_info VALUES ('110', '1', '2019-11-14 22:19:43', null, null, '笔记本游戏版1.jpg', 'fb4902b6-e570-454a-97f9-6fb92f65e636.jpg');
INSERT INTO sys_file_info VALUES ('111', '1', '2019-11-14 22:20:41', null, null, '笔记本游戏版2.jpg', 'd7edab0c-6f77-4eaa-8312-b7dc8cb682a1.jpg');
INSERT INTO sys_file_info VALUES ('112', '1', '2019-11-14 22:20:44', null, null, '笔记本游戏版3.jpg', '105eb759-5c5a-4741-ac43-eb3caa910088.jpg');
INSERT INTO sys_file_info VALUES ('113', '1', '2019-11-14 22:20:47', null, null, '笔记本游戏版4.jpg', '34a87291-8922-4be3-9757-ed0f6777babc.jpg');
INSERT INTO sys_file_info VALUES ('114', '1', '2019-11-14 22:20:52', null, null, '笔记本游戏版5.jpg', '8d02883f-ad75-4578-9153-ce872613e7b3.jpg');
INSERT INTO sys_file_info VALUES ('115', '1', '2019-11-14 22:33:29', null, null, '风冷对开门冰箱1.jpg', 'cd8bfa63-d77d-4b20-beda-b2f57774c149.jpg');
INSERT INTO sys_file_info VALUES ('116', '1', '2019-11-14 22:33:32', null, null, '风冷对开门冰箱2.jpg', 'd6bb51c1-856e-4134-abbc-a59c4ce46603.jpg');
INSERT INTO sys_file_info VALUES ('117', '1', '2019-11-14 22:33:36', null, null, '风冷对开门冰箱3.jpg', 'ede83076-804d-4140-b3b3-fdae640793d9.jpg');
INSERT INTO sys_file_info VALUES ('118', '1', '2019-11-14 22:33:39', null, null, '风冷对开门冰箱4.jpg', 'ea110379-87ec-42b2-aeaf-5e27410902f5.jpg');
INSERT INTO sys_file_info VALUES ('119', '1', '2019-11-14 22:33:48', null, null, '风冷对开门冰箱5.jpg', 'b45f336e-983a-4ebf-aa3a-528e6aa2f390.jpg');
INSERT INTO sys_file_info VALUES ('121', '1', '2019-11-14 22:36:22', null, null, '米家洗烘一体机1.jpg', '0c313b82-6814-408c-ab37-0dde82172fc6.jpg');
INSERT INTO sys_file_info VALUES ('122', '1', '2019-11-14 22:36:24', null, null, '米家洗烘一体机2.jpg', 'd2c3b8a4-1ece-4f1f-a222-9f9e0ee11b15.jpg');
INSERT INTO sys_file_info VALUES ('123', '1', '2019-11-14 22:36:27', null, null, '米家洗烘一体机3.jpg', '08eff5b6-69b6-411d-9d1e-7f6a97d5c6ea.jpg');
INSERT INTO sys_file_info VALUES ('124', '1', '2019-11-14 22:36:30', null, null, '米家洗烘一体机4.jpg', 'f74e5bc6-a0c7-4e1b-ab9b-7efb60ec21c2.jpg');
INSERT INTO sys_file_info VALUES ('125', '1', '2019-11-14 22:38:45', null, null, '米家两门冰箱1.jpg', '77c51323-027d-434c-bed4-18ca7d532b6f.jpg');
INSERT INTO sys_file_info VALUES ('126', '1', '2019-11-14 22:38:47', null, null, '米家两门冰箱2.jpg', '03eae8fc-7835-486e-b682-ded7dad51cf6.jpg');
INSERT INTO sys_file_info VALUES ('127', '1', '2019-11-14 22:38:50', null, null, '米家两门冰箱3.jpg', '298a3c72-8e71-400b-a1ef-e75264c53e11.jpg');
INSERT INTO sys_file_info VALUES ('128', '1', '2019-11-14 22:38:56', null, null, '米家两门冰箱4.jpg', '432ddf59-c823-494c-aba7-a94c2b83093d.jpg');
INSERT INTO sys_file_info VALUES ('129', '1', '2019-11-14 22:41:04', null, null, '空调C11.jpg', '9e18f183-0de6-4137-9ce7-3d5d1bc33bc0.jpg');
INSERT INTO sys_file_info VALUES ('130', '1', '2019-11-14 22:41:07', null, null, '空调C12.jpg', '9fc649a0-e1c6-4ddc-9b60-18205136e90b.jpg');
INSERT INTO sys_file_info VALUES ('131', '1', '2019-11-14 22:41:10', null, null, '空调C13.jpg', 'de767e22-bb4b-4d6a-8b31-0d0f002f4f51.jpg');
INSERT INTO sys_file_info VALUES ('132', '1', '2019-11-14 22:42:52', null, null, '米家新风机1.jpg', '9dab6724-632a-4482-87a7-9bbc0d1524b9.jpg');
INSERT INTO sys_file_info VALUES ('133', '1', '2019-11-14 22:42:56', null, null, '米家新风机2.jpg', '50f43ad5-5d39-4c26-b698-e34f6768f1c8.jpg');
INSERT INTO sys_file_info VALUES ('134', '1', '2019-11-14 22:43:00', null, null, '米家新风机3.jpg', 'c1b97aeb-e15d-4297-a96f-dadeac8d1104.jpg');
INSERT INTO sys_file_info VALUES ('135', '1', '2019-11-14 22:43:02', null, null, '米家新风机4.jpg', 'b9a648c7-0305-45b3-b10a-d1dbd6ca7bf6.jpg');
INSERT INTO sys_file_info VALUES ('136', '1', '2019-11-14 22:45:01', null, null, '变频滚筒洗烘一体机1.jpg', '1ceb9b33-1299-4f8a-a724-7078e96a0dd4.jpg');
INSERT INTO sys_file_info VALUES ('137', '1', '2019-11-14 22:45:12', null, null, '变频滚筒洗烘一体机2.jpg', '5ba1aa87-3758-4a5b-b76c-4d5b3cefdf4b.jpg');
INSERT INTO sys_file_info VALUES ('138', '1', '2019-11-14 22:45:15', null, null, '变频滚筒洗烘一体机3.jpg', '7174c1b3-aaea-4c0c-9942-57804dea69b6.jpg');
INSERT INTO sys_file_info VALUES ('139', '1', '2019-11-14 22:45:18', null, null, '变频滚筒洗烘一体机4.jpg', '411e618d-36ae-4b4b-aa77-cf4f91f61d79.jpg');
INSERT INTO sys_file_info VALUES ('140', '1', '2019-11-14 22:48:08', null, null, '吸油烟机1.jpg', '502be2c9-bbf0-40ed-af89-4bd5c1df94d5.jpg');
INSERT INTO sys_file_info VALUES ('141', '1', '2019-11-14 22:48:11', null, null, '吸油烟机2.jpg', 'f98cf0d2-2a08-4709-b393-831315659efa.jpg');
INSERT INTO sys_file_info VALUES ('142', '1', '2019-11-14 22:48:14', null, null, '吸油烟机3.jpg', '2508b502-db36-45c5-92af-00ab7a7ef61a.jpg');
INSERT INTO sys_file_info VALUES ('143', '1', '2019-11-15 14:55:09', null, null, 'banner_mobile1.jpg', '75b1e658-161e-4b12-83b0-abd2c1bead39.jpg');
INSERT INTO sys_file_info VALUES ('144', '1', '2019-11-15 14:55:11', null, null, 'banner_mobile2.jpg', 'cfd733e0-4a8a-4b87-8f30-fb909025c647.jpg');
INSERT INTO sys_file_info VALUES ('145', '1', '2019-11-15 14:55:14', null, null, 'banner_mobile3.jpg', '2ba1e87f-f04e-40b5-8d99-63e035a9d752.jpg');
INSERT INTO sys_file_info VALUES ('146', '1', '2019-11-15 15:39:58', null, null, 'banner-笔记本1.jpg', '00950b78-0fc6-4e88-b663-07dc46a2b6df.jpg');
INSERT INTO sys_file_info VALUES ('147', '1', '2019-11-15 15:40:00', null, null, 'banner-笔记本2.jpg', '8974ee52-c261-440a-84d3-8f8c1bd43a6a.jpg');
INSERT INTO sys_file_info VALUES ('148', '1', '2019-11-15 15:44:22', null, null, 'banner电视.jpg', '14f9ce27-f133-4321-aeb5-aed470b794d6.jpg');
INSERT INTO sys_file_info VALUES ('149', '1', '2019-11-15 15:46:49', null, null, 'banner-家电.jpg', '0cbeb359-39de-42a9-9d19-96e9887a819e.jpg');
INSERT INTO sys_file_info VALUES ('150', '1', '2020-01-09 20:01:43', null, null, 'topic1.jpg', '4b1f6265-f751-44eb-86d6-8f2f5d4725ab.jpg');
INSERT INTO sys_file_info VALUES ('151', '1', '2020-01-09 20:01:47', null, null, 'topic2.jpg', '9f6604ff-2f3a-456c-aab3-dc2979ae0406.jpg');
INSERT INTO sys_file_info VALUES ('152', '1', '2020-01-09 20:01:52', null, null, 'topic3.jpg', '53d377be-ff01-4fb8-8e8b-e8d3c97678f6.jpg');
INSERT INTO sys_file_info VALUES ('153', '1', '2020-01-09 20:01:56', null, null, 'topic4.jpg', '01d2db50-7091-4299-b5d2-2cee0eec006c.jpg');
INSERT INTO sys_file_info VALUES ('154', '1', '2020-01-09 20:02:00', null, null, 'topic5.jpg', 'f335148e-d583-47c3-ad91-0b062ef3885d.jpg');
INSERT INTO sys_file_info VALUES ('155', '1', '2020-01-09 20:02:03', null, null, 'topic6.jpg', '7e195bf0-b450-4a1c-934d-80df766581a0.jpg');
INSERT INTO sys_file_info VALUES ('156', '1', '2020-01-13 11:39:35', null, null, '机器人1.jpg', '477bc0ea-60a8-4edb-b115-6ae6c92bd678.jpg');
INSERT INTO sys_file_info VALUES ('157', '1', '2020-01-13 11:39:38', null, null, '机器人2.jpg', '91b9b06d-c157-4ba2-bdc4-8f21fd2820c8.jpg');
INSERT INTO sys_file_info VALUES ('158', '1', '2020-03-01 16:49:53', null, null, '8b7a39cd8c7f7e8de7ea0975c4c25a20.jpeg', '449c2a9d-a7f9-4dd7-b0b2-a176ff8bf25a.jpeg');
INSERT INTO sys_file_info VALUES ('159', '2', '2020-03-01 16:50:56', null, null, 'a8e875870d1b4ad700a8d0c54d4fd3b0.jpg', '184e85a7-41dd-4d1b-b67a-2632372ba257.jpg');
INSERT INTO sys_file_info VALUES ('160', '3', '2020-03-01 16:52:02', null, null, '03d00430ff1823f2a1d9fae4e3616f8b.jpeg', '2c650bc2-666e-4621-92cd-36b4165d527c.jpeg');
INSERT INTO sys_file_info VALUES ('161', '4', '2020-03-01 16:52:47', null, null, 'dc241354f9bf60c9a88d5a6ebeac90f3.jpg', '5422cb7c-015d-4c1b-a0d3-612c0d010d65.jpg');
