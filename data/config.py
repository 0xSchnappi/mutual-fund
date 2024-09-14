'''
Author: 0xSchnappi 952768182@qq.com
Date: 2024-09-06 16:04:53
LastEditors: 0xSchnappi 952768182@qq.com
LastEditTime: 2024-09-14 09:31:03
FilePath: /mutual-fund/data/config.py
Description: 获取配置文件信息

Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
'''
import yaml
import configparser
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from dotenv import load_dotenv
import os
import logging
import psycopg2

# 创建一个日志记录器
logger = logging.getLogger(__name__)

class Config:
    _initialized = False
    username = None
    passwd = None
    db_host = None
    db_port = None
    db_username = None
    db_password = None
    db_database = None
    db_session = None

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
        
    @classmethod
    def load_env(cls):
        
        if cls._initialized:
            return
        
        # 加载.env文件
        load_dotenv()
        
        cls.db_host = os.getenv("FUND_DB_HOST")
        cls.db_port = os.getenv("FUND_DB_PORT")
        cls.db_username = os.getenv("FUND_DB_USERNAME")
        cls.db_password = os.getenv("FUND_DB_PASSWORD")
        cls.db_database = os.getenv("FUND_DB_DATABASE")
        
        # 获取获取股票信息的配置
        cls.username = os.getenv("STOCK_USERNAME")
        cls.passwd = os.getenv("STOCK_PASSWORD")
        
        
        if cls.db_host and cls.db_port and cls.db_username and cls.db_password and cls.db_database:
            url = f"postgresql+psycopg2://{cls.db_username}:{cls.db_password}@{cls.db_host}:{cls.db_port}/{cls.db_database}"
            logger.debug(url)
            print(url)
            
            # 初始化数据库连接
            engine = create_engine(url=url)
            
            # 创建DBSession类型
            cls.db_session = sessionmaker(bind=engine)
            
        cls._initialized = True
            
            
        
        
        
        
# 模块加载自动初始化         
Config.load_env()        