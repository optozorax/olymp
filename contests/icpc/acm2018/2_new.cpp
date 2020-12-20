#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <map>
#include <sstream>
#include <bitset>
#include <set>
#include <list>
#include <memory>

//#undef _DEBUG

using namespace std;

void test(string str) {
	string* str1 = new string(str);
	istringstream* iss = new istringstream(*str1);
	cin.rdbuf(iss->rdbuf());
}

string readFromFile(string fileName) {
	std::ifstream fin(fileName);
	std::string result;
	result.assign(std::istreambuf_iterator<char>(fin), std::istreambuf_iterator<char>());
	fin.close();

	return result;
}

/* Класс оптимального множества. В нём добавление и тестирование элемента происходят за O(1). */
class set_bitset
{
public:
	set_bitset() { }

	// Функции для обхода
	const list<int> getvec(void) const { return m_mas; }
	auto begin(void) { return m_mas.begin(); }
	auto end(void) { return m_mas.end(); }
	auto begin(void) const { return m_mas.begin(); }
	auto end(void) const { return m_mas.end(); }

	bool test(int i) const { return m_bitset.test(i); }
	void add(int a) {
		#ifdef _DEBUG
		if (!test(a))
		#endif
			m_mas.push_back(a);
		#ifdef _DEBUG
		else
			throw exception();
		#endif
		m_bitset.set(a, true);
	}
private:
	bitset<5000> m_bitset;
	list<int> m_mas;
};

/* Класс частично упорядоченного множества, при помощи него можно узнавать какие элементы упорядочены относительно каких, а между какими отношение порядка неизвестно. */
class partially_ordered_set
{
public:
	partially_ordered_set(int elemsCount) : g_order(elemsCount), l_order(elemsCount) {
	}

	void add_relation(int greater, int less) {
		// Добавляет отношение порядка к greater и less, при этом, используя свойство транизитивности, это отношение добавляется и ко всем остальным элементам самым оптимальным образом
		#ifdef _DEBUG
		if (!is_unknown(greater, less))
			throw exception();
		#endif

		auto vec = g_order[less].getvec();
		vec.push_back(less);
		add_to_parents(vec, greater);
	}

	bool is_unknown(int a, int b) {
		// Если отношение порядка между a и b неизвестно, возвращает true
		return !(g_order[a].test(b) || g_order[b].test(a));
	}

	bool is_greater(int greater, int less) {
		// Если greater больше чем less возвращает true, иначе false. Если же отношение неизвестно, создает исключение
		#ifdef _DEBUG
		if (is_unknown(greater, less))
			throw exception();
		else
		#endif
			return g_order[greater].test(less);
	}
private:
	vector<set_bitset> g_order; // Здесь записано какой элемент больше каких других
	vector<set_bitset> l_order; // 

	void add_nonexistent_relation(int greater, int less) {
		#ifdef _DEBUG
		if (!is_unknown(greater, less))
			throw exception();
		else {
		#endif
			g_order[greater].add(less);
			l_order[less].add(greater);
		#ifdef _DEBUG
		}
		#endif
	}

	void add_to_parents(const list<int>& change, int parent) {
		// Рекурсивно добавляет ко всем родителям изменение change, при этом делается это максимально эффективно, чтобы не передавать информацию дважды
		list<int> newchange;
		for (const auto& i : change) 
			if (!g_order[parent].test(i)) {
				newchange.push_back(i);
				add_nonexistent_relation(parent, i);
			}

		if (newchange.size() != 0)
			for (const auto& i : l_order[parent])
				add_to_parents(newchange, i);
	}
};


/* Класс id для строк. Каждой уникальной строке выдается уникальный айди. Необходимо, чтобы использование строк заменить на использование чисел. */
class string_id
{
public:
	string_id() {
		count = 0;
	}

	int get_id(const string& str) {
		auto& pos = id[str];
		if (pos == 0) {
			count++;
			pos = count;
			id_reverse.push_back(str);
			return pos-1;
		} else
			return pos-1;
	}

	const string& get_str(int id) const {
		return id_reverse[id];
	}

	int size(void) const {
		return count;
	}
private:
	map<string, int> id;
	vector<string> id_reverse;
	int count;
};

int main() {
	// Тестирование
	//string testNo = "30";
	//test(readFromFile("testacm2018/2/input" + testNo + ".txt"));
	//cout << "Answer: " << readFromFile("testacm2018/2/output" + testNo + ".txt") << "END ANSWER" << endl << endl;

	// Считывание данных
	string_id id;
	int n;
	cin >> n;
	vector<vector<int>> reps(n, vector<int>());
	int nk;
	for (int i = 0; i < n; ++i) {
		cin >> nk;
		reps[i].resize(nk, 0);
		string str;
		for (int j = 0; j < nk; ++j) {
			cin >> str;
			reps[i][j] = id.get_id(str);
		}
	}

	// Обрабатываем все репетиции
	partially_ordered_set set(id.size());
	for (const auto& rep : reps) {
		// Сначала заносим информацию о порядке из каждой репетиции
		for (int i = 0; i < rep.size(); ++i) {
			for (int j = i+1; j < rep.size(); ++j) {
				if (set.is_unknown(rep[i], rep[j]))
					set.add_relation(rep[i], rep[j]);
			}
		}
	}

	list<int> result;
	bitset<5000> used;
	for (const auto& rep : reps) {
		// Далее производим слияние результата и текущей репетиции
		auto start = result.begin();
		for (const auto& i : rep) {
			if (!used.test(i)) { // Если текущий элемент не добавлен в парад, то ищем куда его вставить
				start_find:
				if (start == result.end()) {
					// Если мы сейчас находимся в конце, то добавляем элемент в конец
					result.insert(start, i);
					used.set(i, true);
				} else {
					if (set.is_unknown(i, *start)) {
						// Если неизвестно кто выше, а кто нет, то добавляем такое отношение согласно лексиграфическому сравнению
						if (id.get_str(i) < id.get_str(*start))
							set.add_relation(i, *start);
						else
							set.add_relation(*start, i);
						goto start_find;
					} else {
						if (set.is_greater(*start, i)) {
							// Если же текущий человек выше нового, продолжаем дальше искать
							start++;
							goto start_find;
						} else {
							// А если меньше, то вставяем сюда
							result.insert(start, i);
							used.set(i, true);
						}
					}
				}
			}
		}
	}

	// Вывод результата
	cout << result.size() << endl;
	for (const auto& i : result)
		cout << id.get_str(i) << endl;
}