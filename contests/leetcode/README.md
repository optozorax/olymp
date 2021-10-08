# LeetCode

## Заготовки

Эти куски кода я вставляю в задачи чтобы упростить процесс решения и не писать одно и то же по тысяче раз.

### Бинарный поиск

`F` должно иметь сигнатуру `Fn(int) -> bool`.

Ищет в области `[a, b)`.

Возвращает `-1` если не нашёл (быть аккуратным, если начало области отрицательное).

```c++
template<typename F>
int binary_search(int a, int b, F good) {
    int old_b = b;
    if (b-a == 0) return -1;
    if (good(a)) return a;
    if (!good(b-1)) return -1;
    while (b-a != 1) {
        int c = (a+b)/2;
        if (good(c)) b = c; else a = c;
    }
    if (good(a)) return a;
    else if (b < old_b && good(b)) return b;
    else return -1;
}
```

---

Возвращает `b`, если результат не найден.

```c++
template<typename F>
int binary_search2(int a, int b, F good) {
	int result = binary_search(a, b, good);
	if (result == -1) return b;
	return result;
}
```

### Односвязные списки

```c++
void printList(ListNode* head) {
    for (auto i = head; i != nullptr; i = i->next) {
        cout << i->val << " ";
    }
    cout << endl;
}
    
ListNode* offset(ListNode* a, int count) {
    if (a == nullptr) return nullptr;
    for (int i = 0; i < count; ++i) {
        if (a->next != nullptr) 
            a = a->next;
    }
    return a;
}

ListNode* offset_null(ListNode* a, int count) {
    if (a == nullptr) return nullptr;
    for (int i = 0; i < count; ++i) {
        if (a != nullptr) 
            a = a->next;
    }
    return a;
}

int listSize(ListNode* head) {
    int n = 0;
    for (auto i = head; i != nullptr; i = i->next) {
        n++;
    }
    return n;
}

ListNode* reverseList(ListNode* head) {
    auto i = head;
    ListNode* previous = nullptr;
    while (i != nullptr) {
        head = i;
        auto next = i->next;
        i->next = previous;
        previous = i;
        i = next;
    }
    return head;
}

ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
    if (l1 == nullptr) return l2;
    if (l2 == nullptr) return l1;
    
    auto a = l1;
    auto b = l2;
    if (a->val > b->val) swap(a, b);
    
    auto result = a;
    
    while (a != nullptr && b != nullptr) {
        // we guarantee that a->val <= b->val
        if (a->next == nullptr || a->next->val > b->val) {
            auto next = a->next;
            a->next = b;
            b = next;
        } else {
            a = a->next;
        }
    }
    
    return result;
}
```

### Дебаг

```c++
template <class T> char *debug_arr(T begin, T end) {
    for (; begin != end; ++begin) {
        cout << *begin << " ";
    }
    return "";
}
template <class T> char *debug_arr2(T begin, T end) {
    for (; begin != end; ++begin) {
        cout << "{";
        debug_arr((*begin).begin(), (*begin).end());
        cout << "} ";
    }
    return "";
}
#define DS cout << "? ";
#define D(A) cout << #A << ": " << A << ", ";
#define D2(A) cout << #A << ": " << debug_arr(A.begin(), A.end()) << ", ";
#define D3(A) cout << #A << ": " << debug_arr2(A.begin(), A.end()) << ", ";
#define DL cout << endl;
```

## Patterns

В первую очередь я решил прорешать все задачки из списка [LeetCode Patterns](https://seanprashad.com/leetcode-patterns/), где собрано 170 задач, покрывающие основные темы которые могут встретиться в задачах на собеседованиях.

Во время прорешивания я понял что LeetCode довольно неплохой сайт для того чтобы набраться базы олимпиадного программирования. Например, мне казалось что динамическое программирование может быть только в сложных задачах, но LeetCode показал что оно может быть и в простых задачах.

Так же здесь покрываются такие задачи, которые принципиально не могут встретиться на моём любимом Codeforces, например задачи на односвязные списки. Что интересно, сначала я думал что это простое задротство только ради разработчиков алгоритмов для односвязных списков, но потом задача [Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/) (Approach 7) показала, что алгоритмы на односвязных списках (а именно [эта](https://leetcode.com/problems/linked-list-cycle/)) могут быть составной частью других алгоритмов, где ни слова не говорится об списках.

## Почему это не идеальный сайт

Я не люблю LeetCode из-за того как он оценивает скорость работы программы и её потребление памяти: это абсолютный рандом, и он не проверяет смог ли алгоритм уложиться в нужный предел по времени. Явный пример — [Median of two sorted arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/), в этой задаче требуется сделать сложность `O(log(m + n))`, но решение не проверяется на массивах большого размера, где была бы заметна разница между `O(m + n)` и `O(log(m + n))`. Я локально проверил на больших массивах и обнаружил существенную разницу во времени.

Ну и ещё меня бесит что люди в секции Discussions пишут время по замерам LeetCode, но не пишут сложность `O(...)`.