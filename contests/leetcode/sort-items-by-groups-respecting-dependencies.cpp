// https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies/
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

// deps работает по индексам
// в nums не обязательно индексы
optional<vector<int>> sort_with_dependencies(vector<int>& nums, vector<vector<int>>& deps) {
    vector<vector<int>> rev(nums.size());
    for (int i = 0; i < deps.size(); ++i) {
        for (auto& j : deps[i]) {
            rev[j].push_back(i);
        }
    }

    vector<int> deps_count;
    for (auto& i : deps) {
        deps_count.push_back(i.size());
    }

    deque<int> to_visit;
    for (int i = 0; i < deps.size(); ++i) {
        if (deps[i].empty()) to_visit.push_back(i); 
    }
    
    // DS D2(nums) D3(deps) DL
    // DS D3(rev) D2(deps_count) D2(to_visit) DL

    vector<int> result;
    while (!to_visit.empty()) {
        auto current = to_visit.front(); to_visit.pop_front();
        result.push_back(nums[current]);
        for (auto& i : rev[current]) {
            deps_count[i]--;
            if (deps_count[i] == 0) {
                to_visit.push_back(i);
            }
        }
    }
    
    // DS D2(result) DL
    
    if (result.size() != nums.size()) {
        return nullopt;
    } else {
        return result;
    }
}

class Solution {
public:
    vector<int> sortItems(int n, int m, vector<int>& group, vector<vector<int>>& beforeItems) {
        vector<vector<int>> group_elements(m); // все элементы группы
        vector<int> no_group_elements; // все элементы без группы
        vector<int> number_in_group(n); // определить позицию в своей группе по числу
        for (int i = 0; i < n; ++i) {
            if (group[i] != -1) {
                number_in_group[i] = group_elements[group[i]].size();
                group_elements[group[i]].push_back(i);
            } else {
                number_in_group[i] = no_group_elements.size();
                no_group_elements.push_back(i);
            }
        }
        
        int g1 = no_group_elements.size();

        vector<vector<int>> global_deps(g1 + m);
        vector<int> global_nums = no_group_elements;
        for (int i = 0; i < m; ++i) global_nums.push_back(-(i + 1));
        
        // записываем глобальные зависимости для свободных элементов
        for (int i = 0; i < n; ++i) {
            for (auto& j : beforeItems[i]) {
                int pos1 = group[i] == -1 ? number_in_group[i] : g1 + group[i];
                int pos2 = group[j] == -1 ? number_in_group[j] : g1 + group[j];
                if (pos1 != pos2) global_deps[pos1].push_back(pos2);
            }
        }
        
        // DS D(n) D(m) D2(group) D3(beforeItems) DL
        // DS D3(group_elements) D2(no_group_elements) D2(number_in_group) DL
        // DS D2(global_nums) D3(global_deps) DL

        // сортируем каждую группу локально
        for (int mi = 0; mi < m; ++mi) {
            auto& nums = group_elements[mi];
            vector<vector<int>> local_deps(nums.size());
            for (auto& i : nums) {
                for (auto& j : beforeItems[i]) {
                    if (group[j] == group[i]) {
                        local_deps[number_in_group[i]].push_back(number_in_group[j]);
                    }
                }
            }
            auto res = sort_with_dependencies(nums, local_deps);
            if (res) {
                group_elements[mi] = *res;
            } else {
                return {};
            }
        }

        auto res = sort_with_dependencies(global_nums, global_deps);
        if (res) {
            vector<int> result;
            for (auto& i : *res) {
                if (i < 0) {
                    for (auto& j : group_elements[-i - 1]) {
                        result.push_back(j);
                    }
                } else {
                    result.push_back(i);
                }
            }
            return result;
        } else {
            return {};
        }
    }
};

/* 

алгоритм распределения чисел с учётом зависимостей
    получает:
        массив чисел
        массив зависимостей для каждого числа
    возвращает: 
        либо ответ что невозможно
        либо перераспределённый ответ

    создаём массив обратных индексов
    создаём массив количества зависимостей для каждого элемента
    перебираем все позиции и складываем в to_visit все те, у кого нет зависимостей
    перебираем массив to_visit
        складываем текущий элемент в ответ
        перебираем все обратные индексы, делаем --
            для каждого элемента у которого 0, добавляем его в to_visit

    если размер ответа не равен размеру исходного то ответа не существует


перебираем каждую группу
    перебираем каждый элемент этой группы
        перебираем все его зависимости
            если текущая зависимость относится к свободному числу, то записываем в зависимости группы
            если текущая зависимость относится к числу из другой группы, записываем чтобы та группа была раньше текущей
            если текущая зависимость относится к числу из текущей группы, то записываем это отдельно
    распределяем текущую группу с учётом всех элементов внутри

распределяем все свободные числа и представляем группы как одно число в массив
распределяем это согласно зависимостям
записываем каждое свободное число и каждую группу в массив

 */