use std::io::stdin;

fn main() 
{  

    loop
    {
        let a:i32 = input("\nEnter a : ").trim().parse().expect("failed to parse");

        let b:i32 = input("Enter b : ").trim().parse().expect("failed to parse");

        let state:u8 = input("What do you want to do?\n1) add\n2) subtract\n3) multiply\n4) divide\n0) quit\n(enter number that coresponds to your choice) : ").trim().parse().expect("failed to parse");

        
        match state {
            1 => eprintln!("{} + {} = {}",&a,&b,add(a,b)),
            2 => eprintln!("{} - {} = {}",&a,&b,subtract(a,b)),
            3 => eprintln!("{} * {} = {}",&a,&b,multiply(a,b)),
            4 => eprintln!("{} / {} = {}",&a,&b,divide(a,b)),
            0 => {eprint!("Goodbye!"); break},
            _ => eprintln!("Wrong Input :(")
        };
    }
    
    

}

fn add(a:i32,b:i32) -> i32
{
    a+b
}

fn subtract(a:i32,b:i32) -> i32
{
    a-b
}

fn multiply(a:i32,b:i32) -> i32
{
    a*b
}

fn divide(a:i32,b:i32) -> i32
{
    a/b
}

fn input(msg:&str) -> String
{
    let mut buf = String::new();
    eprint!("{}",msg);
    stdin().read_line(&mut buf).expect("Failed to get input");
    buf
}