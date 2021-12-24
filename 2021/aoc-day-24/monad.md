# Reverse-engineering of each of the 14 subroutines of MONAD:

### Constants used in MONAD:

```rust
const X: [Int; 14] = [12, 12, 12, -9, -9, 14, 14, -10, 15, -2, 11, -15, -9, -3];
const Y: [Int; 14] = [9, 4, 2, 5, 1, 6, 11, 15, 7, 12, 15, 9, 12, 12];
const DIV: [Int; 14] = [1, 1, 1, 26, 26, 1, 1, 26, 1, 26, 1, 26, 26, 26];
```

### Digit 0

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 12
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 9
// mul y x
// add z y
```

```rust
fn d0(mut z: Int, w: Int) -> Int {
    if z % 26 + X[0] != w {
        z *= 26;
        z += w + Y[0]
    }
    z
}
```

### Digit 1

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 12
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 4
// mul y x
// add z y
```

```rust
fn d1(mut z: Int, w: Int) -> Int {
    if z % 26 + X[1] != w {
        z *= 26;
        z += w + Y[1];
    }

    z
}
```

### Digit 2

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 12
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 2
// mul y x
// add z y
```

```rust
fn d2(mut z: Int, w: Int) -> Int {
    if z % 26 + X[2] != w {
        z *= 26;
        z += w + Y[2];
    }

    z
}
```

### Digit 3

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 26
// add x -9
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 5
// mul y x
// add z y
```

```rust
fn d3(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[3];
    z /= 26;

    if x != w {
        z *= 26;
        z += w + Y[3];
    }

    z
}
```

### Digit 4

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 26
// add x -9
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 1
// mul y x
// add z y
```

```rust
fn d4(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[4];
    z /= 26;

    if x != w {
        z *= 26;
        z += w + Y[4];
    }

    z
}
```

### Digit 5

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 14
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 6
// mul y x
// add z y
```

```rust
fn d5(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[5];

    if x != w {
        z *= 26;
        z += w + Y[5];
    }

    z
}
```

### Digit 6

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 14
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 11
// mul y x
// add z y
```

```rust
fn d6(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[6];

    if x != w {
        z *= 26;
        z += w + Y[6];
    }

    z
}
```

### Digit 7

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 26
// add x -10
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 15
// mul y x
// add z y
```

```rust
fn d7(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[7];

    if x != w {
        z *= 26;
        z += w + Y[7];
    }

    z
}
```

### Digit 8

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 15
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 7
// mul y x
// add z y
```

```rust
fn d8(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[8];

    if x != w {
        z *= 26;
        z += w + Y[8];
    }

    z
}
```

### Digit 9

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 26
// add x -2
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 12
// mul y x
// add z y
```

```rust
fn d9(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[9];
    z /= 26;

    if x != w {
        z *= 26;
        z += w + Y[9];
    }

    z
}
```

### Digit 10

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 1
// add x 11
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 15
// mul y x
// add z y
```

```rust
fn d10(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[10];

    if x != w {
        z *= 26;
        z += w + Y[10];
    }

    z
}
```

### Digit 11

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 26
// add x -15
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 9
// mul y x
// add z y
```

```rust
fn d11(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[11];
    z /= 26;

    if x != w {
        z *= 26;
        z += w + Y[11];
    }

    z
}
```

### Digit 12

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 26
// add x -9
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 12
// mul y x
// add z y
```

```rust
fn d12(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[12];
    z /= 26;

    if x != w {
        z *= 26;
        z += w + Y[12];
    }

    z
}
```

### Digit 13

```
// inp w
// mul x 0
// add x z
// mod x 26
// div z 26
// add x -3
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 12
// mul y x
// add z y
```

```rust
fn d13(mut z: Int, w: Int) -> Int {
    let x = z % 26 + X[13];
    z /= 26;

    if x != w {
        z *= 26;
        z += w + Y[13];
    }

    z
}
```

## Common Function

It turns out that all 14 digits use one and the same function, but use different
constants. The only state that is preserved between the digits is the `z`
state. Therefore, it can be simplified to:

```rust
fn monad(input: [u8; 14]) -> bool {
    let mut z = 0;

    for i in 0..14 {
        let x = z % 26 + X[i];
        z /= DIV[i];

        if x != input[i] {
            z *= 26;
            z += w + Y[i];
        }
    }

    z == 0
}
```

### Analysis

The only way for `z` to become `0` is through one of the seven divisions by 26
at positions:

* pos 3: `z % 26 - 9`
* pos 4: `z % 26 - 9`
* pos 7: `z % 26 - 10`
* pos 9: `z % 26 - 2`
* pos 11: `z % 26 - 15`
* pos 12: `z % 26 - 9`
* pos 13: `z % 26 - 3`

In order to get the largest possible input value, the last division must
make `z` become 0, thus `z` should be at most 25 at that time.

Thus, we can construct a recursive function which will check all positions from
0 to 13, which will verify that `z` does not exceed the largest allowed value at
any position.