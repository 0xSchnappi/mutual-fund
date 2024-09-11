/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-11 09:28:02
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-11 10:22:52
 * @FilePath: /mutual-fund/src/fairings/cors.rs
 * @Description: rocket 用于对指定时机的一些回调操作，比如请求时、响应时
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

use rocket::fairing::{Fairing, Info, Kind};
pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }
}
