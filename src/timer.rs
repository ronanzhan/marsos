use riscv::register::time;
use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;


pub fn get_time() -> usize {
    time::read()
}

/**
get_time()返回的是通电后时钟震荡了多少次
CLOCK_FREQ是时钟频率，即每秒震动多少次，TICKS_PER_SEC是用来表示每秒要切分多少时间分片
举个例子：比如当前是要切分100个，那么每个时间分片就是10ms，CLOCK_FREQ / TICKS_PER_SEC代表10ms需要震动多少次，
加上get_time()就是下个触发的时间点 ，本质是又计数了多少次

注意：时钟中断是在S特权级触发的，那么U特权级的应用所占cpu就会被打断
sie 这个 CSR，它的三个字段 ssie/stie/seie 分别控制 S 特权级的软件中断、时钟中断和外部中断的中断使能
 */
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}