fn main() {
    let foo = r##"bar"####; //~ERROR too many `#` when terminating raw string
}
