fn main() {
let v=@
for c in v{
if c==64{
print!("{:?}",v);
}
else{
print!("{}",char::from_u32(c).unwrap());
}
}
}
