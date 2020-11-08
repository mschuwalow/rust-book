fn creating() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let hello = String::from("مكيلع ملاسلا");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("םוֹלׁ ָ ש");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
#[test]
fn updating() {
    let mut s = String::from("foo");
    s.push_str("bar");
    assert_eq!(s, "foobar");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    assert_eq!(s1, "foobar");
    println!("{}", s2);
    let mut s = String::from("lo");
    s.push('l');
    assert_eq!(s, "lol");
}
#[test]
fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "Hello, world!");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    assert_eq!(s, "tic-tac-toe");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    assert_eq!(s, "tic-tac-toe");
}
#[test]
fn indexing() {
    let s1 = String::from("hello");
    //let h = s1[0]; will not compile
    let len = String::from("Hola").len();
    assert_eq!(len, 4);
    let len = String::from("Здравствуйте").len();
    assert_eq!(len, 24);
    let hello = "Здравствуйте";
    // let answer = &hello[0]; will not compile
}
#[test]
fn slicing() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    assert_eq!(s, "Зд");
    let s = "नमस्ते";
    let mut vec = Vec::new();
    for c in s.chars() {
        vec.push(c);
    }
    assert_eq!(vec.into_iter().collect::<String>(), s);
    let s = "नमस्ते";
    let mut vec = Vec::new();
    for b in s.bytes() {
        vec.push(b);
    }
    assert_eq!(String::from_utf8(vec).expect("foo"), s);
}
