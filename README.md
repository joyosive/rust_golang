### 1. Create project with cargo

```bash
mkdir lib && cd lib
```

```bash
cargo new --lib rustdemo
```
Check directory structure by 
```bash
tree rustdemo 
```

### 2. Prepare rust code
The function name exposed by rust is called `rustdemo`, receive an external parameter and print it out. Then set a string from rust.

`CString::new(str_name).unwrap().into_raw()Is converted to the original pointer for later processing by the C language.
`

### 3. Compile rust code

```bash
cargo build --release
```

View the generated file, which is a.so file
`ls target/release/librustdemo.so 
`

### 4. Prepare go code
Go code defines a string, passes it to the rustdemo function, and then prints the string processed by C.

### 5. Declare header file 

In order for the Go program to call the rust function normally, one needs to declare its header file in lib/rustdemo.h 

Write the following in:

```bash
char* rustdemo(char *name);
```

### 6. Compile code
 
Move .so file to lib folder & compile 

```bash
 go build -o go-rust  -ldflags="-r ./lib" main.go
 ```

```bash
./go-rust 
```
