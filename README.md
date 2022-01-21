# goorm edu rs kit, easy helper kit for rust code test environment.

The code test (a.k.a competitive programming) with rust has a problem that it is sometimes difficult to process the integer from standard input. This library solves such situations comfortably.

## Usage Example
```rs
use goorm_edu_rs_kit::goorm_helper;

fn main() {
    // Input Example
    // '8\n'
    let len: usize = goorm_helper::get_int();
    // Input Example
    // '10 20 30 40 50 60 70 80\n'   or
    // '10 20 30 40 50 60 70 80 \n'  or
    // '10 20 30 40 50 60 70 80   \n'
    let list: Vec<i32> = goorm_helper::get_vec_int(len);
    
    assert_eq!(len, 8);
    assert_eq!(&[10, 20, 30, 40, 50, 60, 70, 80], &list[..]);

    solve(&list);
}
```

### `goorm_helper::get_int()`
Get single integer from one line stdio.
* If system architecture is 32bit, cannot parse number bigger than i32::MAX or smaller than i32::MIN.
* If system architecture is 64bit, cannot parse number bigger than i64::MAX or smaller than i64::MIN.

### `goorm_helper::get_vec_int(len: usize)`
Get vectorized multiple integers with fixed length from one line stdio.
* Must contain integer count in one line.
* Experimental support, that super huge one line string that is bigger than 8KB buffer.

### Related Project
#### 10weeks-codingtest : https://github.com/dongyi-kim/10weeks-codingtest
