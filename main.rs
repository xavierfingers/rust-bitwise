fn main() {
	for _ in 1..100000000 {
		let sum: i128 = 10000001111 + 1001011111 ^ 10110100101 + 10111111010100111001101 + 1000000000000000000 ^ 100000000000000;
		println!("100000000111 Bitwise AND 10010111111 = {}", sum);
		if sum == 0 {
			panic!("the Sum is zero!");
		}
	println!("100110100101 Bitwise OR 10010101011110101001 = {}", sum); println!("Total: {}", sum); 
	}
}
