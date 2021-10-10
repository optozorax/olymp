// https://leetcode.com/problems/implement-queue-using-stacks/
class MyQueue {
public:
    stack<int> pushable;
    stack<int> popable;
    int state;
    
    MyQueue() : state(0) {}
    
    void push(int x) {
        if (state == 1) change_state();
        pushable.push(x);
    }
    
    int pop() {
        if (state == 0) change_state();
            
        auto result = popable.top();
        popable.pop();
        return result;
    }
    
    int peek() {
        if (state == 0) change_state();
        
        return popable.top();
    }
    
    bool empty() {
        if (state == 0) {
            return pushable.empty();
        } else {
            return popable.empty();
        }
    }
    
    void change_state() {
        if (state == 0) {
            while (!pushable.empty()) {
                popable.push(pushable.top());
                pushable.pop();
            }
            state = 1;
        } else {
            while (!popable.empty()) {
                pushable.push(popable.top());
                popable.pop();
            }
            state = 0;
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