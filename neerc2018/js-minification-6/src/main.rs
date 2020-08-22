use std::io;
use std::collections::{HashSet, HashMap};

// Итератор, который возвращает все имена в лексикографическом порядке: 'a', 'b', ..., 'z', 'aa', 'ab', ... . Возвращает до бесконечности
struct AlphabeticalNamesIterator {
    letters: Vec<u8>,
}

impl AlphabeticalNamesIterator {
    fn new() -> Self {
        Self {
            letters: vec![0],
        }
    }
}

impl Iterator for AlphabeticalNamesIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        const SIZE: u8 = b'z' - b'a';
        let result = self.letters.iter().rev().map(|x| char::from(b'a' + x)).collect::<String>();

        let expand = self.letters
            .iter_mut()
            .scan((), |(), x| {
                *x += 1;
                if *x > SIZE {
                    *x = 0;
                    Some(())
                } else {
                    None
                }
            })
            .count() == self.letters.len();

        if expand {
            self.letters.push(0);
        }

        Some(result)
    }
}

// Данная структура является итератором, и она позволяет считывать три вещи: число, слово, либо зарезервированное слово. Самый большой геморрой возникает из-за заразервированных слов, которые могут быть чем угодно.
struct TokenReader<'a, 'b> {
    input: &'a str,
    reserved: &'b [String],
}

#[derive(Debug)]
struct ReadResult<'a> {
    result: &'a str,
    tail: &'a str,
}

impl<'a, 'b> TokenReader<'a, 'b> {
    fn new(input: &'a str, reserved: &'b [String]) -> TokenReader<'a, 'b> {
        TokenReader {
            input,
            reserved,
        }
    }

    fn read_by_find<F: FnMut(bool, char) -> bool>(&self, mut f: F) -> Option<ReadResult<'a>> {
        let len = self.input.len();
        let i = self.input.char_indices().find(|(i, c)| f((*i + c.len_utf8()) == len, *c))?.0;
        if i == 0 {
            None
        } else {
            Some(ReadResult { result: &self.input[..i], tail: &self.input[i..] })
        }
    }

    fn read_whitespaces(&self) -> Option<ReadResult<'a>> {
        self.read_by_find(|is_end, c| !c.is_ascii_whitespace() || is_end)
    }

    fn read_comment(&self) -> Option<ReadResult<'a>> {
        if self.input.chars().next()? == '#' {
            self.read_by_find(|is_end, c| c == '\n' || is_end)
        } else {
            None
        }
    }

    fn read_word(&self) -> Option<ReadResult<'a>> {
        if Self::is_word_start(self.input.chars().next()?) {
            self.read_by_find(|is_end, c| !Self::is_word_continuation(c) || is_end)
        } else {
            None
        }
    }

    fn read_number(&self) -> Option<ReadResult<'a>> {
        self.read_by_find(|is_end, c| !c.is_ascii_digit() || is_end)
    }

    fn read_string(&self, string: &str) -> Option<ReadResult<'a>> {
        if self.input.starts_with(string) {
            Some(ReadResult { result: &self.input[..string.len()], tail: &self.input[string.len()..] })
        } else {
            None
        }
    }

    fn read_reserved(&self) -> Option<ReadResult<'a>> {
        for i in self.reserved {
            let getted = self.read_string(&i);
            if getted.is_some() {
                return getted;
            }
        }
        None
    }

    fn move_and_get<'c>(&'c mut self, by: &ReadResult<'a>) -> &'a str {
        self.input = by.tail;
        by.result
    }
}

impl TokenReader<'_, '_> {
    fn is_word_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '$' || c == '_'
    }

    fn is_word_continuation(c: char) -> bool {
        Self::is_word_start(c) || c.is_ascii_digit()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Reserved(String),
    Word(String),
    Number(String),
}

impl Iterator for TokenReader<'_, '_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        // Игнорируем все пробелы и комментарии
        let mut repeat_trim = true;
        while repeat_trim {
            repeat_trim = false;
            if let Some(res) = self.read_whitespaces() {
                self.move_and_get(&res);
                repeat_trim = true;
            }
            if let Some(res) = self.read_comment() {
                self.move_and_get(&res);
                repeat_trim = true;
            }
        }

        // Пытаемся считывать все остальные сущности
        let reserved = self.read_reserved();
        let word = self.read_word();
        let number = self.read_number();

        let reserved_word = reserved
            .as_ref()
            .and_then(|reserved| word
                .as_ref()
                .map(|word| reserved.result.len() >= word.result.len())
            );

        let reserved_number = reserved
            .as_ref()
            .and_then(|reserved| number
                .as_ref()
                .map(|number| reserved.result.len() >= number.result.len())
            );

        // При этом смотрим что если у нас считано зарезервированное что-то и оно по длине больше или равна слову или числу, по это полюбому зарезервированная штука
        match (&reserved, &word, &number, reserved_word, reserved_number) {
            (Some(reserved), Some(_), None, Some(true), None) |
            (Some(reserved), None, Some(_), None, Some(true)) |
            (Some(reserved), None, None, None, None) => Some(Token::Reserved(self.move_and_get(reserved).to_string())),

            (Some(_), Some(word), None, Some(false), None) |
            (None, Some(word), None, None, None) => Some(Token::Word(self.move_and_get(word).to_string())),

            (Some(_), None, Some(number), None, Some(false)) |
            (None, None, Some(number), None, None) => Some(Token::Number(self.move_and_get(number).to_string())),

            _ => None,
        }
    }
}

