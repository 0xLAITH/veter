# Veter
CLI tool to insert strings into templated text objects. 

The input string is taken from your clipboard. The expected format for the input string is (" - " is the delimiter):
>string1 - string2 - string3 

example input string:
```
John - 50 - Red
```

# Usage

1) Make sure you have veter.exe and template.txt in same directory (pre-compiled veter.exe available in veter/target/x86_64-pc-windows-gnu/release/veter.exe).
    
2) Modify template.txt to contain the templates seperated by the "==========\r\n" string, and utilizing the {#} tags to indicate where to input strings:

>example template.txt

```
template1
First string: {1}
Second string: {2}
Third string: {3}
==========
my_intro
My name is: {1}
I am {2} years old
{3} is my favorite color
==========
template3
{1}
{2}
{3}
{4}
```
3) Copy a valid string to your clipboard like the following:
>example input string
```
John - 50 - Red
```

4) Run the executable:
>~#veter [Template_Name]

```
~#veter.exe my_intro
My name is: John
I am 50 years old
Red is my favorite color
```

5) The modified template will be available in your clipboard.
