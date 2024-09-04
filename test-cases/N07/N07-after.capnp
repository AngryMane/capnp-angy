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
  enum EnumTry @0xefe94bdb1c4ec715 {
    foo @0;
    bar @1;
    baz @2;
    qux @3;
  }
  year @0 :EnumTry  = foo;
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

interface Sample {
    initialize @0 (debug: Bool, test: UInt16 = 0) -> (result: Bool);

    interface Subscriber {
        pushMessage @0 () -> (result: Bool);
    }

    subscribe @1 (subscriber: Subscriber) -> (result: Bool);
}

