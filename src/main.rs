use std::io::{BufReader, BufWriter, Write};
use bytelines::*; // USING THIS CRATE FOR TESTING (SHOULD BE MORE PERFORMANT THAN STD READ SINCE NO ALLOCATION IS REQUIRED)
use std::fs::File;
use std::str;
use std::net::{SocketAddr, UdpSocket, TcpStream};
use std::env::args;
// FILE IS CONSTATNLY UPDATED
//const FILE: &str = "C:/Steam/steamapps/common/dota 2 beta/game/dota/console.log"; //ABSOLUTE PATH
const FILE: &str = "console.log"; // FOR LOCAL TESTING


//TCP implementacija
fn tcp_send(port: u16) {
    // OPEN FILE AND KEEP IT OPENED FOR READING 
    let file = File::open(FILE).expect("able to open file");    
    let reader = BufReader::new(file);
    let mut lines = reader.byte_lines();

    //OPEN TCP CONNECTION TO LOCAL ADDRESS TAKING THE PORT AS AN ARG LOOP AFTER CONNECTION IS
    //OPENED
    let ip_addr = SocketAddr::from(([127,0,0,1], port));
    let connect = TcpStream::connect(ip_addr);
    match connect{

        Ok(stream) =>{
            //INITIALIZE BUFFER FOR WRITING TO LOWER OVERHEAD CAUSED BY WRITE SYS CALL
            let mut buffer = BufWriter::with_capacity(100, stream);
            //READ ONLY LAST LINE
            loop{
                while let Some(line) = lines.next(){
                    match line {
                        Ok(bytes) => {
                            //CONVERT BYTES TO STRING
                            let string = str::from_utf8(bytes).unwrap();
                            //FILTER ONLY DATA WE ARE INTERESTED IN
                            if string.contains("Camera position"){
                                let test= &string[32..].replace(" ", "_");

                                
                                println!("{}", test);
                                //SEND VIA TCP
                                let msg = test.as_bytes();
                                buffer.write(msg).unwrap();

                            }
                            //FLUSH BUFFER BEFORE DROPING
                            buffer.flush().unwrap();
                        },
                        Err(_) => println!("[ERR!!!] Reading file!")
                    }
                }
            }
        }
        Err(e) => {
            println!("[ERR!!!] Connection: {}", e);
        }
    }
}

fn udp_send(port: u16, target_ip: &str){
    // OPEN FILE AND KEEP IT OPENED FOR READING 
    let file = File::open(FILE).expect("able to open file");    
    let reader = BufReader::new(file);
    let mut lines = reader.byte_lines();

    //OPEN UDP SOCKET ON LOCAL ADDRESS TAKING THE PORT AS AN ARG LOOP AFTER CONNECTION IS
    //OPENED
    let ip_addr = SocketAddr::from(([127,0,0,1], port));
    let socket = UdpSocket::bind(ip_addr).expect("[ERR!!!]can't open socket on that port");
    loop{
        while let Some(line) = lines.next(){
            match line {
                Ok(bytes) => {
                    //CONVERT BYTES TO STRING
                    let string = str::from_utf8(bytes).unwrap();
                    //FILTER ONLY DATA WE ARE INTERESTED IN
                    if string.contains("Camera position"){
                        let test= &string[32..].replace(" ", "_"); 
                                
                        println!("{}", test);
                        //SEND VIA UDP
                        let msg = test.as_bytes();
                        let sending = socket.send_to(&msg, target_ip);
                        match sending{
                            Ok(number_of_bytes) => println!("{:?}", number_of_bytes),
                            Err(e) => println!("[ERR!!!] Sendng: {:?}", e),
                        }
                    }
                    },
                Err(_) => println!("[ERR!!!] Reading file!")
            }
        }
    }
}


fn main(){
    let args : Vec<String> = args().collect();
   if args.len() < 3 {
        println!("Usage: {} <t> <port number>,\n{} <u> <port number> <destination_address>", args[0], args[0]);
   } else{
   let protocol = args[1].as_str();
   match protocol{
       "t" =>{
           tcp_send(args[2].parse::<u16>().unwrap());
       },
       "u" =>{
           udp_send(args[2].parse::<u16>().unwrap(), args[3].as_str());

       },
       _ =>{
           println!("Usage: {} <t> <port number>,\n{} <u> <port number> <destination_address>", args[0], args[0]);
       }
   };
   }
}
