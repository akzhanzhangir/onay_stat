# onay_stat
[<img alt="crates.io" src="https://img.shields.io/badge/crates.io-v0.1.0-orange">](https://crates.io/crates/onay_stat/0.1.0)
<a href="https://actions-badge.atrox.dev/djakish/onay_stat/goto?ref=main"><img alt="Build Status" src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fdjakish%2Fonay_stat%2Fbadge%3Fref%3Dmain&style=flat" /></a>

Simple crate for calculating sum, mean, median, median but sorted, range, variance, standard deviation, mode, max and min value and their indices.  
## Example
Import in Cargo.toml file
```rust
onay_stat="0.1.3"
```
Import and initialize from a f64 vector.
```rust
use onay_stat::*;
fn main(){
    let mut calc = Calc::new(vec![
            0.4814386506837457,
            0.0339385027520397,
            0.2382140377175458,
            0.2875186407007349,
            0.2041683180134608,
    ]).unwrap();
    calc.run_all();
    calc.display();
}
```
Printing single calculation
```rust
println!(calc.standart_deviation());
```
Or write the result in a variable
```rust
let sd = calc.standart_deviation();
```
Sample output from display method.
```bash
Data [0.4814386506837457, 0.0339385027520397, 0.2382140377175458, 0.2875186407007349, 0.2041683180134608]
Count 5
Total 1.245278149867527
Mean 0.2490556299735054
Median 0.2382140377175458
Sorted median 0.2382140377175458
Mode 0.2041683180134608
Range 0.447500147931706
Variance 0.02077781216605561
Standart Deviation 0.1441451080198548
Max 0.4814386506837457
Max indecies [0]
Min 0.0339385027520397
Min indecies [1]
```

