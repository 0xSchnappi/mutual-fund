'''
Author: 0xSchnappi 952768182@qq.com
Date: 2024-09-14 16:05:01
LastEditors: 0xSchnappi 952768182@qq.com
LastEditTime: 2024-09-14 17:01:47
FilePath: /mutual-fund/KLine/kline.py
Description: k线图绘制

Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
'''
import matplotlib.pyplot as plt

from data.config import Config
from data.stock import DayPrice


def year_kline(code = "000001.XSHE", start="2023-05-30 00:00:00", end="2024-06-05 00:00:00"):
    
    # 获取价格数据
    session = Config.db_session()
    
    datas = session.query(DayPrice).filter(DayPrice.code == code).all()
    print(datas)
    stock_price = []
    for day_price in datas:
        stock_price.append(day_price.close)

    print(stock_price)
    print()
    # 创建 fig 对象
    fig = plt.figure()
    
    # 增加大标题
    fig.suptitle('Year K Line', fontsize = 14, fontweight='bold')
    
    ax = fig.add_subplot(1, 1, 1)
    ax.set_title(code)
    
    # 设置x，y轴的标题
    ax.set_xlabel("{} ~ {}".format(start, end))
    # ax.set_ylabel("y label")
    
    

    # plot 方法绘图
    # 接受一个参数错位Y坐标的值
    # plt.plot([1, 2, 3, 4])
    plt.plot(list(range(1, len(stock_price)+1, 1)),stock_price )
    # plt.plot([0, len(stock_price), 1], [0, len(stock_price), 1])
    plt.savefig("./images/year_{}_kline.jpg".format(code))
    plt.show()
    
    