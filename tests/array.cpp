#include <stdint.h>
#include <iostream>
#include <string.h>
#include <string_view>

namespace std {
    
template<typename T, size_t Size>
class array {
public:
    constexpr size_t size() const { return Size; }

    T& operator[](size_t index) { return m_data[index]; }
    const T& operator[](size_t index) const { return m_data[index]; }

    T* data() { return m_data; }
    const T* data() const { return m_data; }
private:
    T m_data[Size];
};

template<typename T>
class vector {
public:
    vector() {
        realloc(2);
    }

    void push_back(const T& value) {
        if (m_size >= m_capacity)
            realloc(m_capacity + m_capacity / 2);
        m_data[m_size++] = value;
    }

    const T& operator[](size_t index) const {
        if (index >= m_size) { /* TODO: assert */ }
        return m_data[index];
    }
    T& operator[](size_t index) {
        if (index >= m_size) { /* TODO: assert */ }
        return m_data[index];
    }

    size_t size() const { return m_size; }
private:
    void realloc(size_t new_capacity) {
        T* new_block = new T[new_capacity];
        if (new_capacity < m_size) m_size = new_capacity;
        for (size_t i = 0; i < m_size; i++)
            new_block[i] = m_data[i];
        delete[] m_data;
        m_data = new_block;
        m_capacity = new_capacity;
    }
private:
    T* m_data = nullptr;

    size_t m_size = 0;
    size_t m_capacity = 0;
};

} // namespace my_std

template<typename T>
void print_vector(const std::vector<T>& vector) {
    for (size_t i = 0; i < vector.size(); i++)
        std::cout << vector[i] << std::endl;
}

int main() {
    std::vector<std::string> my_vec;
    my_vec.push_back("this");
    my_vec.push_back("is");
    my_vec.push_back("a");
    my_vec.push_back("test");
    print_vector(my_vec)
}