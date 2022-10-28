# rust_bin_tree

- raindrops on roses
- whiskers on kittens
- these are a few of my favorite things

- *italic*
- **bold**

---

### h3
I need to learn markdown

---

I wanna talk about code
inline: `let x = 0;`

block: 
```rust

fn is_prime(num: i32) -> bool {
    if num < 2 { return false; }
    if num == 2 || num == 3 { return true; }
    else if num % 2 == 0 { return false; }
    else if num % 3 == 0 { return false; }
    else {
        let root = sqrt(num);

        let mut tester: i32 = 5;
        while tester <= root {
            if num % tester == 0 { return false; }
            if num % (tester + 2) == 0 { return false; }
            tester += 6;
        }
    }
    return true;
}
```
