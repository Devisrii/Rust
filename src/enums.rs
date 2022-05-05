enum Payment {
    Cash(f32), 
    CreditCard(String, usize),
    DebitCard(DebitData),
    Crypto{acc_number: String, amount: f32},
}

struct DebitData {
 card_number: String,
 amount: f32
}

pub fn run() {
    let some_payment = Payment::Cash(100.);
		process_payment(some_payment);
		let cc_payment = Payment::CreditCard("cc_num".to_string(), 250);
		process_payment(cc_payment);
		let debit_payment = Payment::DebitCard(DebitData {
			card_number: "DebitNum".to_string(),
			amount: 450.95
		});
		process_payment(debit_payment);
		let crypto_payment = Payment::Crypto {
			acc_number: "cryptoNum".to_string(),
			amount: 200.0
		};
		process_payment(crypto_payment);
}

fn process_payment(some_payment:Payment) {
	match some_payment {
			Payment::Cash(amnt) => {
					println!("pay with cah: {}", amnt);
			}
			Payment::CreditCard(some_string, some_f32) => {
					println!("pay with creditCard: cardnum: {} amnt: {}", some_string, some_f32);
			}
			Payment::DebitCard(data) => {
					println!("pay with debitCard: {} {}", data.card_number, data.amount)
			}
			Payment::Crypto { acc_number, amount } => {
					println!("crypto_payment: {} {}", acc_number, amount)
			}
	}
}