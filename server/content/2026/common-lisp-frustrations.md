# Common Lisp Frustrations

I've recently added a public API to Wikistocks. Along with that API, I decided
I should make some templates for each programming language I know as a quick
scaffold to help people get started if they want to make bots for Wikistocks.

I've completed templates for:
[python](https://codeberg.org/onelikeandidie/ws-python-template),
[rust](https://codeberg.org/onelikeandidie/ws-rust-template),
[go](https://codeberg.org/onelikeandidie/ws-go-template) and
[typescript/bun](https://codeberg.org/onelikeandidie/ws-js-bun-template).

It's been fun learning go and relearning python so I wanted to try and make a
template in a language I am not at all familiar with; Common Lisp.

Common Lisp is a Lisp programming language, it has a very unfamiliar syntax to
me but although it looked a little foreign, I was ready to try my hand at
learning it. I found a lot of users recommending
[The Common Lisp Cookbook](https://lispcookbook.github.io/cl-cookbook/) so
that's what I used as a guide.

## Getting a Common Lisp compiler

One of the requirements I set for my bot templates is that I provide a
`shell.nix` with each template so that any user that has the nix package
manager can setup a development enviroment with all dependencies required
by using just `nix-shell shell.nix`. I normally make these templates on the go
with my laptop, this requirement makes it easier for me to clean up the
dependencies after I finish making the bot as my laptop only has 240GB of
storage.

This requirement is so that I don't fill my laptop with leftover files from all
the tooling for each language I use to make these templates.

The cookbook recommends the SBCL (Steel Bank Common Lisp) compiler which is
available on `nix` as `pkgs.sbcl`. Additionally, I can also get any libraries
like below:

````nix
{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  packages = with pkgs.buildPackages; [
    # Use SBCL with custom dependencies
    (sbcl.withPackages (sbcl-pkgs: [
      # Library for HTTP requests
      sbcl-pkgs.dexador
      # Library to use the .env file for environment variables
      sbcl-pkgs.cl-dotenv
      # Library to parse cookies from http requests from dexador
      sbcl-pkgs.cl-cookie
    ]))
  ];
}
````

Great, now I could get started with lisp.

## Lisp is kinda weird

SBCL provides what they call a REPL (Read Eval Print Loop), it's basically a
prompt that lets you test out your program, debug variables and reload your
program in real time.

````
❯ sbcl
This is SBCL 2.5.7, an implementation of ANSI Common Lisp.
More information about SBCL is available at <http://www.sbcl.org/>.

SBCL is free software, provided as is, with absolutely no warranty.
It is mostly in the public domain; some portions are provided under
BSD-style licenses.  See the CREDITS and COPYING files in the
distribution for more information.
* (+ 6 7)

13
* (quit)
❯
````

It's actually a really good way to learn the language and the syntax. The
syntax is a little peculiar, you know how most compilers will interpret your
keywords into tokens and will try to understand which keywords make an
expression? Lets look at C here:

````c
#include <stdio.h>
int main() {
  int a = 6;
  int b = 7;
  int c = a + b;
  int d = a * c + b;
  printf("Result: %d", d);
  return 0;
}
// Result: 87
````

The C compiler will take each character in your program and make them into
expressions automatically. Line 6, `int d = a * c + b;` has 3 expressions,
the assignment operation, the multiplication operation and the addition
operation. And using operator precedence, it knows that it should multiply
`a * c` before adding `b` and only then it will assign the result to `d`.

Common Lisp requires you to specify these statements instead, so let's look
at a similar example in lisp.

````common-lisp
(let ((a 6))
  (let ((b 7))
    (let ((c (+ a b)))
      (let ((d (+ (* a c) b)))
        (format t "Result ~D~%" d)))))
````

This is the result in the SBCL REPL:

````
# ...
* (let ((a 6))
  (let ((b 7))
    (let ((c (+ a b)))
      (let ((d (+ (* a c) b)))
        (format t "Result ~D~%" d)))))
Result 85
NIL
*
````

These 2 programs are equivalent in C and Common Lisp. The lisp syntax requires
you as the developer to implicitly spell out when each statement starts and
ends with the round parentisis. Also, the `let` keyword is how you define
scoped variables, anything inside the "form" inside those parentisis can access
those variables. If you try to access `a` outside these forms, it is not
defined;

````
* (format t "A: ~D~%" a)

debugger invoked on a UNBOUND-VARIABLE @1200B45088 in thread
#<THREAD tid=65854 "main thread" RUNNING {12003D0143}>:
  The variable A is unbound.

Type HELP for debugger help, or (SB-EXT:EXIT) to exit from SBCL.

restarts (invokable by number or by possibly-abbreviated name):
  0: [CONTINUE   ] Retry using A.
  1: [USE-VALUE  ] Use specified value.
  2: [STORE-VALUE] Set specified value and use it.
  3: [ABORT      ] Exit debugger, returning to top level.

(SB-INT:SIMPLE-EVAL-IN-LEXENV A #<NULL-LEXENV>)
0]
````

SBCL will automatically go into debugger mode when something crashes, which is
actually really cool. You can replace the variable with another number, write
another expression or do anything you want really. It's an interesting concept.
To create a global variable, you need to use the `defvar` keyword like so:

````common-lisp
(defvar a 6)
;; or if you want a const:
(defconstant b 7)
;; or another way for some reason
(defparameter c (+ a b))
````

These variables can be passed into functions and other expressions anywhere
after they are defined. SBCL seems to be compiling from the top of the file to
the bottom sequentially.

## So what's wrong?

Back to using lisp. My code editor is [helix editor](https://helix-editor.com),
it's a [kakoune](https://kakoune.org) inspired editor with pretty sensible
built-in configuration for a bunch of languages. Even came with syntax
highlighting for Lisp, but it was missing a language server configuration for
lisp. I said to myself "that's ok, you've made a bot template in assembly
without a language server. lisp can't be that hard" so I stated programming.
It actually went well at the beginning, I learnt that SBCL includes a
package manager called ASDF that is integrated with nix (thanks nix devs) so
that I can use nix to manage dependencies. I loaded `cl-dotenv` like so:

````common-lisp
;; load the ASDF extension
(load (sb-ext:posix-getenv "ASDF"))
;; load the cl-dotenv package
(asdf:load-system 'cl-dotenv)

;; load the .env file
(.env:load-env (merge-pathnames ".env"))

;; get the WS_HOST variable from the environment
(defvar ws_host (uiop:getenv "WS_HOST")) 
;; print it out
(format t "WS_HOST: ~a~%" ws_host)
````

Then I loaded this into the REPL using `sbcl --load src/ws-bot.lisp` after
adding `WS_HOST=6769420` to my `.env` file and everything worked as
expected.

````
❯ sbcl --load src/dotenv_test.lisp
This is SBCL 2.5.7, an implementation of ANSI Common Lisp.
More information about SBCL is available at <http://www.sbcl.org/>.

SBCL is free software, provided as is, with absolutely no warranty.
It is mostly in the public domain; some portions are provided under
BSD-style licenses.  See the CREDITS and COPYING files in the
distribution for more information.
WS_HOST: 6769420
*
````

Epic, we have it working baby. From this I know 3 things:

- I have a package manager
- I can load environment variables
- This is just another language

Ok so now I just had to make the bot template. To talk to Wikistocks, a bot
needs two things: a csrf token and an authentication token. The csrf token is
required to obtain the authentication token. To obtain the csrf token, I've
created an endpoint at `https://wikistocks.pires.gg/sanctum/csrf-cookie` that
will attach the token to the `SET-COOKIE` in the response. A bot needs to
make a request to that path, then extract the token from the response headers.

So the first thing to do when the bot turns on is obtain that token and store
it till the end of the session.

````common-lisp
(load (sb-ext:posix-getenv "ASDF"))
(asdf:load-system 'cl-dotenv)
;; additionally load cookie and dexator
;; these are used to extract cookies from the request and to make requests
(asdf:load-system 'cl-cookie)
(asdf:load-system 'dexador)

(.env:load-env (merge-pathnames ".env"))
(defvar ws_host (uiop:getenv "WS_HOST"))

;; define function to request token
(defun requestCsrfToken ()
  "Returns the CSRF token from the wikistocks host"
  ;; define a cookie jar to put the cookies
  (let ((cookiejar (cl-cookie:make-cookie-jar)))
    ;; request and put cookies in the cookie jar variable
    ((dex:get (concatenate ws_host "/sanctum/csrf-cookie") :cookie-jar cookiejar))))
  ;; TODO: figure out the rest

(defvar xtoken (requestCsrfToken))
````

If you thought this has been smooth sailing so far, this is where I hit a road
block. Loading this into the SBCL I get a garbled mess of errors. Honestly, I
previously thought JavaScript errors were the worst but now that I tried Common
Lisp, I can see how it could be worse.

````
❯ sbcl --load src/ws-bot.lisp
This is SBCL 2.5.7, an implementation of ANSI Common Lisp.
More information about SBCL is available at <http://www.sbcl.org/>.

SBCL is free software, provided as is, with absolutely no warranty.
It is mostly in the public domain; some portions are provided under
BSD-style licenses.  See the CREDITS and COPYING files in the
distribution for more information.

; file: /home/pedro/wikistocks-api-templates/ws-common-lisp-template/src/ws-bot.lisp
; in: DEFUN REQUESTCSRFTOKEN
;     ((DEXADOR:GET (CONCATENATE WS_HOST "/sanctum/csrf-cookie") :COOKIE-JAR
;                   COOKIEJAR))
;
; caught ERROR:
;   illegal function call

;     (COOKIEJAR (CL-COOKIE:MAKE-COOKIE-JAR))
;
; caught STYLE-WARNING:
;   The variable COOKIEJAR is defined but never used.
;
; compilation unit finished
;   caught 1 ERROR condition
;   caught 1 STYLE-WARNING condition
While evaluating the form starting at line 25, column 0
  of #P"/home/pedro/wikistocks-api-templates/ws-common-lisp-template/src/ws-bot.lisp":

debugger invoked on a SB-INT:COMPILED-PROGRAM-ERROR in thread
#<THREAD tid=47856 "main thread" RUNNING {12003D0143}>:
  Execution of a form compiled with errors.
Form:
  ((DEXADOR:GET (CONCATENATE WS_HOST "/sanctum/csrf-cookie") :COOKIE-JAR
              COOKIEJAR))
Compile-time error:
  illegal function call

Type HELP for debugger help, or (SB-EXT:EXIT) to exit from SBCL.

restarts (invokable by number or by possibly-abbreviated name):
  0: [RETRY   ] Retry EVAL of current toplevel form.
  1: [CONTINUE] Ignore error and continue loading file "/home/pedro/wikistocks-api-templates/ws-common-lisp-template/src/ws-bot.lisp".
  2: [ABORT   ] Abort loading file "/home/pedro/wikistocks-api-templates/ws-common-lisp-template/src/ws-bot.lisp".
  3:            Ignore runtime option --load "src/ws-bot.lisp".
  4:            Skip rest of --eval and --load options.
  5:            Skip to toplevel READ/EVAL/PRINT loop.
  6: [EXIT    ] Exit SBCL (calling #'EXIT, killing the process).

(REQUESTCSRFTOKEN)
; File has been modified since compilation:
;   /home/pedro/wikistocks-api-templates/ws-common-lisp-template/src/ws-bot.lisp
; Using form offset instead of character position.

   source: ((DEXADOR:GET (CONCATENATE WS_HOST "/sanctum/csrf-cookie")
                         :COOKIE-JAR COOKIEJAR))
0]
````

So at this point, I had been programming without a Language Server for about 3
hours, trying to figure out how it all works. Going through the "getting
started" tutorial on the cookbook and trying a bunch of things. The REPL is
really good to learn Lisp, and I think the language is fine. The tooling
however... is not as "fine". I really wanted an LSP so that I can understand
where the problem is comming from as the REPL is giving me Minecraft Enchanting
Table type error messages.

The only Language Server I could find that was program agnostic (could work
with any IDE that had the standard language server implementation) was made
with lisp for lisp. Naturally, I decided to download it, the name is `cl-lsp`.
The instructions said to load it using `roswell`, a different package manager
for Common Lisp that isn't included in SBCL.

Reminder, as I defined earlier, these templates are supposed to be loadable
and working after entering the nix shell environment. This requirement ins't
just because I love nix, it's because after I'm done, I can easily get rid of
SBCL and other tooling with just `nix-garbage-collect` and get rid of that
space on my small laptop.

So with that requirement in mind, I had a look at the nix package repository
and didn't find `cl-lsp` but found `roswell`. After adding it to the
environment, I tried to install `cl-lsp` onto my machine so I ran:

````
❯ ros install lem-project/lem cxxxr/cl-lsp
Making core for Roswell...
Installing Quicklisp... Done 33792
building dump:/home/pedro-pires/.roswell/impls/x86-64/linux/sbcl-bin/system/dump/jfq8k9fq2mbvwrnd439fkn1d83073hsg-roswell-24.10.115.core
Installing from github lem-project/lem
While evaluating the form starting at line 307, column 0
  of #P"/home/pedro-pires/.roswell/local-projects/lem-project/lem/lem.asd":
;
; compilation unit aborted
;   caught 1 fatal ERROR condition
Installing from github cxxxr/cl-lsp
To load "cl-lsp":
/*
i truncated the output (about 300 lines)
*/
; Loading "async-process"
.[1/3] System 'cl-lsp' found. Loading the system..Aborted during step [1/3].
Unhandled CFFI:LOAD-FOREIGN-LIBRARY-ERROR in thread #<SB-THREAD:THREAD tid=68911 "main thread" RUNNING
                                                       {12003D0143}>:
  Unable to load foreign library (ASYNC-PROCESS).
  Error opening shared object "libasyncprocess.so":
  libasyncprocess.so: cannot open shared object file: No such file or directory.
/*
some more truncation
*/
61: ((FLET "WITHOUT-INTERRUPTS-BODY-3" :IN SB-IMPL::START-LISP))
62: (SB-IMPL::%START-LISP)

unhandled condition in --disable-debugger mode, quitting
;
; compilation unit aborted
;   caught 1 fatal ERROR condition
````

That just installed quicklisp on my system against my will... and also didn't
install `cl-lsp`. I assume it's because I had `roswell` in nix, in which the
file system is immutable and can't run arbitrary binaries. Maybe it's just
missing that `libasyncprocess` library.

![A screenshot of the search of packages.nixos.org with the search query
"libasyncprocess". The results list is empty.
](/public/images/2026/common-lisp-frustrations/nix-packages-search-libasyncprocess.webp)

So no library, I decided to look around online some more...

---

I went to sleep, when I woke up, magically, youtube recommended me a video:
["Mine: a new IDE for Common Lisp"](https://www.youtube.com/watch?v=qe3vDKQShKs).
Somehow, even without me using any google products to search for common lisp
stuff (using Startpage to search) youtube somehow knew I was struggling with
lisp.

The gentleman in the video recommends
[Mine](https://coalton-lang.github.io/mine/) as a IDE for people who are just
getting started with lisp as it downloads dependencies automatically and sets
up an environment for you automatically. "Sick, lemme try it out". All I
downloaded was an AppImage file; And it looks good, it's a full IDE
that includes the REPL, a way to create projects, a file tree, syntax
highlighting, autocomplete and everything.

![A screenshot of Mine, it shows a tab menu at the top, file tree on the left
with open files at the top, the center has the currently open file and at the
bottom we can see the REPL with the result of the program. It's a simple hello
world in common lisp but instead of "world" it says whatever name you pass it.
In this case, it shows "Hello Pedro".
](/public/images/2026/common-lisp-frustrations/mine-showoff.webp)

This gave me a nice taste of what it could be like programming in Lisp,
apparently it was made for a different flavour of Lisp called "Coalton" but it
also works and supports Common Lisp. It's supposed to come with `quicklisp`,
another package manager that dynamically downloads packages as they are used in
your program, so I wanted to try it out by importing `dexator`.

![A screenshot of Mine, the same as before but I added an import for dexator
through the quicklisp. On the REPL section, an error can be seen showing that
QL package does not exist.
](/public/images/2026/common-lisp-frustrations/mine-ql-error.webp)

`Package QL does not exist.`

`Package QL does not exist.`?

`Package QL does not exist.`?????

So you're telling me, that after I finally conceeded to downloading something
to my computer so I could learn how to program in lisp, so I would have
everything auto set up automatically, doesn't include one of the parts it was
supposed to include?

For reference, I spent in total, probably around 6-7 hours trying to make this
template, in 2 different days; Spending most of this time trying to fiddle with
the lisp tooling that I could. Compared to python, which I've never written in
my life, that took about 3 hours to learn, use pip and write all the code and
Go which I had never touched a piece of Go code before in my life, took around
2 hours to learn it, use go tidy, and write all the code for the template. I
learnt python and Go in the same amount of time I had spent setting up tooling
for Lisp. Even assembly had better tooling. ASSEMBLY!

In my rage, I decided to look up "Popular Programs Made in Lisp", I found the
[awesome-cl](https://awesome-cl.com/) site which has a lot of tools and
libraries made in lisp for lisp users and I found
[Category: Common Lisp Software](https://en.wikipedia.org/wiki/Category:Common_Lisp_(programming_language)_software)
on Wikipedia which is a list of software made in Common Lisp. Most of the
software I found made in Common Lisp fits into 2 categories:

- Tools for lisp
- Not tools for lisp

The only notable software I could find on that list was `GOAL`, a lisp dialect
created by the Naughty Dog team to make the legendary _Jak and Daxter_ series
but has been dropped by Sony since the maintainer left the company.

From that I realised a few things:

- Common Lisp is useless in modern times
- [I need emacs to use Lisp
](https://web.archive.org/web/20070720142546/http://lists.midnightryder.com/pipermail/sweng-gamedev-midnightryder.com/2005-August/003789.html)
- I'm gonna play Jak and Daxter again on my ps2

After that research I decided to never use Common Lisp again.

Might visit OpenGOAL in the future.

Thanks for reading.
