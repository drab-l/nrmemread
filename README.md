# nrmemread
simple process memory reader

usage:

    nrmemread [Option]

option:

    -p  <pid>: Target process ID, default is self.

syntax:

The following table shows operators and associativity.
The higher up in the table, the higher the precedence.

For syntax details, see https://github.com/drab-l/nrmcalc#readme

|operator|associativity|description|
|-------:|------------:|:----------|
|[@q <i>address</i>]|-|load 8 byte from address as little endian|
|[@d <i>address</i>]|-|load 4 byte from address as little endian|
|[@w <i>address</i>]|-|load 2 byte from address as little endian|
|[@b <i>address</i>]|-|load 1 byte from address as little endian|
|[<i>address</i>]<br>[@ <i>address</i>]|-|load 4byte from address as little endian|
|(<i>explessoin</i>)|-|grouping|
|<i>size</i> @dump <i>address</i><br>@be2,@le2,@be4,@le4,@be8,@le8|-|dump memory as string<br>dump memory each 2/4/8 byte as big/little endian|
|*<br>/|left to right|multiplication<br>division|
|+<br>-|left to right|addition<br>subtraction|
|<<br><<br><=<br>>=|left to right|less than<br>greater than<br>less than or equal<br>greater than or equal|
|==<br>!=|left to right|equal<br>not equal|
|\||left to right|bitwise inclusive OR|
|^|left to right|bitwise exclusive OR|
|&|left to right|bitwise AND|
|=,+=,-=,*=,/=,&=,^=,\|=|right to left|assignment operator|
