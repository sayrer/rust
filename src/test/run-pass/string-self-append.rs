// xfail-stage0

use std;
import std::str;

fn main()
{
    // Make sure we properly handle repeated self-appends.
    let str a = "A";
    auto i = 20;
    auto expected_len = 1u;
    while (i > 0) {
        log_err str::byte_len(a);
        assert (str::byte_len(a) == expected_len);
        a += a;
        i -= 1;
        expected_len *= 2u;
    }
}
