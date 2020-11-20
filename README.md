# Заготовка кода на Rust для codeforces.com

```rust
// ---------------------------------------------------------------------------
/* Заготовка для олимпиадных задач. */

/* Документация:
	* Данная заготовка позволит легко тестировать программу путём задания тестов со стандартным вводом и выводом.
	* Для ввода данных используйте только макрос `stdin!()`.
	* Для вывода данных используйте только макросы `print!()`, `println!()`.
	* В тестах используйте функцию `run_main()`, которая получает на вход стандартный ввод, а возвращает стандартный вывод функции.
	* Запускайте программу на ручной тест через `cargo run`.
	* Запускайте программу на локальный тест через `cargo test`.
*/

/* Пояснения за unsafe:
	* В случае запуска на удалённом сервере через `cargo run`, никакого unsafe кода нету. Имеется только макрос `stdin!()`.
	* В случае же локального запуска через `cargo test` допустимо использовать unsafe и всякие трюки чтобы удобнее было тестировать ввод и вывод. 
*/

use std::io::Write;
use std::io::BufRead;

#[cfg(not(test))] #[allow(unused_macros)] macro_rules! stdin { () => { std::io::stdin().lock() }; }

#[cfg(test)] 
mod for_test {
	use super::main;

	pub static mut INPUT: std::io::Cursor<String> = unsafe { std::mem::transmute((1, 1, 1, 1, 1, 1, 1, 1)) };
	static mut INPUT_NOT_INIT: bool = true;
	pub static mut OUTPUT: Vec<u8> = Vec::new();

	pub fn run_main(input_data: &str) -> String {
		let new_input = std::io::Cursor::new(input_data.to_string());
		if unsafe { INPUT_NOT_INIT } {
			// Так нужно, потому что вызывается drop у некорректного значения
			std::mem::forget(std::mem::replace(&mut *unsafe { &mut INPUT }, new_input));
			*unsafe { &mut INPUT_NOT_INIT } = false;
		} else {
			// А тут уже можно вызывать drop нормально
			// Drop может быть нужно вызывать, чтобы пользователь мог тестировать очень большие тесты и не упасть с Out Of Memory
			*unsafe { &mut INPUT } = new_input;
		}
		*unsafe { &mut OUTPUT } = Vec::new();
		main();
		String::from_utf8(unsafe { &OUTPUT }.clone()).expect("failed to convert output to utf8")
	}

	#[macro_export] macro_rules! stdin { () => { unsafe { &mut for_test::INPUT } }; }

	#[macro_export] macro_rules! print { ($($arg:tt)*) => { write!(unsafe { &mut OUTPUT }, $($arg)*).unwrap() }; }
	#[macro_export] macro_rules! println { ($($arg:tt)*) => { writeln!(unsafe { &mut OUTPUT }, $($arg)*).unwrap() }; }
}

#[cfg(test)] 
use for_test::*;

// ---------------------------------------------------------------------------

fn main() {
	stdin!()
		.lines()
		.map(|l| l.unwrap())
		.map(|l| l.parse::<i64>().unwrap())
		.map(|x| x+1)
		.for_each(|x| println!("number is {}", x));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(run_main("16\n5\n5\n"), "number is 17\nnumber is 6\nnumber is 6\n");
		assert_eq!(run_main("6\n3\n2\n"), "number is 7\nnumber is 4\nnumber is 3\n");
	}
}
```