# Day 5: How About a Nice Game of Chess?

You are faced with a security door designed by Easter Bunny
engineers that seem to have acquired most of their security
knowledge by watching
[hacking movies](https://en.wikipedia.org/wiki/Hackers_(film)).

The **eight-character password** for the door is generated one
character at a time by finding the
[MD5](https://en.wikipedia.org/wiki/MD5) hash of some Door ID
(your puzzle input) and an increasing integer index
(starting with 0).

A hash indicates the **next character** in the password if its
[hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal)
representation starts with **five zeroes.** If it
does, the sixth character in the hash is the next character
of the password.

For example, if the Door ID is abc:

* The first index which produces a hash that starts with
  five zeroes is 3231929, which we find by hashing
  abc3231929; the sixth character of the hash, and thus the
  first character of the password, is 1.
* 5017308 produces the next interesting hash, which starts
  with 000008f82..., so the second character of the password
  is 8.
* The third time a hash starts with five zeroes is for
  abc5278568, discovering the character f.

In this example, after continuing this search a total of
eight times, the password is 18f47a30.

Given the actual Door ID, **what is the password?**

Your puzzle answer was 2414bc77.
