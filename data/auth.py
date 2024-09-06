'''
Author: 0xSchnappi 952768182@qq.com
Date: 2024-09-06 16:03:25
LastEditors: 0xSchnappi 952768182@qq.com
LastEditTime: 2024-09-06 18:21:52
FilePath: /mutual-fund/data/auth.py
Description: 身份验证

Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
'''

import data.config as config 

from jqdatasdk import auth as jqauth

def auth():
    jqauth(config.Config.username, config.Config.passwd)
