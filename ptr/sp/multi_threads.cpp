/**
 * @file:	multi_threads.cpp
 * @author:	Jacob Xie
 * @date:	2023/01/12 15:01:41 Thursday
 * @brief:	std::shared_ptr
 *
 * std::shared_ptr 是一个通过指针来保持一个对象的共享所有权的智能指针。
 **/

#include <chrono>
#include <iostream>
#include <memory>
#include <mutex>
#include <thread>

struct Base
{
  Base() { std::cout << " Base::Base()\n"; }
  ~Base() { std::cout << " Base::~Base()\n"; }
};

struct Derived : public Base
{
  Derived() { std::cout << " Derived::Derived()\n"; }
  ~Derived() { std::cout << " Derived:~Derived()\n"; }
};

void thr(std::shared_ptr<Base> p)
{
  std::this_thread::sleep_for(std::chrono::seconds(1));
  std::shared_ptr<Base> lp = p; // thread-safe 即使共享的 use_count 增加

  {
    static std::mutex io_mutex;
    std::lock_guard<std::mutex> lk(io_mutex);
    std::cout << "local pointer in a thread\n"
              << " lp.get() = "
              << lp.get()
              << ", lp.use_count() = "
              << lp.use_count()
              << std::endl;
  }
}

int main(int argc, char** argv)
{
  std::shared_ptr<Base> p = std::make_shared<Derived>();

  std::cout << "Created a shared Derived (as a pointer to Base)\n"
            << " p.get() = "
            << p.get()
            << ", p.use_count() = "
            << p.use_count()
            << std::endl;

  std::thread t1(thr, p), t2(thr, p), t3(thr, p);
  p.reset(); // 从 main 中释放所有权

  std::cout << "Shared ownership between 3 threads and releasen"
            << "ownership from main:\n"
            << " p.get() = "
            << p.get()
            << ", p.use_count() = "
            << p.use_count()
            << std::endl;

  t1.join();
  t2.join();
  t3.join();

  return 0;
}
