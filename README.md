# anagram

A simple anagram generator

# Usage

`anagram` uses utf8, and works with non-latin scripts, but is character agnostic; it is
up to the user to handle whitespace removal, normalization of uppercase, accents, etc.


Start by generating an adequate dictionary. For instance:

```
$ aspell dump master en |tr a-z A-Z |grep -x "[A-Z]*" >dict.txt
```

Look for anagrams of a specific word:

```
$ anagram -l5 DELTAOMICRON <dict.txt >anagrams.txt
158 anagrams found.

$ cat anagrams.txt
CAEDMON TIROL
CAMELOT RODIN
CAROLED MINOT
CAROLED TIMON
CAROTID LEMON
CAROTID MELON
CARTOON LIMED
CITADEL MORON
CITROEN MODAL
CLIMATE DONOR
CLIMATE RONDO
COMORAN TILDE
[...]
NIMROD LOCATE
OILCAN DERMOT
RATION MCLEOD
RECTAL DOMINO
REMOLD ACTION
REMOLD CATION
RETAIL CONDOM
RETOLD MANIOC
RETOLD MONICA
TOILED MACRON
```
