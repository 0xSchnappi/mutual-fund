'''
Author: 0xSchnappi 952768182@qq.com
Date: 2024-09-14 17:23:47
LastEditors: 0xSchnappi 952768182@qq.com
LastEditTime: 2024-09-15 22:56:29
FilePath: /mutual-fund/data/index.py
Description: 指数

Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
'''


from data.config import Config
import jqdatasdk
import logging

from sqlalchemy import Column, DateTime, String, Integer
from sqlalchemy.orm import declarative_base

logger = logging.getLogger(__name__)

Base = declarative_base()


class IndexInfo(Base):
    # 表名
    __tablename__ = "index_info"

    # 字段名
    id = Column(Integer, primary_key=True)
    code = Column(String(50))
    display_name = Column(String(50))
    name = Column(String(50))
    start_date = Column(DateTime)
    end_date = Column(DateTime)
    type = Column(String(50))


def update_all_index_info():

    data = jqdatasdk.get_all_securities(["index"])
    session = Config.db_session()
    index_infos = []
    for index, row in data.iterrows():
        logger.info("index:{} display name:{} name:{}".format(
            index, row["display_name"], row["name"]))
        index_info = IndexInfo(
            code=index,
            display_name=row["display_name"],
            name=row["name"],
            start_date=row["start_date"],
            end_date=row["end_date"],
            type=row["type"]
        )
        index_infos.append(index_info)

    session.add_all(index_infos)
    session.commit()
    session.flush()