fn my_main(input: impl io::BufRead) -> String {
    // Считывание данных
    let mut lines = input.lines().map(|l| l.unwrap());
    let reserved_count: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut reserved: Vec<String> = lines.next().unwrap().trim().split_whitespace().map(|x| x.to_string()).take(reserved_count).collect();
    let lines_count: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut program: String = lines.take(lines_count).collect::<Vec<_>>().join("\n");

    // Костыль, потому что мне лень искать баг в коде считывания токенов
    program.push_str(" ");

    // Сортируем заразервированные слова по убыванию длины, чтобы при их считывании сначала считывалось самое длинное
    reserved.sort_by(|a, b| b.len().cmp(&a.len()));

    // Считываем токены во входной программе
    let mut tokens = TokenReader::new(&program, &reserved).collect::<Vec<_>>();

    // Получаем последовательный список слов в токенах
    let mut words_set = HashSet::new();
    let mut words = Vec::new();
    for i in &tokens {
        if let Token::Word(word) = i {
            if !words_set.contains(word) {
                words_set.insert(word.clone());    
                words.push(word.clone());
            }
        }
    }

    // Находим новые короткие имена, из алфавитных имён, не забываем фильтровать с ключевыми словами
    let words_map = AlphabeticalNamesIterator::new()
        .filter(|x| reserved.iter().find(|y| x == *y).is_none())
        .zip(words.into_iter())
        .map(|(a, b)| (b, a))
        .collect::<HashMap<_, _>>();

    // Заменяем слова в исходных токенах на новые слова
    for i in &mut tokens {
        if let Token::Word(word) = i {
            *word = words_map[&*word].to_string();
        }
    }

    // Самая паршивая часть. Пытаемся выводить все токены сначала без пробела. Но если случается так, что отсутствие пробела рушит считывание, то добавляем пробел. Нельзя учитывать только один предыдущий символ, потому что могут быть очень хитрые зарезервированные слова. Аналогично нельзя учитывать два итд, поэтому я учитываю тупо все.
    let mut result = String::new();
    for (index, i) in tokens.iter().enumerate() {
        let mut joined = result.clone();
        let token_string = match i {
            Token::Reserved(a) => a,
            Token::Word(a) => a,
            Token::Number(a) => a,
        };
        joined.push_str(token_string);
        joined.push_str(" ");

        if !TokenReader::new(&joined, &reserved).by_ref().eq(tokens.iter().take(index+1).cloned()) {
            result.push(' ');
        }

        result.push_str(token_string);
    }

    result
}

fn main() {
    // Передавать io::stdin нужно чтобы код работал со стандартным вводом-выводом, а для тестов можно свой ввод-вывод замутить
	println!("{}", my_main(io::stdin().lock()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Тесты из условия задачи
        assert_eq!(
            my_main(io::Cursor::new("16\nfun while return var { } ( ) , ; > = + ++ - --\n4\n    {   #    \n #   \n   \n  var ")),
            "{var".to_string()
        );
        assert_eq!(
            my_main(io::Cursor::new("16\nfun while return var { } ( ) , ; > = + ++ - --\n9\nfun fib(num) { # compute fibs\n  var return_value = 1, prev = 0, temp;\n  while (num > 0) {\n    temp = return_value; return_value = return_value + prev;\n    prev = temp;\n    num--;\n  }\n  return return_value;\n}")),
            "fun a(b){var c=1,d=0,e;while(b>0){e=c;c=c+d;d=e;b--;}return c;}".to_string()
        );

        // Нагло стыренные тесты, на которых падала моя прога
        assert_eq!(
            my_main(io::Cursor::new("10\n( ) + ++ : -> >> >>: b c)\n2\n($val1++ + +4 kb) >> :out\nb-> + 10 >>: t # using >>: ")),
            "(a+++ +4c )>> :d b->+10>>:e".to_string()
        );
        assert_eq!(
            my_main(io::Cursor::new("4\n+ ++ +++ ++++\n2\n+ ++ +++ ++++ ++ + +++ ++++ + ++++ ++ +\n+ ++ +++ ++++ ++ + +++ ++++ + ++++ ++ +")),
            "+ ++ +++ ++++++ + +++ +++++ ++++++ + + ++ +++ ++++++ + +++ +++++ ++++++ +".to_string()
            
        );
        assert_eq!(
            my_main(io::Cursor::new("9\n:= >> + out 2+3 3+ 10 +16 17\n1\n9+10")),
            "9+10".to_string()
        );
        assert_eq!(
            my_main(io::Cursor::new("9\n:= >> + out 2+3 3+ 10 +16 17\n7\n first  := 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13    >> out\n second := 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14   >> out\n third  := 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15  >> out\n fourth := 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 >> out\n fifth  := 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 + 17>> out\n sixth  := 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 + 17# 18>> out\n out>>++++++++1 2 3 4 5 6 7 8 9 10")),
            "a:=1+2+ 3 +4+5+6+7+8+9+10+11+12+13>>out b:=2+ 3 +4+5+6+7+8+9+10+11+12+13+14>>out c:=3 +4+5+6+7+8+9+10+11+12+13+14+15>>out d:=4+5+6+7+8+9+10+11+12+13+14+15+ 16>>out e:=5+6+7+8+9+10+11+12+13+14+15+ 16+17>>out f:=6+7+8+9+10+11+12+13+14+15+ 16+17out>>++++++++1 2 3 4 5 6 7 8 9 10".to_string()
        );

    }
}
