@0xef41c006a99a86cb;

struct PhoneNumber {
  number @0 :Text;
  type @1 :Type;

  enum Type {
    mobile @0;
    home @1;
    work @2;
  }
}

struct Date {
  enum EnumTest {
    foo @0;
    bar @1;
    baz @2;
    qux @3;
  }
  year @0 :EnumTest = foo;
  month @1 :UInt8;
  day @2 :UInt8;
  target @3 :List(Bool) = [ true, false, false, true ];
}  

struct Person {
  name @0 :Text;
  birthdate @3 :Date;
  email @1 :Text;
  phones @2 :List(PhoneNumber);
  
  union {
    a @4 :Text;
    b @5 :List(PhoneNumber);
  }
}

interface Example @0xda55b331806ed8e2 {
    initialize @0 (debug: Bool, test: UInt16 = 0) -> (result: Bool);

    interface Subscriber {
        pushMessage @0 () -> (result: Bool);
    }

    subscribe @1 (subscriber: Subscriber) -> (result: Bool);
}

