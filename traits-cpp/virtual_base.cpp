#include <cstdio>
#include <iostream>
#include <typeinfo>
using namespace std;

template <typename T>
concept Concept = requires(T a) {
  a.doIt();
};

struct Trait
{
  void doIt()
  {
    cout << "do it" << endl;
  }
};

struct VirtualBase
{
  virtual void doIt()
  {
    cout << "do it virtual" << endl;
  }
};

struct Impl : Trait, VirtualBase
{
  void doIt()
  {
    cout << "do it in impl" << endl;
  }
};

int main()
{
  Impl impl{};
  Trait& trait = impl;
  VirtualBase& virtualBase = impl;
  Concept auto& concept_ = impl;

  cout << typeid(concept_).name() << endl;

  impl.doIt();
  trait.doIt();
  virtualBase.doIt();
  concept_.doIt();

  cout << "sizeof(Impl&): " << sizeof(Impl&) << endl
       << "sizeof(Trait&): " << sizeof(Trait&) << endl
       << "sizeof(VirtualBase&): " << sizeof(VirtualBase&) << endl
       << "sizeof(Concept auto&): " << sizeof(decltype(concept_)) << endl;
}
