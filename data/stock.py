'''
Author: 0xSchnappi 952768182@qq.com
Date: 2024-09-06 16:02:28
LastEditors: 0xSchnappi 952768182@qq.com
LastEditTime: 2024-09-06 17:51:50
FilePath: /mutual-fund/data/stock.py
Description: 获取股票信息

Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
'''
import jqdatasdk


def get_stock_price():
    # 获取数据
    data = jqdatasdk.get_price("000001.XSHE", start_date="2023-05-30", end_date="2024-06-05")
    return data