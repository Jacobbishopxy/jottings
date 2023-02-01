#include <iostream>
#include <memory>

struct MyObj
{
  MyObj()
  {
    std::cout << "MyObj constructed" << std::endl;
  }

  ~MyObj()
  {
    std::cout << "MyObj destructed" << std::endl;
  }
};

struct Container : std::enable_shared_from_this<Container> // 注意：公有继承
{
  std::shared_ptr<MyObj> memberObj;

  void CreateMember()
  {
    memberObj = std::make_shared<MyObj>();
  }

  std::shared_ptr<MyObj> get_as_myobj()
  {
    // member 使用一个共享指针的别名
    return std::shared_ptr<MyObj>(shared_from_this(), memberObj.get());
  }
};

void test()
{
  std::shared_ptr<Container> cont = std::make_shared<Container>();
  std::cout << "cont.use_count() = " << cont.use_count() << '\n';
  std::cout << "cont.memberObj.use_count() = " << cont->memberObj.use_count() << '\n';

  std::cout << "Creating member\n\n";
  cont->CreateMember();
  std::cout << "cont.use_count() = " << cont.use_count() << '\n';
  std::cout << "cont.memberObj.use_count() = " << cont->memberObj.use_count() << '\n';

  std::cout << "Creating another shared container\n\n";
  std::shared_ptr<Container> cont2 = cont;
  std::cout << "cont.use_count() = " << cont.use_count() << '\n';
  std::cout << "cont.memberObj.use_count() = " << cont->memberObj.use_count() << '\n';
  std::cout << "cont2.use_count() = " << cont2.use_count() << '\n';
  std::cout << "cont2.memberObj.use_count() = " << cont2->memberObj.use_count() << '\n';

  std::cout << "GetAsMyObj\n\n";
  std::shared_ptr<MyObj> myobj1 = cont->get_as_myobj();
  std::cout << "myobj1.use_count() = " << myobj1.use_count() << '\n';
  std::cout << "cont.use_count() = " << cont.use_count() << '\n';
  std::cout << "cont.memberObj.use_count() = " << cont->memberObj.use_count() << '\n';
  std::cout << "cont2.use_count() = " << cont2.use_count() << '\n';
  std::cout << "cont2.memberObj.use_count() = " << cont2->memberObj.use_count() << '\n';

  std::cout << "copying alias obj\n\n";
  std::shared_ptr<MyObj> myobj2 = myobj1;
  std::cout << "myobj1.use_count() = " << myobj1.use_count() << '\n';
  std::cout << "myobj2.use_count() = " << myobj2.use_count() << '\n';

  std::cout << "cont.use_count() = " << cont.use_count() << '\n';
  std::cout << "cont.memberObj.use_count() = " << cont->memberObj.use_count() << '\n';
  std::cout << "cont2.use_count() = " << cont2.use_count() << '\n';
  std::cout << "cont2.memberObj.use_count() = " << cont2->memberObj.use_count() << '\n';

  std::cout << "Resetting cont2\n\n";
  cont2.reset();
  std::cout << "myobj1.use_count() = " << myobj1.use_count() << '\n';
  std::cout << "myobj2.use_count() = " << myobj2.use_count() << '\n';

  std::cout << "cont.use_count() = " << cont.use_count() << '\n';
  std::cout << "cont.memberObj.use_count() = " << cont->memberObj.use_count() << '\n';

  std::cout << "Resetting myobj2\n\n";
  myobj2.reset();
  std::cout << "myobj1.use_count() = " << myobj1.use_count() << '\n';
  std::cout << "cont.use_count() = " << cont.use_count() << '\n';
  std::cout << "cont.memberObj.use_count() = " << cont->memberObj.use_count() << '\n';

  std::cout << "Resetting cont\n\n";
  cont.reset();
  std::cout << "myobj1.use_count() = " << myobj1.use_count() << '\n';

  std::cout << "cont.use_count() = " << cont.use_count() << '\n';
}

int main(int argc, char** argv)
{
  test();

  return 0;
}
