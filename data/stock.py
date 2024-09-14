'''
Author: 0xSchnappi 952768182@qq.com
Date: 2024-09-06 16:02:28
LastEditors: 0xSchnappi 952768182@qq.com
LastEditTime: 2024-09-14 09:27:17
FilePath: /mutual-fund/data/stock.py
Description: 获取股票信息

Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
'''
import jqdatasdk

from sqlalchemy import Column, String,Integer,DateTime,Float,Double
from sqlalchemy.orm import declarative_base
from data.config import Config
from datetime import datetime


# 创建对象的基类：
Base = declarative_base()

class StockInfo(Base):
    # 表的名字
    __tablename__ = 'stock_info'
    
    # 表结构
    id = Column(Integer, primary_key=True)
    code = Column(String(50))
    name = Column(String(50))
    display_name = Column(String(50))
    start_date = Column(String(50))
    end_date = Column(String(50))
    type = Column(String(50))
    
class DayPrice(Base):
    # 表名字
    __tablename__ = 'day_price'
    
    # 表结构
    id = Column(Integer, primary_key=True)
    code = Column(String(50))
    day_time =Column(DateTime)
    open = Column(Double)
    close = Column(Double)
    high = Column(Double)
    low = Column(Double)
    volume = Column(Double)
    money = Column(Double)
    

def get_stock_price(stock_code:str):
    # 获取数据
    data = jqdatasdk.get_price(stock_code, start_date="2023-05-30", end_date="2024-06-05")

    return data

def update_stock_all_info():
    df = jqdatasdk.get_all_securities()
    session = Config.db_session()
    for index, row in df.iterrows():
        # print(f'''Index: {index}, DisplayName: {row["display_name"]}, 
        #       Name:{row["name"]}, StartDate:{row["start_date"]},
        #       EndDate:{row["end_date"]}, Type:{row["type"]}''')
        new_stock = StockInfo(
            code = index,
            name = row["name"],
            display_name = row["display_name"],
            start_date = row["start_date"],
            end_date = row["end_date"],
            type = row["type"]
        )
        session.add(new_stock)
        session.commit()
        session.flush()
        
    session.close()
    
def update_stock_all_day_price():
    session = Config.db_session()
    stock_info = session.query(StockInfo).all()
    
    for info in stock_info:
        data = jqdatasdk.get_price(info.code, start_date="2023-05-30", end_date="2024-06-05")
            
        for index, row in data.iterrows():
            # print(f"index:{index}, row:{row}")
            new_day_price = DayPrice(
                code = info.code,
                day_time =index,
                open = float(row["open"]),
                close = float(row["close"]),
                high = float(row["high"]),
                low = float(row["low"]),
                volume = float(row["volume"]),
                money = float(row["money"]),
                
            )
            session.add(new_day_price)
            session.commit()
            session.flush()
    session.close()
    
        