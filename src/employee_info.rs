#[derive(Debug)]
pub struct EmployeeInfo {
	pub name: String,
	pub age: usize,
	pub salary: usize,
	pub call_count: usize,
}

impl EmployeeInfo {
	pub fn name(name: String) -> Self {
		Self{
				name: name,
				age: 25,
				salary: 15000,
				call_count: 0,
		}
	}
	pub fn age(self, other: EmployeeInfo) -> Self {
		if self.age > other.age {
			Self {
				name: self.name,
				age: self.age,
				salary: self.salary,
				call_count: self.call_count
			}
		}
		else {
			Self {
				name: other.name,
				age: other.age,
				salary: other.salary,
				call_count: other.call_count
			}
		}
	}

}