#FUNCTIONS

Syntax
------
```python
def main():
  print("Hello")


```
+ starts with 'def'
+ ends with two blank lines


Arguments and Params
--------------------
```python

# Takes two arguments and returns 3 types
def add_numbers(a: int, b: int) -> (int, int, int):
  return a, b, a + b


# Takes two argumnts or just one while the second is defaulted to 3
def add_numners_default(a: int, b: int = 3) -> int:
  return a + b


# Can have any number of arguments
def my_func(*dogs)
  for dog in dogs:
    print(dog)


my_func("Spot", "Fido", "Sam", "Fred", "Bob")

```

