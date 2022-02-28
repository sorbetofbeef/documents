# FUNDEMENTALS

File Structuring
================

Virtual Environment
-------------------
+ A virtual environment, or venv from now on, is, basically, a mini virtual machine that the python program will use to build and execute its source code
  + I should be using this to build most, if not all, of my python programs and scripts
+ Activate the python environment with:
```bash
source ./venv/bin/activate
```
+ Deactivate with:
```bash
deactivate
```


Datatypes
---------
+ Python is not statically typed
+ strings: str
  + "I am a string."
  + 'I am a string.'
+ integers: int
  + 4
+ booleans: bool
  + True
  + False
+ floats: float 
  + 3.1415927


Scopes
------
+ The place within the source code that a variable can be properly called
+ Scope within a code block takes precedence
+ Function names are used before variable names
