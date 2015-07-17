fn for_loop(){
	
	let mut x = 3;
    for i in 0..10{
    	x+=1;
    }
    println!("{}",x);
}

fn while_loop_with_vec(){
	
	let v = vec![1,2,3,4];
    for num in v{
    	if num%2==0 {
    		println!("{}",num);
    	}
    }
}

fn string_example(){
	
	let hello = "hello".to_string();
	let world1 = "world";
	let hello_world1 = hello + world1; //no ampersand needed to add a string and a &str
	
	let hello2  = "hello".to_string();
	let world2 = "world".to_string();
	let hello_world2 = hello2 + &world2; //ampersand needed when adding two strings, to allow second string to deref into &str
}

struct Circle{
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle{
	fn area(&self)->f64{
		std::f64::consts::PI * self.radius * self.radius
	}
}

fn circle_area(){ 
	
	let little_circle = Circle{ x:0.0, y:0.0, radius:2.0}; 
     
    let area = little_circle.area();
    println!("Area of circle with radius {}cm is {}",little_circle.radius,area); 
}

fn factory(&num: &i32) -> Box<Fn(i32) -> i32> {
	
	Box::new(move |x| x + num )
}
	
fn factory_run(){
	
	let num = 5;
	let f = factory(&num);
	let answer = f(1);
	
	if answer == 6{
		println!("success");
	}
}

fn main() {  

    factory_run();
}
