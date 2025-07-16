use anyhow::anyhow;
use std::string::String;

use crate::error::AppResult;

pub trait Convert {
    fn to_bool(&self) -> AppResult<bool>;
    fn to_i32(&self) -> AppResult<i32>;
    fn to_i64(&self) -> AppResult<i64>;
    fn to_u32(&self) -> AppResult<u32>;
}

impl Convert for String {
    //==================================================================================
    //= String 类型转换为 bool 类型
    //= String to bool
    fn to_bool(&self) -> AppResult<bool> {
        match self.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(anyhow!("Failed to convert String to bool").into()),
        }
    }

    //==================================================================================
    //= String 类型转换为 i32 类型
    //= String to i32
    fn to_i32(&self) -> AppResult<i32> {
        match self.parse::<i32>() {
            Ok(i) => Ok(i),
            Err(_) => Err(anyhow!("Failed to convert String to i32").into()),
        }
    }

    //==================================================================================
    //= String 类型转换为 i64 类型
    //= String to i64
    fn to_i64(&self) -> AppResult<i64> {
        match self.parse::<i64>() {
            Ok(i) => Ok(i),
            Err(_) => Err(anyhow!("Failed to convert String to i64").into()),
        }
    }

    //==================================================================================
    //= String 类型转换为 u32 类型
    //= String to u32
    fn to_u32(&self) -> AppResult<u32> {
        match self.parse::<u32>() {
            Ok(i) => Ok(i),
            Err(_) => Err(anyhow!("Failed to convert String to u32").into()),
        }
    }
}
