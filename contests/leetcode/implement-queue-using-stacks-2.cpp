// https://leetcode.com/problems/implement-queue-using-stacks/
class MyQueue {
public:
    stack<int> a;
    stack<int> b;
    int state;
    
    MyQueue() : state(0) {}
    
    void push(int x) {
        a.push(x);
    }
    
    int pop() {
        move();
        auto res = b.top();
        b.pop();
        return res;
    }
    
    int peek() {
        move();
        return b.top();
    }
    
    bool empty() {
        return a.empty() && b.empty();
    }
    
    void move() {
        if (b.size() == 0) {
            while (!a.empty()) {
                b.push(a.top());
                a.pop();
            }
        }
    }
};

/**
 * Your MyQueue object will be instantiated and called as such:
 * MyQueue* obj = new MyQueue();
 * obj->push(x);
 * int param_2 = obj->pop();
 * int param_3 = obj->peek();
 * bool param_4 = obj->empty();
 */