'''
Author: 0xSchnappi 952768182@qq.com
Date: 2024-09-06 16:04:53
LastEditors: 0xSchnappi 952768182@qq.com
LastEditTime: 2024-09-06 17:30:55
FilePath: /mutual-fund/data/config.py
Description: 获取配置文件信息

Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
'''
import yaml


class Config:
    _initialized = False
    username = None
    passwd = None
    
    @classmethod
    def initialize(cls):
        if cls._initialized:
            return
        try:
            with open(".config.yaml", 'rb') as f:
                documents = yaml.safe_load_all(f)
                config = next(documents, {})
                cls.username = config.get("username", None)
                cls.passwd = config.get("passwd", None)
        except FileNotFoundError:
            raise(".config.yaml not found")
        except yaml.YAMLError as e:
            raise("Error parsing YAML file: {e}")
        
        
# 模块加载自动初始化
Config.initialize()                
        