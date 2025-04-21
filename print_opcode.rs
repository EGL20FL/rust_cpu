fn main(){
let opcode = 0xB123;
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >>  8) as u8;
        let y = ((opcode & 0x00F0) >>  4) as u8;
        let d = ((opcode & 0x000F) >>  0) as u8;
let nnn = opcode & 0x0FFF;
println!("{:#08x}",c);
println!("{:#08x}",x);
println!("{:#08x}",y);
println!("{:#08x}",d);
println!("{:#08x}",nnn);
}
