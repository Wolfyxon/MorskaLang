# CobaiLang
> [!NOTE]
> This project is on a experimental stage. It may not be completed anytime soon (if ever).

Cobai is a high level statically typed object oriented programming language that aims for simplicity and strong typing.

Inspired by GDScript, Lua and Java.

## Examples
[Longer examples](https://github.com/Wolfyxon/cobai-lang/tree/main/examples)

```js
// Using := automatically assigns the type that input() returns to 'name'
var name := input("What's your name? ")
print("Hello", name)
```
```rs
public class LifeForm:
  public var age: int = 0
end

public class Human(name: String):
  public var name: String

  public func new(name: String) self:
    self.name = name
  end

  public func getGreeting(someone: Human) String:
    return "Hi " + someone.name
  end
end

var h1 := Human("Bob")
var h2 := Human("Alice")

print(h1.getGreeting(h2))
```
## Why 'Cobai'
*Cobai* means *guinea pig* in Romanian and I think gunea pigs are cute so why not use that.