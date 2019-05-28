fn main() {
    let a = r##"This //~ ERROR too many `#` when terminating raw string
    is
    a
    very
    long
    string
    which
    goes
    over
    a
    b
    c
    d
    e
    f
    g
    h
    lines
    "###;
}
