import { Foo } from "@ffi/example";

{
  using foo = new Foo();
  foo.bar();
  // foo is disposed here...
}