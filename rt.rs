#[crate_type = "lib"];
#[link(name = "rt", vers = "0.0")];

use std::local_data;
use std::io;

pub struct AheuiRt {
    dqs: ~[~[i32]],
}

local_data_key!(key_rt: AheuiRt)

#[no_mangle] #[inline(never)]
pub extern "C" fn aheui_getchar() -> char {
    let mut stdin = io::buffered::BufferedReader::new(io::stdin());
    print("input an unicode character: ");
    let line = stdin.read_line().unwrap();
    line.char_at(0)
}

#[no_mangle]
pub extern "C" fn aheui_putchar(c: char) {
    print!("{:c}", c);
}

#[no_mangle]
pub extern "C" fn aheui_getint() -> i32 {
    let mut stdin = io::buffered::BufferedReader::new(io::stdin());
    print("input an integer: ");
    let line = stdin.read_line().unwrap();
    println!("line: {:?}", line);
    from_str(line).unwrap()
}

#[no_mangle]
pub extern "C" fn aheui_putint(i: i32) {
    debug!("aheui_putint({:?})", i);
    print!("{:d}", i as int);
}

#[no_mangle]
pub extern "C" fn aheui_trace(x: i32, y: i32, c: char) {
    debug!("trace({:c}: {:d}, {:d})", c, x as int, y as int);
}

#[no_mangle]
pub extern "C" fn aheui_push(idx: i8, v: i32) {
    debug!("aheui_push(idx {:d}, val {:d})", idx as int, v as int);
    local_data::get_mut(key_rt, |ar| {
        let ar = ar.unwrap();
        match idx {
            27 => fail!("Aheui extension is not supported."),
            _ => {
                ar.dqs[idx].push(v);
            },
        }
        debug!("aheui_push: stack[{:d}]: {:?}", idx as int, ar.dqs[idx]);
    })
}

#[no_mangle]
pub extern "C" fn aheui_pop(idx: i8) -> i32 {
    local_data::get_mut(key_rt, |ar| {
        let ar = ar.unwrap();
        let ret = match idx {
            27 => fail!("Aheui extension is not supported."),
            21 => {
                ar.dqs[idx].shift()
            },
            _ => {
                ar.dqs[idx].pop()
            },
        };
        debug!("aheui_pop: stack[{:d}]: {:?}", idx as int, ar.dqs[idx]);
        ret
    })
}

#[no_mangle]
pub extern "C" fn aheui_dup(idx: i8) {
    local_data::get_mut(key_rt, |ar| {
        let ar = ar.unwrap();
        match idx {
            27 => fail!("Aheui extension is not supported."),
            21 => {
                let n = ar.dqs[idx][0];
                ar.dqs[idx].unshift(n);
            },
            _ => {
                let n = ar.dqs[idx][ar.dqs[idx].len() - 1];
                ar.dqs[idx].push(n)
            },
        }
        debug!("aheui_dup: stack[{:d}]: {:?}", idx as int, ar.dqs[idx]);
    })
}

#[no_mangle]
pub extern "C" fn aheui_swap(idx: i8) {
    local_data::get_mut(key_rt, |ar| {
        let ar = ar.unwrap();
        match idx {
            27 => fail!("Aheui extension is not supported."),
            21 => {
                let len = ar.dqs[idx].len();
                assert!(len >= 2);
                let m = ar.dqs[idx][0];
                let n = ar.dqs[idx][1];
                ar.dqs[idx][0] = n;
                ar.dqs[idx][1] = m;
            },
            _ => {
                let len = ar.dqs[idx].len();
                assert!(len >= 2);
                let m = ar.dqs[idx][len - 2];
                let n = ar.dqs[idx][len - 1];
                ar.dqs[idx][len - 2] = n;
                ar.dqs[idx][len - 1] = m;
            },
        }
        debug!("aheui_swap: stack[{:d}]: {:?}", idx as int, ar.dqs[idx]);
    })
}

pub fn rt_init() {
    let mut dqs = ~[];
    for _ in range(0, 26) {
        dqs.push(~[]);
    }
    let ar = AheuiRt {
        dqs: dqs,
    };

    local_data::set(key_rt, ar);
}
