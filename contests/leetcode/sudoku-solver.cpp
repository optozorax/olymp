// https://leetcode.com/problems/sudoku-solver/
struct Set {
    uint16_t inner;
    
    Set() : inner(0) {}
    
    Set(uint16_t inner) : inner(inner) {}
    
    Set unite(Set b) const {
        return Set(this->inner | b.inner);
    }
    
    int size() const {
        int count = 0;
        for (int i = 1; i <= 9; ++i) {
            if ((this->inner & (1 << i)) == 0) 
                count++;
        }
        return count;
    }
    
    int get(int pos) const {
        int count = 0;
        for (int i = 1; i <= 9; ++i) {
            if ((this->inner & (1 << i)) == 0) 
                count++;
            if (count-1 == pos) return i;
        }
        return 0;
    }
    
    void add(int val) {
        this->inner = this->inner | (1 << val);
    }
    
    void remove(int val) {
        this->inner = this->inner & ~(1 << val);
    }
};

int get_k(int i, int j) {
    return (i / 3) * 3 + j / 3;
}

class Solution {
public:
    void solveSudoku(vector<vector<char>>& board) {
        vector<Set> si(9);
        vector<Set> sj(9);
        vector<Set> sk(9);
        
        for (int i = 0; i < 9; ++i) {
            for (int j = 0; j < 9; ++j) {
                if (board[i][j] != '.') {
                    int val = board[i][j] - '0';
                    si[i].add(val);
                    sj[j].add(val);
                    sk[get_k(i, j)].add(val);
                }
            }
        }
        
        recur(board, si, sj, sk);
    }
    
    bool recur(vector<vector<char>>& board, vector<Set>& si, vector<Set>& sj, vector<Set>& sk) {
        int x = 0;
        int y = 0;
        int best = 10;
        for (int i = 0; i < 9; ++i) {
            for (int j = 0; j < 9; ++j) {
                if (board[i][j] == '.') {
                    int size = si[i]
                        .unite(sj[j])
                        .unite(sk[get_k(i, j)])
                        .size();
                    if (size < best) {
                        best = size;
                        x = i;
                        y = j;
                    }
                }
            }
        }
        
        if (best == 10) return true;
        
        Set available = si[x].unite(sj[y]).unite(sk[get_k(x, y)]);
        int size = available.size();
        for (int i = 0; i < size; ++i) {
            int current = available.get(i);
            si[x].add(current);
            sj[y].add(current);
            sk[get_k(x, y)].add(current);
            board[x][y] = current + '0';
            if (recur(board, si, sj, sk)) {
                // cout << i << " " << best << endl;
                return true;
            }
            board[x][y] = '.';
            si[x].remove(current);
            sj[y].remove(current);
            sk[get_k(x, y)].remove(current);
        }
        return false;
    }
};
