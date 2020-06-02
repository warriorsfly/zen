CREATE TABLE user_location
(
  id INT NOT NULL PRIMARY KEY,
  -- '用户ID',
  curr_nation TEXT NOT NULL DEFAULT '' ,
  -- '所在地国',
  curr_province TEXT NOT NULL DEFAULT '' ,
  -- '所在地省',
  curr_city TEXT NOT NULL DEFAULT '' ,
  -- '所在地市',
  curr_district TEXT NOT NULL DEFAULT '' ,
  -- '所在地地区',
  location TEXT NOT NULL DEFAULT '' ,
  -- '具体地址',
  longitude FLOAT NOT NULL DEFAULT 0,
  -- '经度',
  latitude FLOAT NOT NULL DEFAULT 0,
  -- '纬度',
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  -- '修改时间',
)