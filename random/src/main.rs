use std::collections::HashMap;

fn greet(n: i32) -> String {
  match n {
    0 => "User 0".to_string(),
    1 => "User 1".to_string(),
    2 => "User 2".to_string(),
    3 => "User 3".to_string(),
    4 => "User 4".to_string(),
    5..=10 => "User 5..10".to_string(),
    _other if n % n == 0 => format!("{} is divisible by {}", n, n).to_string(),
    other => format!("User {}", other).to_string()
  }
}

fn for_fn() {
  println!("\n-- For 0..4 --");
  for i in 0..=5 {
    let a = greet(i);
    let b = String::from(" are dead!");
    let usr = a + &b;
    println!("Name: {}", usr);
  }
}

fn borrow() {
  println!("\n-- Borrow test --");
  let mut a = String::from("A");
  {
    let b = &mut a;
    println!("B: {}", b);
    b.push_str("B");
  }
  println!("A: {}", a);
}

fn array() {
  println!("\n-- Array --");
  let mut projects = [
    "Kilate", "KCStd", "Robok-Engine", "Sketchware.pro"
  ];
  println!("Projects count: {}", projects.len());
  println!("DISPLAY ALL ELEMENTS OF ARRAY WITH FOR");
  for project in &projects {
    println!("{}", project);
  }
  projects[1] = "KCStandard";
  println!("DISPLAY ALL ELEMENTS OF ARRAY DIRECTLY");
  println!("{:?}", projects);
}

fn vec() {
  println!("\n-- Vector --");
  let mut bikes = vec!["BMW S1000RR"];
  println!("Bikes count: {}", bikes.len());
  bikes.push("Yamaha R15");
  println!("Bikes count: {}", bikes.len());
  println!("DISPLAY ALL ELEMENTS OF VEC WITH FOR");
  for bike in &bikes {
    println!("{}", bike);
  }
  bikes.pop(); // remove last element
  bikes.insert(0, "Yamaha R3");
  bikes.remove(1);
  println!("Bikes count: {}", bikes.len());
  println!("DISPLAY ALL ELEMENTS OF VEC DIRECTLY");
  println!("{:?}", bikes);
}

fn tuple() {
  println!("\n-- Tuple --");
  let ppl = ("Aquiles", 14, true);
  println!("Name: {}", ppl.0);
  println!("Age: {}", ppl.1);
  println!("Is dev: {}", ppl.2);

  let (name, age, is_dev) = ppl;
  println!("Name: {}", name);
  println!("Age: {}", age);
  println!("Is dev: {}", is_dev);

  let aquiles = get_aquiles_tuple();
  println!("Name: {}", aquiles.0);
  println!("Age: {}", aquiles.1);
  println!("Is dev: {}", aquiles.2);
}

fn get_aquiles_tuple() -> (String, i8, bool) {
  (String::from("Aquiles"), 14, true)
}

fn hashmap() {
  println!("\n-- HashMap --");
  let mut funcs = HashMap::new();
  funcs.insert(
    "println".to_string(),
    "print a str in stdout".to_string()
  );
  if !funcs.contains_key("write") {
    println!("write function aren't here.");
  }
  funcs.insert(
    "draw".to_string(),
    "draws in screen".to_string()
  );

  funcs.remove("println");
  if !funcs.contains_key("println") {
    println!("println function aren't here.");
  }

  // funcs.remove("draw");

  if let Some(city) = funcs.get("draw") {
    println!("draw::desc => {}", city);
  } else {
    println!("draw function aren't here.");
  }

  for (name, desc) in &funcs {
    println!("Name: {}", name);
    println!("Desc: {}", desc);
  }
}

#[derive(Clone, Copy)]
enum Brand {
  Bmw,
  Yamaha,
  Honda,
  HarleyDavison
}

enum LoadingStatus {
  Waiting(String),
  Error(String),
  Done(String)
}

fn enums() {
  println!("\n-- Enums --");
  {
    let brand = brand_tostr(Brand::Bmw);
    println!("Brand 1 => {}", brand);
  }
  {
    let brand = brand_tostr(Brand::Yamaha);
    println!("Brand 2 => {}", brand);
  }
  {
    let brand = brand_tostr(Brand::Honda);
    println!("Brand 3 => {}", brand);
  }
  {
    let brand = brand_tostr(Brand::HarleyDavison);
    println!("Brand 4 => {}", brand);
  }
  let r1 = LoadingStatus::Waiting(String::from("The drivers"));
  let r2 = LoadingStatus::Error(String::from("Wrong drivers"));
  let r3 = LoadingStatus::Done(String::from("All drivers setup"));
  println!("r1 => {}", loading_status_tostr(r1));
  println!("r2 => {}", loading_status_tostr(r2));
  println!("r3 => {}", loading_status_tostr(r3));
}

fn brand_tostr(brand: Brand) -> String {
  match brand {
    Brand::Bmw => String::from("Bmw"),
    Brand::Yamaha => String::from("Yamaha"),
    Brand::Honda => String::from("Honda"),
    Brand::HarleyDavison => String::from("HarleyDavison")
  }
}

fn loading_status_tostr(ls: LoadingStatus) -> String {
  match ls {
    LoadingStatus::Waiting(message) => String::from(format!("Waiting {}...", message)),
    LoadingStatus::Error(message) => String::from(format!("Error: {}.", message)),
    LoadingStatus::Done(message) => String::from(format!("Done {}!", message)),
  }
}

#[derive(Clone)]
struct Bike {
  name: String,
  brand: Brand,
  year: u32,
  price: f64,
  sport: bool,
  cc: u64
}

fn structs() {
  println!("\n-- Structs --");
  let my_bike = Bike {
    name: String::from("s1000rr"),
    brand: Brand::Bmw,
    year: 2024,
    price: 150.000,
    sport: true,
    cc: 1000
  };

  println!("Name: {}", my_bike.name);
  println!("Brand: {}", brand_tostr(my_bike.brand));
  println!("Year: {}", my_bike.year);
  println!("Price: {}", my_bike.price);
  println!("Sport: {}", my_bike.sport);
  println!("cc: {}", my_bike.cc);

  let cg150 = make_2025_basic_bike(String::from("cg 150"), Brand::Honda, 10.000, 150);
  println!("\nName: {}", cg150.name);
  println!("Brand: {}", brand_tostr(cg150.brand));
  println!("Year: {}", cg150.year);
  println!("Price: {}", cg150.price);
  println!("Sport: {}", cg150.sport);
  println!("cc: {}", cg150.cc);

  let titan150 = Bike {
    name: String::from("Titan 150"),
    price: 5.000,
    ..cg150
  };
  println!("\nName: {}", titan150.name);
  println!("Brand: {}", brand_tostr(titan150.brand));
  println!("Year: {}", titan150.year);
  println!("Price: {}", titan150.price);
  println!("Sport: {}", titan150.sport);
  println!("cc: {}", titan150.cc);
}

fn make_2025_basic_bike(name: String, brand: Brand, price: f64, cc: u64) -> Bike {
  Bike {
    name,
    brand,
    price,
    cc,
    year: 2025,
    sport: false,
  }
}

fn main() {
  for_fn();
  borrow();
  array();
  vec();
  tuple();
  hashmap();
  enums();
  structs();
}