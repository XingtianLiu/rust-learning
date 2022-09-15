/*
 * @Author: lxt
 * @Date: 2022-09-15 16:12:42
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-15 16:25:42
 * @Description: // # 
 */
pub fn add_one(left: usize) -> usize {
    left + 1
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_works1(){
        assert_eq!(3,add_one(2))
    }
}