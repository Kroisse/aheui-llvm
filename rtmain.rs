#[feature(globs)];

extern mod rt;
use rt::*;

extern "C" {
    fn aheui_main();
}

fn main() {
    rt_init();

    unsafe {
        aheui_main();
    }
}
