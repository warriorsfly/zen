CREATE TABLE user_location
(
  uid VARCHAR(36) NOT NULL PRIMARY KEY,
  -- '用户ID',
  curr_nation VARCHAR(10) NOT NULL DEFAULT '' ,
  -- '所在地国',
  curr_province VARCHAR(10) NOT NULL DEFAULT '' ,
  -- '所在地省',
  curr_city VARCHAR(10) NOT NULL DEFAULT '' ,
  -- '所在地市',
  curr_district VARCHAR(20) NOT NULL DEFAULT '' ,
  -- '所在地地区',
  location VARCHAR(255) NOT NULL DEFAULT '' ,
  -- '具体地址',
  longitude float NOT NULL DEFAULT 0,
  -- '经度',
  latitude float NOT NULL DEFAULT 0,
  -- '纬度',
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  -- '修改时间',
)