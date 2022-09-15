/*
 * @Author: lxt
 * @Date: 2022-09-15 16:22:21
 * @LastEditors: [lxt]
 * @LastEditTime: 2022-09-15 16:26:33
 * @Description: // # 
 */
pub fn add_two(left: usize) -> usize {
    left + 2
}

#[cfg(test)]
mod tests{
    #[test]
    fn it_works2(){
        assert_eq!(2+2,4)
    }
}