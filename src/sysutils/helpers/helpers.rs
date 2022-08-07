use std::ops::Sub;




pub struct Helpers;





impl Helpers {
    pub fn u64overflowed_substract(a : &u64, b : &u64, default : &u64)-> u64{
        if (a < b) { return *default }
        else {return a-b}
    }

    pub fn u64overflowed_rangefit(min : &u64, max : &u64, val : &u64)-> u64{
        if(min<max) {
            if val > max {*max}
            else if val < min {*min}
            else {*val}
        }else {panic!("u64overflowed_rangefit : Max is inferior to min")}
    }

    pub fn u64closest_down_divider(val : &u64, step : &u64)-> u64{
        val - val%step
    }
    pub fn u64closest_up_divider(val : &u64, step : &u64)-> u64{
        Helpers::u64closest_down_divider(val,step) + step
    }
}