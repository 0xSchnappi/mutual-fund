'''
Author: 0xSchnappi 952768182@qq.com
Date: 2024-09-06 15:58:40
LastEditors: 0xSchnappi 952768182@qq.com
LastEditTime: 2024-09-14 09:30:06
FilePath: /mutual-fund/main.py
Description: 将股票数据获取到数据库

Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
'''
import data.config
from data.auth import auth
from data.stock import update_stock_all_info, update_stock_all_day_price
import logging

# 配置日志记录
logging.basicConfig(level=logging.DEBUG,
                    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s")



if __name__ == "__main__":
    auth()
    update_stock_all_info()
    update_stock_all_day_price()
    # print(data)
    
    
    
    